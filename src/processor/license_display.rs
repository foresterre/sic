use std::process;

use crate::config::{Config, SelectedLicenses};
use crate::processor::ProcessWithConfig;

const SIC_LICENSE: &str = include_str!("../../LICENSE");
const DEP_LICENSES: &str = include_str!("../../LICENSES_DEPENDENCIES");

#[derive(Debug)]
pub(crate) struct LicenseDisplayProcessor;

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
