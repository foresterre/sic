use crate::errors::SicIoError;
use image::buffer::ConvertBuffer;
use image::DynamicImage;
use sic_core::image;
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

/// Use the ConversionWriter to convert and write image buffers to an output.
pub struct ConversionWriter<'a> {
    image: &'a image::DynamicImage,
}

impl<'a> ConversionWriter<'a> {
    pub fn new(image: &image::DynamicImage) -> ConversionWriter {
        ConversionWriter { image }
    }

    pub fn write<W: Write>(
        &self,
        writer: &mut W,
        output_format: image::ImageOutputFormat,
        color_type_adjustment: AutomaticColorTypeAdjustment,
    ) -> Result<(), SicIoError> {
        let color_processing = &ConversionWriter::pre_process_color_type(
            &self.image,
            &output_format,
            color_type_adjustment,
        );

        let export_buffer = match color_processing {
            Some(replacement) => replacement,
            None => &self.image,
        };

        ConversionWriter::save_to(writer, &export_buffer, output_format)
    }

    /// Some image output format types require color type pre-processing.
    /// This is the case if the output image format does not support the color type held by the image buffer prior to the final conversion.
    ///
    /// If pre-processing of the color type took place, Some(<new image>) will be returned.
    /// If no pre-processing of the color type is required will return None.
    fn pre_process_color_type(
        image: &image::DynamicImage,
        output_format: &image::ImageOutputFormat,
        color_type_adjustment: AutomaticColorTypeAdjustment,
    ) -> Option<image::DynamicImage> {
        // A remaining open question: does a user expect for an image to be able to convert to a format even if the color type is not supported?
        // And even if the user does, should we?
        // I suspect that users expect that color type conversions should happen automatically.
        //
        // Testing also showed that even bmp with full black full white pixels do not convert correctly as of now. Why exactly is unclear;
        // Perhaps the color type of the bmp formatted test image?

        match color_type_adjustment {
            AutomaticColorTypeAdjustment::Enabled => match output_format {
                image::ImageOutputFormat::Farbfeld => {
                    Some(DynamicImage::ImageRgba16(image.to_rgba8().convert()))
                }
                image::ImageOutputFormat::Pnm(image::pnm::PNMSubtype::Bitmap(_)) => {
                    Some(image.grayscale())
                }
                image::ImageOutputFormat::Pnm(image::pnm::PNMSubtype::Graymap(_)) => {
                    Some(image.grayscale())
                }
                image::ImageOutputFormat::Pnm(image::pnm::PNMSubtype::Pixmap(_)) => {
                    Some(image::DynamicImage::ImageRgb8(image.to_rgb8()))
                }
                _ => None,
            },
            AutomaticColorTypeAdjustment::Disabled => None,
        }
    }

    fn save_to<W: Write>(
        writer: &mut W,
        buffer: &image::DynamicImage,
        format: image::ImageOutputFormat,
    ) -> Result<(), SicIoError> {
        buffer
            .write_to(writer, format)
            .map_err(SicIoError::ImageError)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, Read};

    use sic_testing::{clean_up_output_path, setup_output_path, setup_test_image};

    use super::*;

    // Individual tests:

    const INPUT: &str = "rainbow_8x6.bmp";
    const OUTPUT: &str = "_out.png";

    #[test]
    fn will_output_file_be_created() -> io::Result<()> {
        let our_output = &format!("will_output_file_be_created{}", OUTPUT); // this is required because tests are run in parallel, and the creation, or deletion can collide.
        let output_path = setup_output_path(our_output);

        let buffer = image::open(setup_test_image(INPUT)).expect("Can't open test file.");
        let example_output_format = image::ImageOutputFormat::Png;
        let conversion_processor = ConversionWriter::new(&buffer);
        conversion_processor
            .write(
                &mut File::create(&output_path)?,
                example_output_format,
                AutomaticColorTypeAdjustment::Enabled,
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

        let buffer = image::open(setup_test_image(INPUT)).expect("Can't open test file.");
        let example_output_format = image::ImageOutputFormat::Png;
        let conversion_processor = ConversionWriter::new(&buffer);
        conversion_processor
            .write(
                &mut File::create(&output_path)?,
                example_output_format,
                AutomaticColorTypeAdjustment::Enabled,
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

        let buffer = image::open(setup_test_image(INPUT)).expect("Can't open test file.");
        let example_output_format = image::ImageOutputFormat::Png;
        let conversion_processor = ConversionWriter::new(&buffer);
        conversion_processor
            .write(
                &mut File::create(&output_path)?,
                example_output_format,
                AutomaticColorTypeAdjustment::Enabled,
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
    const INPUT_FORMATS: &[&str] = &[
        "bmp", "farbfeld", "gif", "ico", "jpg", "jpeg", "png", "pbm", "pgm", "ppm", "pam",
    ];
    const OUTPUT_FORMATS: &[image::ImageOutputFormat] = &[
        image::ImageOutputFormat::Bmp,
        image::ImageOutputFormat::Farbfeld,
        image::ImageOutputFormat::Gif,
        image::ImageOutputFormat::Ico,
        image::ImageOutputFormat::Jpeg(80),
        image::ImageOutputFormat::Jpeg(80),
        image::ImageOutputFormat::Png,
        image::ImageOutputFormat::Pnm(image::pnm::PNMSubtype::Bitmap(
            image::pnm::SampleEncoding::Binary,
        )),
        image::ImageOutputFormat::Pnm(image::pnm::PNMSubtype::Graymap(
            image::pnm::SampleEncoding::Binary,
        )),
        image::ImageOutputFormat::Pnm(image::pnm::PNMSubtype::Pixmap(
            image::pnm::SampleEncoding::Binary,
        )),
        image::ImageOutputFormat::Pnm(image::pnm::PNMSubtype::ArbitraryMap),
    ];

    const EXPECTED_VALUES: &[image::ImageFormat] = &[
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
    ];

    fn test_conversion_with_header_match(
        input: &str,
        enc_format: &str,
        format: image::ImageOutputFormat,
        expected_format: image::ImageFormat,
    ) -> io::Result<()> {
        let our_output = &format!("header_match_conversion.{}", enc_format); // this is required because tests are run in parallel, and the creation, or deletion can collide.
        let output_path = setup_output_path(our_output);

        let buffer = image::open(setup_test_image(input)).expect("Can't open test file.");
        let conversion_processor = ConversionWriter::new(&buffer);
        let mut writer = File::create(&output_path)?;

        conversion_processor
            .write(&mut writer, format, AutomaticColorTypeAdjustment::Enabled)
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

    #[test]
    fn test_conversions_with_header_match() -> io::Result<()> {
        for test_image in INPUT_MULTI.iter() {
            let zipped = INPUT_FORMATS
                .iter()
                .zip(OUTPUT_FORMATS.iter().cloned())
                .zip(EXPECTED_VALUES.iter());

            for ((ext, to_format), expected_format) in zipped {
                test_conversion_with_header_match(test_image, ext, to_format, *expected_format)?;
            }
        }

        Ok(())
    }
}
