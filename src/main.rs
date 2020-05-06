use sic_cli::cli::build_app_config;
use sic_cli::config::InputOutputMode;
use sic_cli::license::LicenseTexts;
use sic_cli::pipeline::{run_display_licenses, run_with_devices};

const LICENSE_SELF: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE",));
const LICENSE_DEPS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/compressed_dep_licenses"));

const ABOUT: &str = include_str!("../resources/help-pages/about.txt");
const HELP_OPERATIONS_AVAILABLE: &str =
    include_str!("../resources/help-pages/image_operations.txt");

fn main() -> anyhow::Result<()> {
    let app = sic_cli::cli::cli(ABOUT, HELP_OPERATIONS_AVAILABLE);
    let matches = app.get_matches();

    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    let configuration = build_app_config(&matches)?;

    if license_display {
        run_display_licenses(
            &configuration,
            &LicenseTexts::new(LICENSE_SELF, LICENSE_DEPS),
        )
    } else {
        let io_device = InputOutputMode::try_from_matches(&matches)?;
        run_with_devices(io_device, &configuration)
    }
}
