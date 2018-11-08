use std::process;

use crate::help::HelpIndex;
use crate::operations;
use crate::operations::transformations::apply_operations_on_image;
use crate::operations::Operation;

// Currently uses String instead of &str for easier initial development (i.e. no manual lifetimes).
// It should be replaced by &str where possible.
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
}

pub struct FormatEncodingSettings {
    pub jpeg_settings: JPEGEncodingSettings,

    pub pnm_settings: PNMEncodingSettings,
}

pub struct JPEGEncodingSettings {
    // Valid values are actually 1...100 (inclusive)
    pub quality: u8,
}

impl JPEGEncodingSettings {
    const JPEG_ENCODING_QUALITY_DEFAULT: u8 = 80;

    // Param:
    // * quality: (present?, value)
    // Can panic on invalid settings
    pub fn new(quality: (bool, Option<&str>)) -> JPEGEncodingSettings {
        let proposed_quality = match quality.1 {
            Some(v) => v.parse::<u8>(),
            None if !quality.0 => Ok(JPEGEncodingSettings::JPEG_ENCODING_QUALITY_DEFAULT),
            None => panic!("JPEG Encoding Settings error: Unreachable"),
        };

        const ALLOWED_RANGE: std::ops::Range<u8> = 1..100;

        let res_quality: u8 = match proposed_quality {
            Ok(v) if ALLOWED_RANGE.contains(&v) => v,
            Ok(_) => panic!("JPEG Encoding Settings error: --jpeg-encoding-quality requires a number between 1 and 100. (type 2: value not in range)"),
            Err(_) => panic!("JPEG Encoding Settings error: --jpeg-encoding-quality requires a number between 1 and 100. (type 1: not a valid number)"),
        };

        // result
        JPEGEncodingSettings {
            quality: res_quality,
        }
    }
}

// Subtype + ascii combined create an image::PNMSubtype;
// see https://docs.rs/image/0.19.0/image/pnm/enum.PNMSubtype.html
// Ascii option can not be used with ArbitraryMap
pub struct PNMEncodingSettings {
    // This option defines whether PNM encoding will use a binary or a ascii based encoding.
    // The default encoding is 'binary' (false).
    pub ascii: bool,

    // Setting which defines the subtype of the PNM image type.
    // Valid options are:
    // - "bitmap" (aka: pbm)
    // - "graymap" (aka: pgm)
    // - "pixmap" (aka: ppm)
    // - "arbitrarymap" (aka: pam)
    // These options are possibly lossy.
    pub subtype: PNMEncodingSubtype,
}

impl PNMEncodingSettings {
    const PNM_ENCODING_SUBTYPE_DEFAULT: PNMEncodingSubtype = PNMEncodingSubtype::Pixmap;

    // Param:
    //  * as_ascii: present?  ; flags can only be present or not present
    //  * subtype: (present?, value)
    // Can panic on invalid settings
    pub fn new(as_ascii: bool, subtype: (bool, Option<&str>)) -> PNMEncodingSettings {
        // option subtype not present
        let res_subtype = if !subtype.0 {
            PNMEncodingSettings::PNM_ENCODING_SUBTYPE_DEFAULT
        } else {
            match subtype.1 {
                Some("bitmap") => PNMEncodingSubtype::Bitmap,
                Some("graymap") => PNMEncodingSubtype::Graymap,
                Some("pixmap") => PNMEncodingSubtype::Pixmap,
                Some("arbitrarymap") if !as_ascii => PNMEncodingSubtype::ArbitraryMap,
                Some("arbitrarymap") => panic!("PNM Encoding Settings error: the option --pnm-encoding-subtype 'arbitrarymap' can not be used when used with --pnm-encoding-ascii."),
                _ => panic!("PNM Encoding Settings error: The provided subtype is not a valid option.")
            }
        };

        PNMEncodingSettings {
            ascii: as_ascii,
            subtype: res_subtype,
        }
    }
}

pub enum PNMEncodingSubtype {
    Bitmap,
    Graymap,
    Pixmap,
    ArbitraryMap,
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

pub enum SelectedLicenses {
    ThisSoftware,
    Dependencies,
}

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

// User manual will be refactored later.
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
