#[derive(Debug, Clone)]
pub struct Config<'a> {
    pub tool_name: &'static str,

    // Display license of this software or its dependencies.
    pub show_license_text_of: Option<SelectedLicenses>,

    // Format to which an image will be converted (enforced).
    pub forced_output_format: Option<&'a str>,

    // --disable-automatic-color-type-adjustment
    pub disable_automatic_color_type_adjustment: bool,

    pub encoding_settings: FormatEncodingSettings,

    // output path
    pub output: Option<&'a str>,

    /// If a user wants to perform image operations on input image, they will need to provide
    /// the image operation commands as a str.
    pub image_operations_script: Option<&'a str>,

    /// If a user wants to be informed about a specific image operation, they should provide
    /// the keyword of the operation which they want to be informed about.
    pub image_operations_manual_topic: Option<&'a str>,
}

impl Default for Config<'_> {
    fn default() -> Self {
        Config {
            /// If using default, requires the `CARGO_PKG_NAME` to be set.
            tool_name: env!("CARGO_PKG_NAME"),

            /// Defaults to no displayed license text.
            show_license_text_of: None,

            /// Defaults to not forcing a specific image output format.
            forced_output_format: None,

            /// Defaults to using automatic color type adjustment where appropriate.
            disable_automatic_color_type_adjustment: false,

            /// Default format encoding settings.
            encoding_settings: FormatEncodingSettings {
                /// Default JPEG quality is set to 80.
                jpeg_quality: 80,

                /// Default encoding type of PNM files (excluding PAM) is set to binary.
                pnm_use_ascii_format: false,
            },

            /// Default output path is None. The program may require an output to be set
            /// for most of its program behaviour.
            output: None,

            /// Defaults to no provided image operations script.
            image_operations_script: None,

            /// Default to no provided topic for the image operations manual.
            image_operations_manual_topic: None,
        }
    }
}

/// Builder for [crate::app::config::Config]. Should be used with the Default implementation
/// of [crate::app::config::Config].
/// If the default trait is not used with this builder, some settings may be inaccessible.
/// For example, `output_path` can be set to some value, but not unset.
///
/// Builder is consuming.
#[derive(Debug, Clone, Default)]
pub struct ConfigBuilder<'a> {
    settings: Config<'a>,
}

impl<'a> ConfigBuilder<'a> {
    pub fn new() -> Self {
        ConfigBuilder::default()
    }

    pub fn show_license_text_of(mut self, selection: SelectedLicenses) -> ConfigBuilder<'a> {
        self.settings.show_license_text_of = Some(selection);
        self
    }

    pub fn forced_output_format(mut self, format: &'a str) -> ConfigBuilder<'a> {
        self.settings.forced_output_format = Some(format);
        self
    }

    pub fn disable_automatic_color_type_adjustment(mut self, toggle: bool) -> ConfigBuilder<'a> {
        self.settings.disable_automatic_color_type_adjustment = toggle;
        self
    }

    pub fn jpeg_quality(mut self, quality: u8) -> ConfigBuilder<'a> {
        self.settings.encoding_settings.jpeg_quality = quality;
        self
    }

    pub fn pnm_format_type(mut self, use_ascii: bool) -> ConfigBuilder<'a> {
        self.settings.encoding_settings.pnm_use_ascii_format = use_ascii;
        self
    }

    pub fn output_path(mut self, path: &'a str) -> ConfigBuilder<'a> {
        self.settings.output = Some(path);
        self
    }

    pub fn image_operations_script(mut self, script: &'a str) -> ConfigBuilder<'a> {
        self.settings.image_operations_script = Some(script);
        self
    }

    pub fn image_operations_manual_keyword(mut self, topic: &'a str) -> ConfigBuilder<'a> {
        self.settings.image_operations_manual_topic = Some(topic);
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
        builder = builder.image_operations_script("my");
        let config = builder.build();

        assert_eq!(config.image_operations_script.unwrap(), "my");
        assert_eq!(config.output.unwrap(), "lalala");
    }
}
