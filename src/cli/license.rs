use inflate::inflate_bytes;

use anyhow::anyhow;

use crate::cli::config::SelectedLicenses;

pub struct LicenseTexts<'a> {
    pub(crate) this_software: &'a str,
    pub(crate) dependencies: &'a [u8],
}

impl<'a> LicenseTexts<'a> {
    pub fn new(this_software: &'a str, dependencies: &'a [u8]) -> Self {
        Self {
            this_software,
            dependencies,
        }
    }
}

pub(crate) trait PrintTextFor {
    fn print(&self, texts: &LicenseTexts) -> anyhow::Result<()>;
}

impl PrintTextFor for SelectedLicenses {
    fn print(&self, texts: &LicenseTexts) -> anyhow::Result<()> {
        let print_for_this_software = || {
            println!("sic image tools license:\n\n{}", texts.this_software);

            Ok(())
        };

        let print_for_dependencies = || {
            let inflated = inflate_bytes(texts.dependencies)
                .map_err(|err| anyhow!("Unable to uncompress license text {}", err))?;
            let text = std::str::from_utf8(&inflated).map_err(|err| anyhow!("{}", err))?;

            println!("{}", text);

            Ok(())
        };

        match self {
            SelectedLicenses::ThisSoftware => print_for_this_software(),
            SelectedLicenses::Dependencies => print_for_dependencies(),
            SelectedLicenses::ThisSoftwarePlusDependencies => {
                print_for_this_software().and_then(|_| print_for_dependencies())
            }
        }
    }
}
