use crate::errors::{FormatError, SicIoError};
use crate::preprocessor::Preprocess;
use image::buffer::ConvertBuffer;
use sic_core::image::codecs::gif::Repeat;
use sic_core::{image, AnimatedImage, SicImage};
use std::io::{Seek, Write};
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, Read};

    use parameterized::parameterized;
    use sic_core::image::ImageFormat;
    use sic_testing::{clean_up_output_path, setup_output_path, setup_test_image};

    use super::*;

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
        let example_output_format = image::ImageFormat::Png;
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
        let example_output_format = image::ImageFormat::Png;
        let conversion_processor = ConversionWriter::new(&buffer);
        conversion_processor
            .write_all(
                &mut File::create(output_path)?,
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
        let example_output_format = image::ImageFormat::Png;
        let conversion_processor = ConversionWriter::new(&buffer);
        conversion_processor
            .write_all(
                &mut File::create(output_path)?,
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
        format: image::ImageFormat,
        expected_format: image::ImageFormat,
    ) -> io::Result<()> {
        let our_output = &format!("header_match_conversion.{}", enc_format); // this is required because tests are run in parallel, and the creation, or deletion can collide.
        let output_path = setup_output_path(our_output);

        let buffer = image::open(setup_test_image(input))
            .expect("Can't open test file.")
            .into();
        let conversion_processor = ConversionWriter::new(&buffer);
        let mut writer = File::create(output_path)?;

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
            image::ImageFormat::Bmp,
            image::ImageFormat::Farbfeld,
            image::ImageFormat::Gif,
            image::ImageFormat::Ico,
            image::ImageFormat::Jpeg(80),
            image::ImageFormat::Jpeg(80),
            image::ImageFormat::Png,
            image::ImageFormat::Pnm(image::codecs::pnm::PnmSubtype::Bitmap(
                image::codecs::pnm::SampleEncoding::Binary,
            )),
            image::ImageFormat::Pnm(image::codecs::pnm::PnmSubtype::Graymap(
                image::codecs::pnm::SampleEncoding::Binary,
            )),
            image::ImageFormat::Pnm(image::codecs::pnm::PnmSubtype::Pixmap(
                image::codecs::pnm::SampleEncoding::Binary,
            )),
            image::ImageFormat::Pnm(
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
        to_format: ImageFormat,
        expected_format: ImageFormat,
    ) {
        for test_image in INPUT_MULTI.iter() {
            test_conversion_with_header_match(test_image, ext, to_format.clone(), expected_format)
                .unwrap();
        }
    }
}
