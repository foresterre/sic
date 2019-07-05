use sic_core::combostew::operations::engine::Program;
use sic_lib::app::cli::sic_config;
use sic_lib::app::custom_config::script_arg;
use sic_lib::app::run_mode::{run, run_display_help, run_display_licenses};
use sic_parser;

fn main() -> Result<(), String> {
    let app = sic_lib::app::cli::cli();
    let matches = app.get_matches();

    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");
    let help_display = matches.is_present("user_manual");

    if license_display {
        run_display_licenses(&sic_config(&matches)?)
    } else if help_display {
        run_display_help(&sic_config(&matches)?)
    } else {
        let options = sic_config(&matches)?;

        let ops: Program = if let Some(script) = script_arg(&options.application_specific) {
            sic_parser::parse_script(script)?
        } else {
            Vec::new()
        };

        run(&matches, ops, &options)
    }
}
