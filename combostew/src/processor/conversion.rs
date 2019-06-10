use std::io::{self, Write};
use std::path::Path;

use crate::config::Config;
use crate::processor::ProcessWithConfig;

pub struct ConversionProcessor<'a> {
    image: &'a image::DynamicImage,
    output_format: image::ImageOutputFormat,
}

impl<'a> ConversionProcessor<'a> {
    pub fn new(
        image: &image::DynamicImage,
        output_format: image::ImageOutputFormat,
    ) -> ConversionProcessor {
        ConversionProcessor {
            image,
            output_format,
        }
    }

    // Some image output format types require color type preprocessing.
    // This is the case if the output image format does not support the color type held by the image buffer prior to the final conversion.
    //
    // The open question remains as follows: does a user expect for an image to be able to convert to a format even if the color type is not supported?
    // And even if the user does, should we?
    // I suspect that users expect that color type conversions should happen automatically.
    //
    // Testing also showed that even bmp with full black full white pixels do not convert correctly as of now. Why exactly is unclear;
    // Perhaps the color type of the bmp formatted test image?
    //
    // If preprocessing of the color type took place, Some(<new image>) will be returned.
    // If no preprocessing of the color type is required will return None.
    fn preprocess_color_type(
        config: &Config,
        image: &image::DynamicImage,
        output_format: &image::ImageOutputFormat,
    ) -> Option<image::DynamicImage> {
        if config.disable_automatic_color_type_adjustment {
            return None;
        }

        match output_format {
            image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Bitmap(_)) => {
                Some(image.grayscale())
            }
            image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Graymap(_)) => {
                Some(image.grayscale())
            }
            image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Pixmap(_)) => {
                Some(image::DynamicImage::ImageRgb8(image.to_rgb()))
            }
            _ => None,
        }
    }

    fn save_to_file<P: AsRef<Path>>(
        buffer: &image::DynamicImage,
        format: image::ImageOutputFormat,
        path: P,
    ) -> Result<(), String> {
        let mut out = std::fs::File::create(path).map_err(|err| err.to_string())?;

        buffer
            .write_to(&mut out, format)
            .map_err(|err| err.to_string())
    }

    fn export_to_stdout(
        buffer: &image::DynamicImage,
        format: image::ImageOutputFormat,
    ) -> Result<(), String> {
        let mut write_buffer = Vec::new();

        buffer
            .write_to(&mut write_buffer, format)
            .map_err(|err| err.to_string())?;

        io::stdout()
            .write(&write_buffer)
            .map(|_| ())
            .map_err(|err| err.to_string())
    }
}

