use clap::ArgMatches;
use combostew::config::{Config, ConfigItem};

use combostew::operations::engine::Program;
use sic_lib::app_cli::get_app_config;
use sic_lib::app_config::script_arg;
use sic_lib::app_run::{run, run_display_help, run_display_licenses};
use sic_lib::get_tool_name;
use sic_lib::parser;

fn main() -> Result<(), String> {
    let app = sic_lib::app_cli::sic_app();
    let matches = app.get_matches();

    let custom_app_config = vec![
        ConfigItem::OptionStringItem(matches.value_of("script").map(String::from)),
        ConfigItem::OptionStringItem(matches.value_of("user_manual").map(String::from)),
    ];
    let license_display = matches.is_present("license") || matches.is_present("dep_licenses");
    let help_display = matches.is_present("user_manual");

    if license_display {
        run_display_licenses(&matches, get_tool_name(), custom_app_config)
    } else if help_display {
        run_display_help(&sic_config(&matches, custom_app_config)?)
    } else {
        let options = sic_config(&matches, custom_app_config)?;

        let ops: Program = if let Some(script) = script_arg(&options.application_specific) {
            parser::parse_script(script)?
        } else {
            Vec::new()
        };

        run(&matches, ops, &options)
    }
}

fn sic_config(
    matches: &ArgMatches,
    app_specific_config: Vec<ConfigItem>,
) -> Result<Config, String> {
    get_app_config(matches, get_tool_name(), app_specific_config)
}
