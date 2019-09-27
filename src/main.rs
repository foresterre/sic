use sic_lib::app::cli::build_app_config;
use sic_lib::app::procedure::{run, run_display_licenses};

fn main() -> Result<(), String> {
    let app = sic_lib::app::cli::cli();
    let matches = app.get_matches();

    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    let configuration = build_app_config(&matches)?;

    if license_display {
        run_display_licenses(&configuration)
    } else {
        run(&matches, &configuration)
    }
}
