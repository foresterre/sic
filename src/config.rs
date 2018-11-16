use std::path::Path;
use std::process;

use crate::help::HelpIndex;
use crate::operations;
use crate::operations::transformations::apply_operations_on_image;
use crate::operations::Operation;

// Currently uses String instead of &str for easier initial development (i.e. no manual lifetimes).
// It should be replaced by &str where possible.
#[derive(Debug)]
pub struct Config {
    // Display license of this software or its dependencies.
    pub licenses: Vec<SelectedLicenses>,

    // User manual with help topics; provided argument is a help topic;
    // Should display the index on None or non-existing topic.
    // Perhaps this can be removed in the future; Clap has long_help() built in on the type Arg.
    pub user_manual: Option<String>,

    // Image transformation script
    pub script: Option<String>,

    // Format to which an image will be converted (enforced).
    pub forced_output_format: Option<String>,

    pub encoding_settings: FormatEncodingSettings,

    // output path
    pub output: String,
}

#[derive(Debug)]
pub struct FormatEncodingSettings {
    pub jpeg_settings: JPEGEncodingSettings,

    pub pnm_settings: PNMEncodingSettings,
}

#[derive(Debug)]
pub struct JPEGEncodingSettings {
    // Valid values are actually 1...100 (inclusive)
    pub quality: u8,
}

impl JPEGEncodingSettings {
    const JPEG_ENCODING_QUALITY_DEFAULT: u8 = 80;

    // Param:
    // * quality: (present?, value)
    pub fn new_result(quality: (bool, Option<&str>)) -> Result<JPEGEncodingSettings, String> {
        let proposed_quality = match quality.1 {
            Some(v) => v
                .parse::<u8>()
                .map_err(|_| "JPEG Encoding Settings error: QUALITY is not a valid number.".into()),
            None if !quality.0 => Ok(JPEGEncodingSettings::JPEG_ENCODING_QUALITY_DEFAULT),
            None => Err("JPEG Encoding Settings error: Unreachable".into()),
        };

        fn within_range(v: u8) -> Result<JPEGEncodingSettings, String> {
            const ALLOWED_RANGE: std::ops::RangeInclusive<u8> = 1..=100;
            if ALLOWED_RANGE.contains(&v) {
                let res = JPEGEncodingSettings { quality: v };

                Ok(res)
            } else {
                Err("JPEG Encoding Settings error: --jpeg-encoding-quality requires a number between 1 and 100.".into())
            }
        }

        proposed_quality.and_then(within_range)
    }
}

#[derive(Debug)]
pub struct PNMEncodingSettings {
    // Use ascii for PBM, PGM or PPM. Not compatible with PAM.
    pub ascii: bool,
}

impl PNMEncodingSettings {
    pub fn new(ascii: bool) -> PNMEncodingSettings {
        PNMEncodingSettings { ascii }
    }
}

/// Linear application pipeline trait for immutable references.
pub trait ProcessWithConfig<T> {
    fn process(&self, config: &Config) -> T;
}

/// Linear application pipeline trait for mutable references.
pub trait ProcessMutWithConfig<T> {
    fn process_mut(&mut self, config: &Config) -> T;
}

const SIC_LICENSE: &str = include_str!("../LICENSE");
const DEP_LICENSES: &str = include_str!("../LICENSES_DEPENDENCIES");

#[derive(Debug)]
pub enum SelectedLicenses {
    ThisSoftware,
    Dependencies,
}

#[derive(Debug)]
pub struct LicenseDisplayProcessor;

impl LicenseDisplayProcessor {
    pub fn new() -> LicenseDisplayProcessor {
        LicenseDisplayProcessor {}
    }

    fn print_licenses(slice: &[SelectedLicenses]) {
        for item in slice {
            match item {
                SelectedLicenses::ThisSoftware => {
                    println!("Simple Image Converter license: \n\n{}\n\n", SIC_LICENSE);
                }
                SelectedLicenses::Dependencies => println!("{}", DEP_LICENSES),
            };
        }

        if !slice.is_empty() {
            process::exit(0);
        }
    }
}

impl ProcessWithConfig<()> for LicenseDisplayProcessor {
    fn process(&self, config: &Config) {
        LicenseDisplayProcessor::print_licenses(&config.licenses);
    }
}

// TODO{foresterre}: User manual should be refactored later.
#[derive(Debug)]
pub struct HelpDisplayProcessor;

impl HelpDisplayProcessor {
    pub fn new() -> HelpDisplayProcessor {
        HelpDisplayProcessor {}
    }

    fn print_help(help: &HelpIndex, topic: &str) {
        let page = help.get_topic(&*topic.to_lowercase());

        match page {
            Some(it) => println!("{}", it.help_text),
            None => println!("This topic is unavailable in the user manual. The following topics are available: \n\t* {}", help.get_available_topics()),
        }
    }
}

impl ProcessWithConfig<()> for HelpDisplayProcessor {
    fn process(&self, config: &Config) {
        if let Some(topic) = &config.user_manual {
            let help = HelpIndex::new();

            if topic == "index" {
                println!(
                    "The following topics are available: \n\t* {}",
                    help.get_available_topics()
                );
            } else {
                HelpDisplayProcessor::print_help(&help, &topic);
            }

            process::exit(0);
        }
    }
}

pub struct ImageOperationsProcessor<'a> {
    buffer: &'a mut image::DynamicImage,
}

impl<'a> ImageOperationsProcessor<'a> {
    pub fn new(buffer: &'a mut image::DynamicImage) -> ImageOperationsProcessor {
        ImageOperationsProcessor { buffer }
    }

    fn parse_script(&self, config: &Config) -> Result<Vec<Operation>, String> {
        println!("Parsing image operations script.");

        match &config.script {
            Some(it) => operations::parse_script(&it),
            None => Err("Script unavailable.".into()),
        }
    }

    fn apply_operations(&mut self, ops: &[Operation]) -> Result<(), String> {
        println!("Applying image operations.");

        apply_operations_on_image(&mut self.buffer, ops)
    }
}

impl<'a> ProcessMutWithConfig<Result<(), String>> for ImageOperationsProcessor<'a> {
    fn process_mut(&mut self, config: &Config) -> Result<(), String> {
        // If we don't have the script option defined, do nothing.
        if config.script.is_some() {
            let operations = self.parse_script(config);

            self.apply_operations(&operations?)
        } else {
            Ok(())
        }
    }
}

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
}

impl<'a> ProcessWithConfig<Result<(), String>> for ConversionProcessor<'a> {
    fn process(&self, config: &Config) -> Result<(), String> {
        let mut out = std::fs::File::create(&std::path::Path::new(&config.output))
            .map_err(|err| err.to_string())?;

        let output_format = self.output_format.clone();

        self.image
            .write_to(&mut out, output_format)
            .map_err(|err| err.to_string())
    }
}
