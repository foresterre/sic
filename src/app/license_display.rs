use std::process;

use sic_config::{Config, SelectedLicenses};

#[derive(Debug, Default)]
pub struct LicenseDisplayProcessor<'a> {
    self_license: &'a str,
    dependency_licenses: &'a str,
}

impl<'a> LicenseDisplayProcessor<'a> {
    pub fn new(self_license: &'a str, dependency_licenses: &'a str) -> LicenseDisplayProcessor<'a> {
        LicenseDisplayProcessor {
            self_license,
            dependency_licenses,
        }
    }
}

impl<'a> LicenseDisplayProcessor<'a> {
    fn print_licenses(&self, slice: &[SelectedLicenses], tool_name: &str) {
        for item in slice {
            match item {
                SelectedLicenses::ThisSoftware => {
                    println!(
                        "{} image tools license:\n\n{}\n\n",
                        tool_name, &self.self_license
                    );
                }
                SelectedLicenses::Dependencies => println!("{}", &self.dependency_licenses),
            };
        }

        if !slice.is_empty() {
            process::exit(0);
        }
    }

    pub fn process(&self, config: &Config) {
        self.print_licenses(&config.licenses, &config.tool_name);
    }
}
