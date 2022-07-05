use crate::errors::{FormatError, SicIoError};
use crate::export::ExportSettings;
use crate::WriteSeek;
use image::buffer::ConvertBuffer;
use image::DynamicImage;
use sic_core::image::codecs::gif::Repeat;
use sic_core::image::codecs::pnm;
use sic_core::{image, AnimatedImage, SicImage};
use std::io::Write;

#[derive(Clone, Copy, Debug)]
pub enum AutomaticColorTypeAdjustment {
    // Usually the default
    Enabled,
    Disabled,
}

impl Default for AutomaticColorTypeAdjustment {
    fn default() -> Self {
        AutomaticColorTypeAdjustment::Enabled
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RepeatAnimation {
    Finite(u16),
    Infinite,
    Never,
}

impl RepeatAnimation {
    pub fn try_from_str(input: &str) -> Result<Self, SicIoError> {
        match input {
            "infinite" => Ok(Self::Infinite),
            "never" => Ok(Self::Never),
            elsy => elsy
                .parse::<u16>()
                .map(Self::Finite)
                .map_err(|_| SicIoError::FormatError(FormatError::GIFRepeatInvalidValue)),
        }
    }
}

impl Default for RepeatAnimation {
    fn default() -> Self {
        Self::Infinite
    }
}

/// Use the ConversionWriter to convert and write image buffers to an output.
pub struct ConversionWriter<'a> {
    image: &'a SicImage,
}

impl<'a> ConversionWriter<'a> {
    pub fn new(image: &SicImage) -> ConversionWriter {
        ConversionWriter { image }
    }

    pub fn write_all<W: WriteSeek>(
        &self,
        writer: &mut W,
        output_format: image::ImageOutputFormat,
        export_settings: &ExportSettings,
    ) -> Result<(), SicIoError> {
        let color_processing = &ConversionWriter::pre_process_color_type(
            self.image,
            &output_format,
            export_settings.adjust_color_type,
        );

        let export_buffer = match color_processing {
            Some(replacement) => replacement,
            None => self.image,
        };

        ConversionWriter::export(writer, export_buffer, output_format, export_settings)
    }

    /// Some image output format types require color type pre-processing.
    /// This is the case if the output image format does not support the color type held by the image buffer prior to the final conversion.
    ///
    /// If pre-processing of the color type took place, Some(<new image>) will be returned.
    /// If no pre-processing of the color type is required will return None.
    /// Frames of animated images are not adjusted.
    fn pre_process_color_type(
        image: &SicImage,
        output_format: &image::ImageOutputFormat,
        color_type_adjustment: AutomaticColorTypeAdjustment,
    ) -> Option<SicImage> {
        if let AutomaticColorTypeAdjustment::Disabled = color_type_adjustment {
            return None;
        }

        match image {
            SicImage::Animated(_) => None,
            SicImage::Static(image) => {
                adjust_dynamic_image(image, output_format).map(SicImage::from)
            }
        }
    }

