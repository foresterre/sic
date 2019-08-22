use crate::app::config::SelectedLicenses;
use std::io::Read;

const LICENSE_SELF: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE",));
const LICENSE_DEPS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/compressed_dep_licenses",));

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
            // based on the size the reader gets as input as of 2019/08/22.
            // size was: 579028, rounded up (power of two): 1048576 (about one MB!)
            const SIZE: usize = 579028;

            let mut reader = snap::Reader::new(LICENSE_DEPS);
            let mut vec: Vec<u8> = Vec::with_capacity(SIZE);
            let _ = reader
                .read_to_end(&mut vec)
                .map_err(|err| format!("Unable to uncompress license text: {}", err))?;

            let text = std::str::from_utf8(&vec).map_err(|err| err.to_string())?;

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
