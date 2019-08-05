use sic_image_engine::engine::Program;
use sic_lib::app::cli::build_app_config;
use sic_lib::app::run_mode::{run, run_display_help, run_display_licenses};
use sic_parser;

fn main() -> Result<(), String> {
    let app = sic_lib::app::cli::cli();
    let matches = app.get_matches();

    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");
    let help_display = matches.is_present("user_manual");

    let configuration = build_app_config(&matches)?;

    if license_display {
        run_display_licenses(&configuration)
    } else if help_display {
        run_display_help(&configuration)
    } else {
        let ops: Program = if let Some(script) = configuration.image_operations_script {
            sic_parser::parse_script(script)?
        } else {
            Vec::new()
        };

        run(&matches, ops, &configuration)
    }
}
