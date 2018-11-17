use std::path::Path;

use crate::config::Config;
use crate::processor::ProcessWithConfig;

#[derive(Debug)]
pub struct EncodingFormatDecider;

impl EncodingFormatDecider {
    pub fn new() -> EncodingFormatDecider {
        EncodingFormatDecider {}
    }

    fn get_output_extension(config: &Config) -> Option<String> {
        let path = &Path::new(&config.output);
        let extension = path.extension();

        extension
            .and_then(|out| out.to_str())
            .map(|v| v.to_lowercase())
    }

    fn sample_encoding(config: &Config) -> image::pnm::SampleEncoding {
        if config.encoding_settings.pnm_settings.ascii {
            image::pnm::SampleEncoding::Ascii
        } else {
            image::pnm::SampleEncoding::Binary
        }
    }

    // <output format type as String, error message as String>
    fn determine_format_string(config: &Config) -> Result<String, String> {
        if let Some(v) = &config.forced_output_format {
            Ok(v.to_lowercase())
        } else {
            EncodingFormatDecider::get_output_extension(config).ok_or_else(|| "Unable to determine a supported image format type from the image's file extension.".to_string())
        }
    }

    fn determine_format_from_str(
        config: &Config,
        identifier: &str,
    ) -> Result<image::ImageOutputFormat, String> {
        match identifier {
            "bmp" => Ok(image::ImageOutputFormat::BMP),
            "gif" => Ok(image::ImageOutputFormat::GIF),
            "ico" => Ok(image::ImageOutputFormat::ICO),
            "jpeg" | "jpg" => Ok(image::ImageOutputFormat::JPEG(
                config.encoding_settings.jpeg_settings.quality,
            )),
            "png" => Ok(image::ImageOutputFormat::PNG),
            "pbm" => {
                let sample_encoding = EncodingFormatDecider::sample_encoding(&config);

                Ok(image::ImageOutputFormat::PNM(
                    image::pnm::PNMSubtype::Bitmap(sample_encoding),
                ))
            }
            "pgm" => {
                let sample_encoding = EncodingFormatDecider::sample_encoding(&config);

                Ok(image::ImageOutputFormat::PNM(
                    image::pnm::PNMSubtype::Graymap(sample_encoding),
                ))
            }
            "ppm" => {
                let sample_encoding = EncodingFormatDecider::sample_encoding(&config);

                Ok(image::ImageOutputFormat::PNM(
                    image::pnm::PNMSubtype::Pixmap(sample_encoding),
                ))
            }
            "pam" => Ok(image::ImageOutputFormat::PNM(
                image::pnm::PNMSubtype::ArbitraryMap,
            )),
            _ => Err(format!(
                "Unable to determine a supported image format type, input: {}.",
                identifier
            )),
        }
    }

    fn compute_format(config: &Config) -> Result<image::ImageOutputFormat, String> {
        // 1. get the format type
        //   a. if not -f or and we have an extension
        //      use the extension to determine the type
        //   b. if  -f v
        //      use v to determine the type
        //   c. else
        //      unable to determine format error
        let format = EncodingFormatDecider::determine_format_string(&config);

        // 2. match on additional options such as PNM's subtype or JPEG's quality
        //    ensure that user set cases are above default cases
        EncodingFormatDecider::determine_format_from_str(&config, &format?)
    }
}

impl ProcessWithConfig<Result<image::ImageOutputFormat, String>> for EncodingFormatDecider {
    fn process(&self, config: &Config) -> Result<image::ImageOutputFormat, String> {
        EncodingFormatDecider::compute_format(&config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        Config, FormatEncodingSettings, JPEGEncodingSettings, PNMEncodingSettings,
    };
    use crate::processor::mod_test_includes::*;

    const INPUT: &str = "rainbow_8x6.bmp";
    const OUTPUT_NO_EXT: &str = "encoding_rainbow_8x6";

    fn setup_dummy_config(output: &str, ext: &str) -> Config {
        Config {
            licenses: vec![],
            user_manual: None,
            script: None,
            forced_output_format: None,

            encoding_settings: FormatEncodingSettings {
                jpeg_settings: JPEGEncodingSettings::new_result((false, None))
                    .expect("Invalid jpeg settings"),
                pnm_settings: PNMEncodingSettings::new(false),
            },

            output: String::from(
                setup_output_path(&format!("{}.{}", output, ext))
                    .to_str()
                    .expect("Path given is no good!"),
            ),
        }
    }

    fn test_with_extension(ext: &str) {

    }


    fn test_with_force_format(f: &str) {
        
    }

    #[test]
    fn which_encoding_bmp_extension() {
        let our_output = &format!("encoding_processing_{}", OUTPUT_NO_EXT); // this is required because tests are run in parallel, and the creation, or deletion can collide with other image file of the same name.

        let settings = setup_dummy_config(our_output, ext);

        let conversion_processor = EncodingFormatDecider::new();
        let result = conversion_processor
            .process(&settings)
            .expect("Unable to save file to the test computer.");

        // TODO
        assert_eq!(image::ImageOutputFormat::BMP, result);
        
    }
}
