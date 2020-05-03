use sic_cli::cli::build_app_config;
use sic_cli::license::LicenseTexts;
use sic_cli::pipeline::{run, run_display_licenses};

const LICENSE_SELF: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE",));
const LICENSE_DEPS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/compressed_dep_licenses"));

fn main() -> anyhow::Result<()> {
    let app = sic_cli::cli::cli();
    let matches = app.get_matches();

    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    let configuration = build_app_config(&matches)?;

    if license_display {
        run_display_licenses(
            &configuration,
            &LicenseTexts::new(LICENSE_SELF, LICENSE_DEPS),
        )
    } else {
        run(&configuration)
    }
}
