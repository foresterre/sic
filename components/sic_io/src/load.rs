use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;

use crate::errors::SicIoError;
use sic_core::image::{AnimationDecoder, ImageFormat};
use sic_core::{image, SicImage};

/// Load an image using a reader.
/// All images are currently loaded from memory.
pub fn load_image<R: Read>(reader: &mut R, config: &ImportConfig) -> ImportResult<SicImage> {
    let reader = image::io::Reader::new(Cursor::new(load(reader)?))
        .with_guessed_format()
        .map_err(SicIoError::Io)?;

    match reader.format() {
        Some(ImageFormat::Png) => decode_png(reader, config.selected_frame),
        Some(ImageFormat::Gif) => decode_gif(reader, config.selected_frame),
        Some(_) => reader
            .decode()
            .map_err(SicIoError::ImageError)
            .map(SicImage::from),
        None => Err(SicIoError::ImageError(image::error::ImageError::Decoding(
            image::error::DecodingError::from_format_hint(image::error::ImageFormatHint::Unknown),
        ))),
    }
}

/// Result which is returned for operations within this module.
type ImportResult<T> = Result<T, SicIoError>;

/// Constructs a reader which reads from the stdin.
pub fn stdin_reader() -> ImportResult<Box<dyn Read>> {
    Ok(Box::new(BufReader::new(std::io::stdin())))
}

/// Constructs a reader which reads from a file path.
pub fn file_reader<P: AsRef<Path>>(path: P) -> ImportResult<Box<dyn Read>> {
    Ok(Box::new(BufReader::new(
        File::open(path).map_err(SicIoError::Io)?,
    )))
}

// Let the reader store the raw bytes into a buffer.
fn load<R: Read>(reader: &mut R) -> ImportResult<Vec<u8>> {
    let mut buffer = Vec::new();
    let _size = reader.read_to_end(&mut buffer).map_err(SicIoError::Io)?;
    Ok(buffer)
}

#[derive(Debug, Default)]
pub struct ImportConfig {
    /// For animated images; decides which frame will be used as static image.
    pub selected_frame: FrameIndex,
}

/// Decode an image into frames
fn frames<'decoder, D: AnimationDecoder<'decoder>>(decoder: D) -> ImportResult<Vec<image::Frame>> {
    decoder
        .into_frames()
        .collect_frames()
        .map_err(SicIoError::ImageError)
}

/// Select an image frame from a slice of frames and a frame index
fn select_frame(frames: &[image::Frame], index: FrameIndex) -> ImportResult<SicImage> {
    let frame_index = match index {
        FrameIndex::First => 0,
        FrameIndex::Last => frames.len() - 1,
        FrameIndex::Nth(n) => n,
    };

    frames
        .get(frame_index)
        .ok_or_else(|| SicIoError::NoSuchFrame(frame_index, frames.len() - 1))
        .map(|f| image::DynamicImage::ImageRgba8(f.clone().into_buffer()))
        .map(SicImage::from)
}

/// Zero-indexed frame index.
#[derive(Clone, Copy, Debug)]
pub enum FrameIndex {
    First,
    Last,
    Nth(usize),
}

impl Default for FrameIndex {
    fn default() -> Self {
        FrameIndex::First
    }
}

fn decode_gif<R: Read>(reader: image::io::Reader<R>, frame: FrameIndex) -> ImportResult<SicImage> {
    let decoder =
        image::gif::GifDecoder::new(reader.into_inner()).map_err(SicIoError::ImageError)?;

    frames(decoder)
        .and_then(|f| select_frame(&f, frame))
        .map(SicImage::from)
}

