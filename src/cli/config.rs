use crate::cli::app::arg_names::{ARG_INPUT, ARG_MODE, ARG_OUTPUT};
use crate::cli::common_dir::CommonDir;
use anyhow::bail;
use clap::ArgMatches;
use sic_image_engine::engine::Instr;
use sic_io::load::FrameIndex;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum PathVariant {
    StdStream,
    Path(PathBuf),
}

impl PathVariant {
    pub fn is_std_stream(&self) -> bool {
        match self {
            PathVariant::StdStream => true,
            PathVariant::Path(_) => false,
        }
    }
}

pub enum InputOutputMode {
    Single {
        input: PathVariant,
        output: PathVariant,
    },
    Batch {
        inputs: CommonDir,
        output_root_folder: PathBuf,
    },
}

impl InputOutputMode {
    pub fn try_from_matches(matches: &ArgMatches) -> anyhow::Result<Self> {
        let mode = matches.value_of(ARG_MODE);
        let input = matches.value_of(ARG_INPUT);
        let output = matches.value_of(ARG_OUTPUT);

        let res = match (mode, input, output) {
            (Some("simple"), input, output) => InputOutputMode::Single {
                input: match input {
                    Some(p) => PathVariant::Path(p.into()),
                    None => PathVariant::StdStream,
                },
                output: match output {
                    Some(p) => PathVariant::Path(p.into()),
                    None => PathVariant::StdStream,
                },
            },
            (Some("glob"), Some(inputs), Some(output)) => InputOutputMode::Batch {
                inputs: {
                    let inputs = globwalk::glob(inputs)?;
                    let paths = inputs.map(|entry| entry.map_err(|err| {
                        anyhow::anyhow!("Error while trying to find glob matches on the fs ({})", err)
                    }).map(|f| f.into_path())).collect::<anyhow::Result<Vec<_>>>()?;

                    CommonDir::try_new(paths)?
                },
                output_root_folder: {
                    output.into()
                },
            },
            _ => bail!("unable to set mode: found invalid combination of mode, input arguments, and output argument"),
        };

        Ok(res)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InputOutputModeType {
    Simple,
    Batch,
}

#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub tool_name: &'static str,

    pub mode: InputOutputModeType,

    // organisational
    /// Display license of this software or its dependencies.
    pub show_license_text_of: Option<SelectedLicenses>,

    pub selected_frame: FrameIndex,

    /// Disable color type adjustments on save.
    pub disable_automatic_color_type_adjustment: bool,

    /// Format to which an image will be converted (enforced).
    pub forced_output_format: Option<&'a str>,

    /// Encoding settings for specific output formats.
    pub encoding_settings: FormatEncodingSettings,

    /// If a user wants to perform image operations on input image, they will need to provide
    /// the image operation commands.
    /// THe value set here should be presented as a [sic_image_engine::engine::Program].
    /// If no program is present, an empty vec should be provided.
    pub image_operations_program: Vec<Instr>,
}

impl Default for Config<'_> {
    fn default() -> Self {
        Config {
            /// If using default, requires the `CARGO_PKG_NAME` to be set.
            tool_name: env!("CARGO_PKG_NAME"),

            mode: InputOutputModeType::Simple,

            /// Defaults to no displayed license text.
            show_license_text_of: None,

            /// By default the first frame of a gif is used.
            selected_frame: FrameIndex::First,

            /// Defaults to using automatic color type adjustment where appropriate.
            disable_automatic_color_type_adjustment: false,

            /// Defaults to not forcing a specific image output format.
            forced_output_format: None,

            /// Default format encoding settings.
            encoding_settings: FormatEncodingSettings {
                /// Default JPEG quality is set to 80.
                jpeg_quality: 80,

                /// Default encoding type of PNM files (excluding PAM) is set to binary.
                pnm_use_ascii_format: false,
            },

            /// Defaults to no provided image operations script.
            image_operations_program: Vec::new(),
        }
    }
}

/// Builder for [crate::config::Config]. Should be used with the Default implementation
/// of [crate::config::Config].
/// If the default trait is not used with this builder, some settings may be inaccessible.
/// For example, `output_path` can be set to some value, but not unset.
///
/// Builder is consuming.
#[derive(Debug, Default)]
pub struct ConfigBuilder<'a> {
    settings: Config<'a>,
}

