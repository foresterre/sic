use std::path::Path;

use crate::config::Config;
use crate::processor::ProcessWithConfig;

#[derive(Debug)]
pub struct EncodingFormatDecider;

impl EncodingFormatDecider {
    pub fn new() -> EncodingFormatDecider {
        EncodingFormatDecider {}
    }

    // return: Ok: valid extension, err: invalid i.e. no extension or no valid output path
    fn get_output_extension(config: &Config) -> Result<String, String> {
        match &config.output {
            Some(v) => {
                let path = &Path::new(v);
                let extension = path.extension();

                extension
                    .and_then(|out| out.to_str())
                    .ok_or_else(|| "No extension was found".into())
                    .map(|v| v.to_lowercase())
            }
            None => Err("No valid output path found (type: efd/ext)".into()),
        }
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
            EncodingFormatDecider::get_output_extension(config)
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

    const OUTPUT_NO_EXT: &str = "dont_care";
    const INPUT_FORMATS: &[&str] = &[
        "bmp", "gif", "ico", "jpg", "jpeg", "png", "pbm", "pgm", "ppm", "pam",
    ];
    const EXPECTED_VALUES: &[image::ImageOutputFormat] = &[
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

    fn setup_dummy_config(
        output: &str,
        ext: &str,
        force_format: Option<String>,
        pnm_ascii: bool,
    ) -> Config {
        Config {
            licenses: vec![],
            user_manual: None,
            script: None,
            forced_output_format: force_format,
            disable_automatic_color_type_adjustment: false,

            encoding_settings: FormatEncodingSettings {
                jpeg_settings: JPEGEncodingSettings::new_result((false, None))
                    .expect("Invalid jpeg settings"),
                pnm_settings: PNMEncodingSettings::new(pnm_ascii),
            },

            output: setup_output_path(&format!("{}.{}", output, ext))
                .to_str()
                .map(|v| v.into()),
        }
    }

    fn test_with_extension(ext: &str, expected: &image::ImageOutputFormat) {
        let output_name = &format!("encoding_processing_w_ext_{}", OUTPUT_NO_EXT);

        let settings = setup_dummy_config(output_name, ext, None, false);

        let conversion_processor = EncodingFormatDecider::new();
        let result = conversion_processor
            .process(&settings)
            .expect("Failed to compute image format.");

        assert_eq!(*expected, result);
    }

    fn test_with_force_format(format: &str, expected: &image::ImageOutputFormat) {
        let output_name = &format!("encoding_processing_w_ff_{}", OUTPUT_NO_EXT);

        let settings = setup_dummy_config(output_name, "", Some(String::from(format)), false);

        let conversion_processor = EncodingFormatDecider::new();
        let result = conversion_processor
            .process(&settings)
            .expect("Failed to compute image format.");

        assert_eq!(*expected, result);
    }

    #[test]
    fn test_with_extensions_with_defaults() {
        let zipped = INPUT_FORMATS.iter().zip(EXPECTED_VALUES.iter());

        for (ext, exp) in zipped {
            println!("testing `test_with_extension`: {}", ext);
            test_with_extension(ext, exp);
        }
    }

    #[test]
    fn test_with_force_formats_with_defaults() {
        let zipped = INPUT_FORMATS.iter().zip(EXPECTED_VALUES.iter());

        for (format, exp) in zipped {
            println!("testing `test_with_force_format`: {}", format);
            test_with_force_format(format, exp);
        }
    }

    #[test]
    fn test_with_extension_jpg_and_force_format_png() {
        let output_name = &format!("encoding_processing_w_ext_and_ff_{}", OUTPUT_NO_EXT);

        let settings = setup_dummy_config(output_name, "jpg", Some(String::from("png")), false);

        let conversion_processor = EncodingFormatDecider::new();
        let result = conversion_processor
            .process(&settings)
            .expect("Failed to compute image format.");

        assert_eq!(image::ImageOutputFormat::PNG, result);
    }

    #[test]
    fn test_with_extension_and_ascii_pbm() {
        let output_name = &format!("encoding_processing_ascii_pbm_{}", OUTPUT_NO_EXT);

        let settings = setup_dummy_config(output_name, "pbm", None, true);

        let conversion_processor = EncodingFormatDecider::new();
        let result = conversion_processor
            .process(&settings)
            .expect("Failed to compute image format.");

        assert_eq!(
            image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Bitmap(
                image::pnm::SampleEncoding::Ascii,
            )),
            result
        );
    }

    #[test]
    fn test_with_extension_and_ascii_pgm() {
        let output_name = &format!("encoding_processing_ascii_pgm_{}", OUTPUT_NO_EXT);

        let settings = setup_dummy_config(output_name, "pgm", None, true);

        let conversion_processor = EncodingFormatDecider::new();
        let result = conversion_processor
            .process(&settings)
            .expect("Failed to compute image format.");

        assert_eq!(
            image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Graymap(
                image::pnm::SampleEncoding::Ascii,
            )),
            result
        );
    }

