use crate::config::Config;
use crate::operations;
use crate::operations::transformations::apply_operations_on_image;
use crate::operations::Operation;
use crate::processor::ProcessMutWithConfig;

pub(crate) struct ImageOperationsProcessor<'a> {
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
