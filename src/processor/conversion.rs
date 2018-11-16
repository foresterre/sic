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
