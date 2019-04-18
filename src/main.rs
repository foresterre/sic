use clap::ArgMatches;
use combostew::config::{Config, ConfigItem};
use combostew::operations::engine::{ImageEngine, Program};
use combostew::processor::encoding_format::EncodingFormatDecider;
use combostew::processor::license_display::LicenseDisplayProcessor;
use combostew::processor::ProcessWithConfig;

use combostew::io::{export, import};
use sic_lib::app_config::script_arg;
use sic_lib::get_tool_name;
use sic_lib::parser;
use sic_lib::sic_processor::help_display::HelpDisplayProcessor;
use sic_lib::app_cli::get_app_config;

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

/// The run function runs the sic application, taking the matches found by Clap.
/// This function is separated from the main() function so that it can be used more easily in test cases.
/// This function consumes the matches provided.
pub fn run(matches: &ArgMatches, program: Program, options: &Config) -> Result<(), String> {
    if options.output.is_none() {
        eprintln!(
            "The default output format is BMP. Use --output-format <FORMAT> to specify \
             a different output format."
        );
    }

    // "input_file" is sic specific.
    let img = import(
        matches
            .value_of("input")
            .or_else(|| matches.value_of("input_file")),
    )?;

    let mut image_engine = ImageEngine::new(img);
    let out = image_engine
        .ignite(program)
        .map_err(|err| err.to_string())?;

    let format_decider = EncodingFormatDecider::default();
    export(out, &format_decider, &options)
}

pub fn run_display_licenses(
    matches: &ArgMatches,
    tool_name: &'static str,
    app_config: Vec<ConfigItem>,
) -> Result<(), String> {
    let options = get_app_config(&matches, tool_name, app_config)?;

    let license_display_processor = LicenseDisplayProcessor::default();

    license_display_processor.process(&options);

    Ok(())
}

fn run_display_help(config: &Config) -> Result<(), String> {
    let help = HelpDisplayProcessor::default();
    help.process(config);

    Ok(())
}