impl<'a> ProcessWithConfig<Result<(), String>> for ConversionProcessor<'a> {
    fn process(&self, config: &Config) -> Result<(), String> {
        let output_format = self.output_format.clone();
        let color_processing =
            &ConversionProcessor::preprocess_color_type(&config, &self.image, &output_format);

        let export_buffer = match color_processing {
            Some(replacement) => replacement,
            None => &self.image,
        };

        match &config.output {
            // Some() => write to file
            Some(v) => ConversionProcessor::save_to_file(&export_buffer, output_format, v),
            // None => write to stdout
            None => ConversionProcessor::export_to_stdout(&export_buffer, output_format),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::config::ConfigItem;

    use crate::config::{
        Config, FormatEncodingSettings, JPEGEncodingSettings, PNMEncodingSettings,
    };
    use crate::processor::mod_test_includes::*;

    use super::*;

    // Individual tests:

    const INPUT: &str = "rainbow_8x6.bmp";
    const OUTPUT: &str = "conversion_rainbow_8x6.png";

    fn setup_dummy_config(output: &str) -> Config {
        Config {
            tool_name: env!("CARGO_PKG_NAME"),
            licenses: vec![],
            forced_output_format: None,
            disable_automatic_color_type_adjustment: false,

            encoding_settings: FormatEncodingSettings {
                jpeg_settings: JPEGEncodingSettings::new_result((false, None))
                    .expect("Invalid jpeg settings"),
                pnm_settings: PNMEncodingSettings::new(false),
            },

            output: setup_output_path(output).to_str().map(|v| v.into()),

            application_specific: vec![
                ConfigItem::OptionStringItem(None),
                ConfigItem::OptionStringItem(None),
            ],
        }
    }

    #[test]
    fn will_output_file_be_created() {
        let our_output = &format!("will_output_file_be_created{}", OUTPUT); // this is required because tests are run in parallel, and the creation, or deletion can collide.

        let buffer = image::open(setup_test_image(INPUT)).expect("Can't open test file.");
        let example_output_format = image::ImageOutputFormat::PNG;
        let settings = setup_dummy_config(our_output);

        let conversion_processor = ConversionProcessor::new(&buffer, example_output_format);
        conversion_processor
            .process(&settings)
            .expect("Unable to save file to the test computer.");

        assert!(setup_output_path(our_output).exists());

        clean_up_output_path(our_output);
    }

    #[test]
    fn has_png_extension() {
        let our_output = &format!("has_png_extension{}", OUTPUT);

        let buffer = image::open(setup_test_image(INPUT)).expect("Can't open test file.");
        let example_output_format = image::ImageOutputFormat::PNG;
        let settings = setup_dummy_config(our_output);

        let conversion_processor = ConversionProcessor::new(&buffer, example_output_format);
        conversion_processor
            .process(&settings)
            .expect("Unable to save file to the test computer.");

        assert_eq!(
            Some(std::ffi::OsStr::new("png")),
            setup_output_path(our_output).extension()
        );

        clean_up_output_path(our_output);
    }

    #[test]
    fn is_png_file() {
        let our_output = &format!("is_png_file{}", OUTPUT);

        let buffer = image::open(setup_test_image(INPUT)).expect("Can't open test file.");
        let example_output_format = image::ImageOutputFormat::PNG;
        let settings = setup_dummy_config(our_output);

        let conversion_processor = ConversionProcessor::new(&buffer, example_output_format);
        conversion_processor
            .process(&settings)
            .expect("Unable to save file to the test computer.");

        let mut file = std::fs::File::open(setup_output_path(our_output))
            .expect("Unable to find file we made.");
        let mut bytes = vec![];
        file.read_to_end(&mut bytes)
            .expect("Unable to finish reading our test image.");

        assert_eq!(
            image::ImageFormat::PNG,
            image::guess_format(&bytes).expect("Format could not be guessed.")
        );

        clean_up_output_path(our_output);
    }

    // Multi tests:
    // Below all supported formats are testsed using the inputs listed below.

    const INPUT_MULTI: &[&str] = &["blackwhite_2x2.bmp", "palette_4x4.png"];
    const INPUT_FORMATS: &[&str] = &[
        "bmp", "gif", "ico", "jpg", "jpeg", "png", "pbm", "pgm", "ppm", "pam",
    ];
    const OUTPUT_FORMATS: &[image::ImageOutputFormat] = &[
        image::ImageOutputFormat::BMP,
        image::ImageOutputFormat::GIF,
        image::ImageOutputFormat::ICO,
        image::ImageOutputFormat::JPEG(80),
        image::ImageOutputFormat::JPEG(80),
        image::ImageOutputFormat::PNG,
        image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Bitmap(
            image::pnm::SampleEncoding::Binary,
        )),
        image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Graymap(
            image::pnm::SampleEncoding::Binary,
        )),
        image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Pixmap(
            image::pnm::SampleEncoding::Binary,
        )),
        image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::ArbitraryMap),
    ];

    const EXPECTED_VALUES: &[image::ImageFormat] = &[
        image::ImageFormat::BMP,
        image::ImageFormat::GIF,
        image::ImageFormat::ICO,
        image::ImageFormat::JPEG,
        image::ImageFormat::JPEG,
        image::ImageFormat::PNG,
        image::ImageFormat::PNM,
        image::ImageFormat::PNM,
        image::ImageFormat::PNM,
        image::ImageFormat::PNM,
    ];

    fn test_conversion_with_header_match(
        input: &str,
        enc_format: &str,
        to_format: image::ImageOutputFormat,
        expected_format: image::ImageFormat,
    ) {
        let our_output = &format!("header_match_conversion.{}", enc_format);

        let buffer = image::open(setup_test_image(input)).expect("Can't open test file.");
        let settings = setup_dummy_config(our_output);

        let conversion_processor = ConversionProcessor::new(&buffer, to_format);
        conversion_processor
            .process(&settings)
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
    }

    #[test]
    fn test_conversions_with_header_match() {
        for test_image in INPUT_MULTI.iter() {
            let zipped = INPUT_FORMATS
                .iter()
                .zip(OUTPUT_FORMATS.iter().cloned())
                .zip(EXPECTED_VALUES.iter());

            for ((ext, to_format), expected_format) in zipped {
                println!(
                    "testing `test_conversion_with_header_match`, converting {} => : {}",
                    test_image, ext
                );
                test_conversion_with_header_match(test_image, ext, to_format, *expected_format);
            }
        }
    }
}
