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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{
        Config, FormatEncodingSettings, JPEGEncodingSettings, PNMEncodingSettings,
    };
    use crate::processor::mod_test_includes::*;

    const INPUT: &str = "rainbow_8x6.bmp";
    const OUTPUT: &str = "conversion_rainbow_8x6.png";

    fn setup_dummy_config(output: &str) -> Config {
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
                setup_output_path(output)
                    .to_str()
                    .expect("Path given is no good!"),
            ),
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
        use std::io::Read;
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
}
