use crate::app::config::SelectedLicenses;
use inflate::inflate_bytes;

const LICENSE_SELF: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE",));
const LICENSE_DEPS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/compressed_dep_licenses"));

pub(crate) trait PrintTextFor {
    fn print(&self) -> Result<(), String>;
}

impl PrintTextFor for SelectedLicenses {
    fn print(&self) -> Result<(), String> {
        fn print_for_this_software() -> Result<(), String> {
            println!("sic image tools license:\n\n{}\n\n", LICENSE_SELF);

            Ok(())
        }

        fn print_for_dependencies() -> Result<(), String> {
            let inflated = inflate_bytes(LICENSE_DEPS)?;
            let text = std::str::from_utf8(&inflated).map_err(|err| err.to_string())?;

            println!("{}", text);

            Ok(())
        }

        match self {
            SelectedLicenses::ThisSoftware => print_for_this_software(),
            SelectedLicenses::Dependencies => print_for_dependencies(),
            SelectedLicenses::ThisSoftwarePlusDependencies => {
                print_for_this_software().and_then(|_| print_for_dependencies())
            }
        }
    }
}
