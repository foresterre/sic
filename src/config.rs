use std::process;

use crate::help::HelpIndex;
use crate::operations;
use crate::operations::operations::apply_operations_on_image;
use crate::operations::Operations;

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
    // Options because they are not required if certain options (such as `--license`) are chosen.
    // This should be handled by Clap, but Option was chosen out of a defensive strategy nonetheless.
    // TODO: input_file, output_file are excluded from Config? Should they be included?
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
    fn process(&self, config: &Config) -> () {
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
    fn process(&self, config: &Config) -> () {
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

    fn parse_script(&self, config: &Config) -> Result<Operations, String> {
        println!("Parsing image operations script.");

        match &config.script {
            Some(it) => operations::parse_script(&it),
            None => Err("Script unavailable.".into()),
        }
    }

    fn apply_operations(&mut self, ops: &Operations) -> Result<(), String> {
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
