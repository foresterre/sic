use clap::{App, Arg, ArgMatches};
use combostew::config::{Config, ConfigItem};
use combostew::operations::Operation;
use combostew::{get_app_skeleton, get_default_config, run, run_display_licenses};
use sic_lib::get_tool_name;

use combostew::processor::ProcessWithConfig;
use sic_lib::app_config::script_arg;
use sic_lib::parser;
use sic_lib::sic_processor::help_display::HelpDisplayProcessor;

const HELP_OPERATIONS_AVAILABLE: &str = include_str!("../docs/cli_help_script.txt");

fn main() -> Result<(), String> {
    let app = sic_app();
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

        let mut ops: Vec<Operation> =
            if let Some(script) = script_arg(&options.application_specific) {
                parser::parse_script(script)?
            } else {
                Vec::new()
            };

        // TODO: work in progress
        run(&matches, &mut ops, &options)
    }
}

fn sic_app() -> App<'static, 'static> {
    get_app_skeleton(get_tool_name())
        .arg(Arg::with_name("user_manual")
            .long("user-manual")
            .short("H")
            .help("Displays help text for different topics such as each supported script operation. Run `sic -H index` to display a list of available topics.")
            .value_name("TOPIC")
            .takes_value(true))
        .arg(Arg::with_name("script")
            .long("apply-operations")
            .short("A")
            .help(HELP_OPERATIONS_AVAILABLE)
            .value_name("OPERATIONS")
            .takes_value(true))
        .arg(Arg::with_name("input_file")
            .help("Sets the input file")
            .value_name("INPUT_FILE")
            .required_unless_one(&["input", "license", "dep_licenses", "user_manual"])
            .index(1))
        .arg(Arg::with_name("output_file")
            .help("Sets the desired output file")
            .value_name("OUTPUT_FILE")
            .required_unless_one(&["output", "license", "dep_licenses", "user_manual"])
            .index(2))
}

fn sic_config(
    matches: &ArgMatches,
    app_specific_config: Vec<ConfigItem>,
) -> Result<Config, String> {
    get_default_config(matches, get_tool_name(), app_specific_config)
}

fn run_display_help(config: &Config) -> Result<(), String> {
    let help = HelpDisplayProcessor::new();
    Ok(help.process(config))
}