fn decode_png<R: Read>(reader: image::io::Reader<R>, frame: FrameIndex) -> ImportResult<SicImage> {
    let decoder =
        image::png::PngDecoder::new(reader.into_inner()).map_err(SicIoError::ImageError)?;

    if decoder.is_apng() {
        frames(decoder.apng()).and_then(|f| select_frame(&f, frame))
    } else {
        image::DynamicImage::from_decoder(decoder)
            .map_err(SicIoError::ImageError)
            .map(SicImage::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;
    use sic_testing::*;

    const GIF_LOOP: &str = "loop.gif";
    const GIF_NO_LOOP: &str = "noloop.gif";
    const XY: u32 = 10;

    #[test]
    fn load_gif_non_looping_frame_first() {
        let load_path = setup_test_image(GIF_NO_LOOP);

        let config = ImportConfig {
            selected_frame: FrameIndex::First,
        };

        let image = load_image(&mut file_reader(load_path).unwrap(), &config).unwrap();

        // color = red
        let expected: [u8; 4] = [254, 0, 0, 255];
        assert_eq!(image.get_pixel(XY, XY).0, expected);
    }

    #[test]
    fn load_gif_non_looping_frame_first_is_zero() {
        let load_path = setup_test_image(GIF_NO_LOOP);

        let first = ImportConfig {
            selected_frame: FrameIndex::First,
        };

        let zero = ImportConfig {
            selected_frame: FrameIndex::Nth(0),
        };

        let first = load_image(&mut file_reader(&load_path).unwrap(), &first).unwrap();
        let zero = load_image(&mut file_reader(&load_path).unwrap(), &zero).unwrap();

        assert_eq!(first.get_pixel(XY, XY).0, zero.get_pixel(XY, XY).0);
    }

    #[test]
    fn load_gif_looping_frame_first_is_zero() {
        let load_path = setup_test_image(GIF_LOOP);

        let first = ImportConfig {
            selected_frame: FrameIndex::First,
        };

        let zero = ImportConfig {
            selected_frame: FrameIndex::Nth(0),
        };

        let first = load_image(&mut file_reader(&load_path).unwrap(), &first).unwrap();
        let zero = load_image(&mut file_reader(&load_path).unwrap(), &zero).unwrap();

        assert_eq!(first.get_pixel(XY, XY).0, zero.get_pixel(XY, XY).0);
    }

    // [[expected color]; amount]
    // verified with pastel cli;
    const FRAME_COLORS: [[u8; 4]; 8] = [
        [254, 0, 0, 255],     // red
        [254, 165, 0, 255],   // orange
        [255, 255, 0, 255],   // yellow
        [0, 128, 1, 255],     // green
        [0, 0, 254, 255],     // blue
        [75, 0, 129, 255],    // indigo
        [238, 130, 239, 255], // violet
        [0, 0, 0, 255],       // black
    ];

    #[test]
    fn load_gif_non_looping_frame_nth() {
        for (i, expected) in FRAME_COLORS.iter().enumerate() {
            let load_path = setup_test_image(GIF_NO_LOOP);

            let config = ImportConfig {
                selected_frame: FrameIndex::Nth(i),
            };

            let image = load_image(&mut file_reader(load_path).unwrap(), &config).unwrap();

            assert_eq!(&image.get_pixel(XY, XY).0, expected);
        }
    }

    #[test]
    fn load_gif_looping_frame_nth() {
        for (i, expected) in FRAME_COLORS.iter().enumerate() {
            let load_path = setup_test_image(GIF_LOOP);

            let config = ImportConfig {
                selected_frame: FrameIndex::Nth(i),
            };

            let image = load_image(&mut file_reader(load_path).unwrap(), &config).unwrap();

            assert_eq!(&image.get_pixel(XY, XY).0, expected);
        }
    }

    #[test]
    fn load_gif_non_looping_frame_nth_beyond_length() {
        let load_path = setup_test_image(GIF_NO_LOOP);

        let config = ImportConfig {
            selected_frame: FrameIndex::Nth(8),
        };

        let result = load_image(&mut file_reader(load_path).unwrap(), &config);
        assert!(result.is_err());
    }

    // Even if the gif loops, it still has 8 frames.
    #[test]
    fn load_gif_looping_frame_nth_beyond_length() {
        let load_path = setup_test_image(GIF_LOOP);

        let config = ImportConfig {
            selected_frame: FrameIndex::Nth(8),
        };

        let result = load_image(&mut file_reader(load_path).unwrap(), &config);
        assert!(result.is_err());
    }

    #[test]
    fn load_gif_non_looping_frame_last_is_seven_index() {
        let load_path = setup_test_image(GIF_NO_LOOP);

        let last = ImportConfig {
            selected_frame: FrameIndex::Last,
        };

        let seven = ImportConfig {
            selected_frame: FrameIndex::Nth(7),
        };

        let last = load_image(&mut file_reader(&load_path).unwrap(), &last).unwrap();
        let seven = load_image(&mut file_reader(&load_path).unwrap(), &seven).unwrap();

        assert_eq!(last.get_pixel(XY, XY).0, seven.get_pixel(XY, XY).0);
    }

    #[test]
    fn load_gif_looping_frame_last_is_seven_index() {
        let load_path = setup_test_image(GIF_LOOP);

        let last = ImportConfig {
            selected_frame: FrameIndex::Last,
        };

        let seven = ImportConfig {
            selected_frame: FrameIndex::Nth(7),
        };

        let last = load_image(&mut file_reader(&load_path).unwrap(), &last).unwrap();
        let seven = load_image(&mut file_reader(&load_path).unwrap(), &seven).unwrap();

        assert_eq!(last.get_pixel(XY, XY).0, seven.get_pixel(XY, XY).0);
    }

    const NOT_GIFS: [&str; 3] = [
        "blackwhite_2x2.bmp",
        "bwlines.png",
        "unsplash_763569_cropped.jpg",
    ];

    #[test]
    fn load_not_gif_formatted() {
        for path in NOT_GIFS.iter() {
            let load_path = setup_test_image(path);
            let config = ImportConfig::default();
            let result = load_image(&mut file_reader(load_path).unwrap(), &config);
            assert!(result.is_ok());
        }
    }

    mod apng {
        use super::*;

        const APNG_SAMPLE: &str = "apng_sample.png";

        #[parameterized(
            frame = {
                FrameIndex::First,
                FrameIndex::Nth(0),
                FrameIndex::Nth(1),
                FrameIndex::Nth(2),
                FrameIndex::Last,
                FrameIndex::Nth(3),
            },
            expected_color = {
                Some([255, 255, 255, 255]),
                Some([255, 255, 255, 255]),
                Some([237, 28, 36, 255]), // default red in paint.exe
                Some([0, 0, 0, 255]),
                Some([0, 0, 0, 255]),
                None,
            }
        )]
        fn apng(frame: FrameIndex, expected_color: Option<[u8; 4]>) {
            let load_path = setup_test_image(APNG_SAMPLE);

            let config = ImportConfig {
                selected_frame: frame,
            };

            let image = load_image(&mut file_reader(load_path).unwrap(), &config);

            if expected_color.is_some() {
                assert_eq!(image.unwrap().get_pixel(0, 0).0, expected_color.unwrap());
            } else {
                assert!(image.is_err())
            }
        }
    }
}