impl<'a> ConfigBuilder<'a> {
    pub fn new() -> Self {
        ConfigBuilder::default()
    }

    pub fn mode(mut self, mode: InputOutputModeType) -> ConfigBuilder<'a> {
        self.settings.mode = mode;
        self
    }

    // organisational
    pub fn show_license_text_of(mut self, selection: SelectedLicenses) -> ConfigBuilder<'a> {
        self.settings.show_license_text_of = Some(selection);
        self
    }

    // config(in)
    pub fn select_frame(mut self, frame: FrameIndex) -> ConfigBuilder<'a> {
        self.settings.selected_frame = frame;
        self
    }

    // config(out)
    pub fn forced_output_format(mut self, format: &'a str) -> ConfigBuilder<'a> {
        self.settings.forced_output_format = Some(format);
        self
    }

    // config(out)
    pub fn disable_automatic_color_type_adjustment(mut self, toggle: bool) -> ConfigBuilder<'a> {
        self.settings.disable_automatic_color_type_adjustment = toggle;
        self
    }

    // config(out)
    pub fn jpeg_quality(mut self, quality: u8) -> ConfigBuilder<'a> {
        self.settings.encoding_settings.jpeg_quality = quality;
        self
    }

    // config(out)
    pub fn pnm_format_type(mut self, use_ascii: bool) -> ConfigBuilder<'a> {
        self.settings.encoding_settings.pnm_use_ascii_format = use_ascii;
        self
    }

    // image-operations
    pub fn image_operations_program(mut self, program: Vec<Instr>) -> ConfigBuilder<'a> {
        self.settings.image_operations_program = program;
        self
    }

    pub fn build(self) -> Config<'a> {
        self.settings
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SelectedLicenses {
    ThisSoftware,
    Dependencies,
    // not optimal for combinations, but is that actually necessary?
    ThisSoftwarePlusDependencies,
}

#[derive(Debug, Clone)]
pub struct FormatEncodingSettings {
    pub jpeg_quality: u8,
    pub pnm_use_ascii_format: bool,
}

/// Strictly speaking not necessary here since the responsible owners will validate the quality as well.
/// However, by doing anyways it we can exit earlier.
pub fn validate_jpeg_quality(quality: u8) -> anyhow::Result<u8> {
    fn within_range(v: u8) -> anyhow::Result<u8> {
        // Upper bound is exclusive with .. syntax.
        // When the `range_contains` feature will be stabilised Range.contains(&v)
        // should be used instead.
        const ALLOWED_RANGE: std::ops::Range<u8> = 1..101;
        if ALLOWED_RANGE.contains(&v) {
            Ok(v)
        } else {
            bail!("JPEG Encoding Settings error: JPEG quality requires a number between 1 and 100 (inclusive).")
        }
    }

    within_range(quality)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use sic_image_engine::engine::Instr;
    use sic_image_engine::ImgOp;

    use super::*;

    #[test]
    fn jpeg_in_quality_range_lower_bound_inside() {
        let value: &str = "1";
        assert!(validate_jpeg_quality(u8::from_str(value).unwrap()).is_ok())
    }

    #[test]
    fn jpeg_in_quality_range_lower_bound_outside() {
        let value: &str = "0";
        assert!(validate_jpeg_quality(u8::from_str(value).unwrap()).is_err())
    }

    #[test]
    fn jpeg_in_quality_range_upper_bound_inside() {
        let value: &str = "100";
        assert!(validate_jpeg_quality(u8::from_str(value).unwrap()).is_ok())
    }

    #[test]
    fn jpeg_in_quality_range_upper_bound_outside() {
        let value: &str = "101";
        assert!(validate_jpeg_quality(u8::from_str(value).unwrap()).is_err())
    }

    #[test]
    fn config_builder_override_defaults() {
        let mut builder = ConfigBuilder::new();
        builder = builder.image_operations_program(vec![Instr::Operation(ImgOp::Blur(1.0))]);
        let config = builder.build();

        assert!(!config.image_operations_program.is_empty());
    }
}
