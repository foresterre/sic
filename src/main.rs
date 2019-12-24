use anyhow::anyhow;
use sic_lib::app::cli::build_app_config;
use sic_lib::app::procedure::{run, run_display_licenses};

fn main() -> anyhow::Result<()> {
    let app = sic_lib::app::cli::cli();
    let matches = app.get_matches();

    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");

    let configuration =
        build_app_config(&matches).map_err(|_err| anyhow!("TODO . build app config"))?;

    if license_display {
        run_display_licenses(&configuration)
            .map_err(|_err| anyhow!("TODO . run display licenses error"))
    } else {
        run(&matches, &configuration).map_err(|_err| anyhow!("TODO . run error"))
    }
}
