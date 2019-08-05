use std::process;

use crate::app::config::{Config, SelectedLicenses};

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
    fn print_licenses(&self, requested_texts: SelectedLicenses, tool_name: &str) {
        let print_for_this_software = || {
            println!(
                "{} image tools license:\n\n{}\n\n",
                tool_name, &self.self_license
            );
        };

        let print_for_dependencies = || println!("{}", &self.dependency_licenses);

        match requested_texts {
            SelectedLicenses::ThisSoftware => print_for_this_software(),
            SelectedLicenses::Dependencies => print_for_dependencies(),
            SelectedLicenses::ThisSoftwarePlusDependencies => {
                print_for_this_software();
                print_for_dependencies();
            }
        };
    }

    pub fn process(&self, config: &Config) {
        if let Some(selection) = config.show_license_text_of {
            self.print_licenses(selection, &config.tool_name);
        } else {
            process::exit(0)
        }
    }
}
