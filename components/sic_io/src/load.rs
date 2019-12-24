use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;

use crate::errors::SicIoError;
use sic_core::image;
use sic_core::image::{AnimationDecoder, ImageFormat};

/// Load an image using a reader.
/// All images are currently loaded from memory.
pub fn load_image<R: Read>(
    reader: &mut R,
    config: &ImportConfig,
) -> ImportResult<image::DynamicImage> {
    let reader = image::io::Reader::new(Cursor::new(load(reader)?))
        .with_guessed_format()
        .map_err(SicIoError::Io)?;

    match reader.format() {
        Some(f) => match f {
            ImageFormat::GIF => {
                let frames = image::gif::Decoder::new(reader.into_inner())
                    .and_then(|decoder| decoder.into_frames().collect_frames())
                    .map_err(SicIoError::ImageError)?;

                let frame_index = match config.selected_frame {
                    FrameIndex::First => 0,
                    FrameIndex::Last => frames.len() - 1,
                    FrameIndex::Nth(n) => n,
                };
                match frames.into_iter().nth(frame_index) {
                    Some(frame) => Ok(image::DynamicImage::ImageRgba8(frame.into_buffer())),
                    None => Err(SicIoError::NoSuchFrame(
                        frame_index + 1,
                        "Frame index out of range.".to_string(),
                    )),
                }
            }
            _ => Ok(reader.decode().map_err(SicIoError::ImageError)?),
        },
        None => Err(SicIoError::ImageError(image::ImageError::FormatError(
            "Image format not supported.".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;
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
}