    fn export<W: WriteSeek>(
        writer: &mut W,
        image: &SicImage,
        format: image::ImageOutputFormat,
        export_settings: &ExportSettings,
    ) -> Result<(), SicIoError> {
        match image {
            SicImage::Animated(image) => {
                encode_animated_image(writer, image.collect_frames(), format, export_settings)
            }
            SicImage::Static(image) => encode_static_image(writer, image, format),
        }
    }
}

/// Adjusts the type of image buffer, unless it's determined to be unnecessary
fn adjust_dynamic_image(
    image: &image::DynamicImage,
    output_format: &image::ImageOutputFormat,
) -> Option<DynamicImage> {
    // A remaining open question: does a user expect for an image to be able to convert to a format even if the color type is not supported?
    // And even if the user does, should we?
    // I suspect that users expect that color type conversions should happen automatically.
    //
    // Testing also showed that even bmp with full black full white pixels do not convert correctly as of now. Why exactly is unclear;
    // Perhaps the color type of the bmp formatted test image?

    match output_format {
        image::ImageOutputFormat::Farbfeld => {
            Some(DynamicImage::ImageRgba16(image.to_rgba8().convert()))
        }
        image::ImageOutputFormat::Pnm(pnm::PnmSubtype::Bitmap(_)) => {
            Some(DynamicImage::ImageLuma8(image.to_luma8()))
        }
        image::ImageOutputFormat::Pnm(pnm::PnmSubtype::Graymap(_)) => {
            Some(DynamicImage::ImageLuma8(image.to_luma8()))
        }
        image::ImageOutputFormat::Pnm(pnm::PnmSubtype::Pixmap(_)) => {
            Some(image::DynamicImage::ImageRgb8(image.to_rgb8()))
        }
        _ => None,
    }
}

fn encode_static_image<W: WriteSeek>(
    writer: &mut W,
    image: &image::DynamicImage,
    format: image::ImageOutputFormat,
) -> Result<(), SicIoError> {
    image
        .write_to(writer, format)
        .map_err(SicIoError::ImageError)
}

fn encode_animated_image<W: WriteSeek>(
    writer: &mut W,
    frames: Vec<image::Frame>, // note: should be owned for the encoder, so can't be a slice
    format: image::ImageOutputFormat,
    export_settings: &ExportSettings,
) -> Result<(), SicIoError> {
    match format {
        image::ImageOutputFormat::Gif => {
            encode_animated_gif(writer, frames, export_settings.gif_repeat)
        }
        _ => {
            eprintln!("WARN: The animated image buffer could not be encoded to the {:?} format; encoding only the first frame", format);
            let image = AnimatedImage::from_frames(frames).try_into_static_image(0)?;
            encode_static_image(writer, &image, format)
        }
    }
}

fn encode_animated_gif<W: Write>(
    writer: &mut W,
    frames: Vec<image::Frame>,
    repeat: RepeatAnimation,
) -> Result<(), SicIoError> {
    let mut encoder = image::codecs::gif::GifEncoder::new(writer);
    encoder.encode_frames(frames)?;

    match repeat {
        RepeatAnimation::Finite(amount) => encoder.set_repeat(Repeat::Finite(amount))?,
        RepeatAnimation::Infinite => encoder.set_repeat(Repeat::Infinite)?,
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, Read};

    use parameterized::parameterized;
    use sic_core::image::{ImageFormat, ImageOutputFormat};
    use sic_testing::{clean_up_output_path, setup_output_path, setup_test_image};

    use super::*;

    impl WriteSeek for File {}

    // Individual tests:

    const INPUT: &str = "rainbow_8x6.bmp";
    const OUTPUT: &str = "_out.png";

    #[test]
    fn will_output_file_be_created() -> io::Result<()> {
        let our_output = &format!("will_output_file_be_created{}", OUTPUT); // this is required because tests are run in parallel, and the creation, or deletion can collide.
        let output_path = setup_output_path(our_output);

        let buffer = image::open(setup_test_image(INPUT))
            .expect("Can't open test file.")
            .into();
        let example_output_format = image::ImageOutputFormat::Png;
        let conversion_processor = ConversionWriter::new(&buffer);
        conversion_processor
            .write_all(
                &mut File::create(&output_path)?,
                example_output_format,
                &ExportSettings {
                    adjust_color_type: AutomaticColorTypeAdjustment::Enabled,
                    ..Default::default()
                },
            )
            .expect("Unable to save file to the test computer.");

        assert!(output_path.exists());

        clean_up_output_path(our_output);
        Ok(())
    }

    #[test]
    fn has_png_extension() -> io::Result<()> {
        let our_output = &format!("has_png_extension{}", OUTPUT); // this is required because tests are run in parallel, and the creation, or deletion can collide.
        let output_path = setup_output_path(our_output);

        let buffer = image::open(setup_test_image(INPUT))
            .expect("Can't open test file.")
            .into();
        let example_output_format = image::ImageOutputFormat::Png;
        let conversion_processor = ConversionWriter::new(&buffer);
        conversion_processor
            .write_all(
                &mut File::create(&output_path)?,
                example_output_format,
                &ExportSettings {
                    adjust_color_type: AutomaticColorTypeAdjustment::Enabled,
                    ..Default::default()
                },
            )
            .expect("Unable to save file to the test computer.");

        assert_eq!(
            Some(std::ffi::OsStr::new("png")),
            setup_output_path(our_output).extension()
        );

        clean_up_output_path(our_output);
        Ok(())
    }

    #[test]
    fn is_png_file() -> io::Result<()> {
        let our_output = &format!("is_png_file{}", OUTPUT); // this is required because tests are run in parallel, and the creation, or deletion can collide.
        let output_path = setup_output_path(our_output);

        let buffer = image::open(setup_test_image(INPUT))
            .expect("Can't open test file.")
            .into();
        let example_output_format = image::ImageOutputFormat::Png;
        let conversion_processor = ConversionWriter::new(&buffer);
        conversion_processor
            .write_all(
                &mut File::create(&output_path)?,
                example_output_format,
                &ExportSettings {
                    adjust_color_type: AutomaticColorTypeAdjustment::Enabled,
                    ..Default::default()
                },
            )
            .expect("Unable to save file to the test computer.");

        let mut file = std::fs::File::open(setup_output_path(our_output))
            .expect("Unable to find file we made.");
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)
            .expect("Unable to finish reading our test image.");

        assert_eq!(
            image::ImageFormat::Png,
            image::guess_format(&bytes).expect("Format could not be guessed.")
        );

        clean_up_output_path(our_output);
        Ok(())
    }

