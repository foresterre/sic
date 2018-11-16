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
