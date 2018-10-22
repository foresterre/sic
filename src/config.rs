use std::process;

use crate::help::HelpIndex;

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
    pub input_file: Option<String>,
    pub output_file: Option<String>,
}

pub trait ProcessWithConfig<T> {
    fn act(&self, config: &Config) -> T;
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
    fn act(&self, config: &Config) -> () {
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
    fn act(&self, config: &Config) -> () {
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
        }

        process::exit(0);
    }
}