    // Multi tests:
    // Below all supported formats are testsed using the inputs listed below.

    const INPUT_MULTI: &[&str] = &["blackwhite_2x2.bmp", "palette_4x4.png"];

    fn test_conversion_with_header_match(
        input: &str,
        enc_format: &str,
        format: image::ImageOutputFormat,
        expected_format: image::ImageFormat,
    ) -> io::Result<()> {
        let our_output = &format!("header_match_conversion.{}", enc_format); // this is required because tests are run in parallel, and the creation, or deletion can collide.
        let output_path = setup_output_path(our_output);

        let buffer = image::open(setup_test_image(input))
            .expect("Can't open test file.")
            .into();
        let conversion_processor = ConversionWriter::new(&buffer);
        let mut writer = File::create(&output_path)?;

        conversion_processor
            .write_all(
                &mut writer,
                format,
                &ExportSettings {
                    adjust_color_type: AutomaticColorTypeAdjustment::Enabled,
                    ..Default::default()
                },
            )
            .expect("Unable to save file to the test computer.");

        let mut file = std::fs::File::open(setup_output_path(our_output))
            .expect("Unable to find file we made.");
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)
            .expect("Unable to finish reading our test image.");

        assert_eq!(
            expected_format,
            image::guess_format(&bytes).expect("Format could not be guessed.")
        );

        clean_up_output_path(our_output);
        Ok(())
    }

    #[parameterized(
        ext = {
            "bmp", 
            "farbfeld", 
            "gif", 
            "ico", 
            "jpg", 
            "jpeg", 
            "png", 
            "pbm", 
            "pgm", 
            "ppm", 
            "pam",
        },
        to_format = {
            image::ImageOutputFormat::Bmp,
            image::ImageOutputFormat::Farbfeld,
            image::ImageOutputFormat::Gif,
            image::ImageOutputFormat::Ico,
            image::ImageOutputFormat::Jpeg(80),
            image::ImageOutputFormat::Jpeg(80),
            image::ImageOutputFormat::Png,
            image::ImageOutputFormat::Pnm(image::codecs::pnm::PnmSubtype::Bitmap(
                image::codecs::pnm::SampleEncoding::Binary,
            )),
            image::ImageOutputFormat::Pnm(image::codecs::pnm::PnmSubtype::Graymap(
                image::codecs::pnm::SampleEncoding::Binary,
            )),
            image::ImageOutputFormat::Pnm(image::codecs::pnm::PnmSubtype::Pixmap(
                image::codecs::pnm::SampleEncoding::Binary,
            )),
            image::ImageOutputFormat::Pnm(
                image::codecs::pnm::PnmSubtype::ArbitraryMap
            ),
        },
        expected_format = {
            image::ImageFormat::Bmp,
            image::ImageFormat::Farbfeld,
            image::ImageFormat::Gif,
            image::ImageFormat::Ico,
            image::ImageFormat::Jpeg,
            image::ImageFormat::Jpeg,
            image::ImageFormat::Png,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
        }
    )]
    fn test_conversions_with_header_match(
        ext: &str,
        to_format: ImageOutputFormat,
        expected_format: ImageFormat,
    ) {
        for test_image in INPUT_MULTI.iter() {
            test_conversion_with_header_match(test_image, ext, to_format.clone(), expected_format)
                .unwrap();
        }
    }
}
