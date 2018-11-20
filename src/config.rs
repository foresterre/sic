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

    // --disable-automatic-color-type-adjustment
    pub disable_automatic_color_type_adjustment: bool,

    pub encoding_settings: FormatEncodingSettings,

    // output path
    pub output: String,
}

#[derive(Debug)]
pub enum SelectedLicenses {
    ThisSoftware,
    Dependencies,
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