    #[test]
    fn test_with_extension_and_ascii_ppm() {
        let output_name = &format!("encoding_processing_ascii_ppm_{}", OUTPUT_NO_EXT);

        let settings = setup_dummy_config(output_name, "ppm", None, true);

        let conversion_processor = EncodingFormatDecider::new();
        let result = conversion_processor
            .process(&settings)
            .expect("Failed to compute image format.");

        assert_eq!(
            image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Pixmap(
                image::pnm::SampleEncoding::Ascii,
            )),
            result
        );
    }

    #[test]
    fn test_with_extension_and_ascii_pam_doesnt_care() {
        // PAM is not influenced by the PNM ascii setting
        let output_name = &format!(
            "encoding_processing_ascii_pam_doesnt_care_{}",
            OUTPUT_NO_EXT
        );

        let settings = setup_dummy_config(output_name, "pam", None, true);

        let conversion_processor = EncodingFormatDecider::new();
        let result = conversion_processor
            .process(&settings)
            .expect("Failed to compute image format.");

        assert_eq!(
            image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::ArbitraryMap),
            result
        );
    }

    #[test]
    fn test_jpeg_custom_quality() {
        let jpeg_conf = Config {
            licenses: vec![],
            user_manual: None,
            script: None,
            forced_output_format: None,
            disable_automatic_color_type_adjustment: false,

            encoding_settings: FormatEncodingSettings {
                jpeg_settings: JPEGEncodingSettings::new_result((true, Some("40")))
                    .expect("Invalid jpeg settings"),
                pnm_settings: PNMEncodingSettings::new(false),
            },

            output: setup_output_path("encoding_processing_jpeg_quality_valid.jpg")
                .to_str()
                .map(|v| v.into()),
        };

        let conversion_processor = EncodingFormatDecider::new();
        let result = conversion_processor
            .process(&jpeg_conf)
            .expect("Failed to compute image format.");

        assert_eq!(image::ImageOutputFormat::JPEG(40), result);
    }

    #[should_panic]
    #[test]
    fn test_output_unsupported_extension() {
        let jpeg_conf = Config {
            licenses: vec![],
            user_manual: None,
            script: None,
            forced_output_format: None,
            disable_automatic_color_type_adjustment: false,

            encoding_settings: FormatEncodingSettings {
                jpeg_settings: JPEGEncodingSettings { quality: 90 },
                pnm_settings: PNMEncodingSettings::new(false),
            },

            output: setup_output_path("encoding_processing_invalid.😉")
                .to_str()
                .map(|v| v.into()),
        };

        let conversion_processor = EncodingFormatDecider::new();
        let _ = conversion_processor
            .process(&jpeg_conf)
            .expect("Failed to compute image format.");
    }

    #[should_panic]
    #[test]
    fn test_output_no_ext_or_ff() {
        let jpeg_conf = Config {
            licenses: vec![],
            user_manual: None,
            script: None,
            forced_output_format: None,
            disable_automatic_color_type_adjustment: false,

            encoding_settings: FormatEncodingSettings {
                jpeg_settings: JPEGEncodingSettings { quality: 90 },
                pnm_settings: PNMEncodingSettings::new(false),
            },

            output: setup_output_path("encoding_processing_invalid.")
                .to_str()
                .map(|v| v.into()),
        };

        let conversion_processor = EncodingFormatDecider::new();
        let _ = conversion_processor
            .process(&jpeg_conf)
            .expect("Failed to compute image format.");
    }

    #[should_panic]
    #[test]
    fn test_output_unsupported_ff_with_ext() {
        let jpeg_conf = Config {
            licenses: vec![],
            user_manual: None,
            script: None,
            forced_output_format: Some("OiOi".into()), // unsupported format
            disable_automatic_color_type_adjustment: false,

            encoding_settings: FormatEncodingSettings {
                jpeg_settings: JPEGEncodingSettings { quality: 90 },
                pnm_settings: PNMEncodingSettings::new(false),
            },

            output: setup_output_path("encoding_processing_invalid.jpg")
                .to_str()
                .map(|v| v.into()),
        };

        let conversion_processor = EncodingFormatDecider::new();
        let _ = conversion_processor
            .process(&jpeg_conf)
            .expect("Unable to save file to the test computer");
    }

    #[should_panic]
    #[test]
    fn test_output_unsupported_ff_without_ext() {
        let jpeg_conf = Config {
            licenses: vec![],
            user_manual: None,
            script: None,
            forced_output_format: Some("OiOi".into()), // unsupported format
            disable_automatic_color_type_adjustment: false,

            encoding_settings: FormatEncodingSettings {
                jpeg_settings: JPEGEncodingSettings { quality: 90 },
                pnm_settings: PNMEncodingSettings::new(false),
            },

            output: setup_output_path("encoding_processing_invalid")
                .to_str()
                .map(|v| v.into()),
        };

        let conversion_processor = EncodingFormatDecider::new();
        let _ = conversion_processor
            .process(&jpeg_conf)
            .expect("Unable to save file to the test computer");
    }

    // TODO{}: test bad cases, edges
}
