use sic_image_engine::engine::Instruction;
use sic_io::load::FrameIndex;

#[derive(Debug)]
pub struct Config<'a> {
    pub tool_name: &'static str,

    // organisational
    /// Display license of this software or its dependencies.
    pub show_license_text_of: Option<SelectedLicenses>,

    // io(output)
    /// The image output path.
    pub output: Option<&'a str>,

    // config(in)
    pub selected_frame: FrameIndex,

    // config(out)
    /// Disable color type adjustments on save.
    pub disable_automatic_color_type_adjustment: bool,

    // config(out)
    /// Format to which an image will be converted (enforced).
    pub forced_output_format: Option<&'a str>,

    // config(out)
    /// Encoding settings for specific output formats.
    pub encoding_settings: FormatEncodingSettings,

    // image-operations
    /// If a user wants to perform image operations on input image, they will need to provide
    /// the image operation commands.
    /// THe value set here should be presented as a [sic_image_engine::engine::Program].
    /// If no program is present, an empty vec should be provided.
    pub image_operations_program: Vec<Instruction>,
}

impl Default for Config<'_> {
    fn default() -> Self {
        Config {
            /// If using default, requires the `CARGO_PKG_NAME` to be set.
            tool_name: env!("CARGO_PKG_NAME"),

            /// Defaults to no displayed license text.
            show_license_text_of: None,

            /// Default output path is None. The program may require an output to be set
            /// for most of its program behaviour.
            output: None,

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

/// Builder for [crate::app::config::Config]. Should be used with the Default implementation
/// of [crate::app::config::Config].
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

    // config(out)
    pub fn output_path(mut self, path: &'a str) -> ConfigBuilder<'a> {
        self.settings.output = Some(path);
        self
    }

    // image-operations
    pub fn image_operations_program(mut self, program: Vec<Instruction>) -> ConfigBuilder<'a> {
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
pub fn validate_jpeg_quality(quality: u8) -> Result<u8, String> {
    fn within_range(v: u8) -> Result<u8, String> {
        // Upper bound is exclusive with .. syntax.
        // When the `range_contains` feature will be stabilised Range.contains(&v)
        // should be used instead.
        const ALLOWED_RANGE: std::ops::Range<u8> = 1..101;
        if ALLOWED_RANGE.contains(&v) {
            Ok(v)
        } else {
            Err("JPEG Encoding Settings error: JPEG quality requires a number between 1 and 100 (inclusive).".to_string())
        }
    }

    within_range(quality)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sic_image_engine::engine::Instruction;
    use sic_image_engine::ImgOp;
    use std::str::FromStr;

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
        builder = builder.output_path("lalala");
        builder = builder.image_operations_program(vec![Instruction::Operation(ImgOp::Blur(1.0))]);
        let config = builder.build();

        assert!(!config.image_operations_program.is_empty());
        assert_eq!(config.output.unwrap(), "lalala");
    }
}
