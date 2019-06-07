use clap::ArgMatches;
use combostew::config::Config;
use combostew::io::{export, import};
use combostew::operations::engine::{ImageEngine, Program};
use combostew::processor::ProcessWithConfig;

const LICENSE_SELF: &str = include_str!("../LICENSE");
const LICENSE_DEPS: &str = include_str!("../thanks/dependency_licenses.txt");

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

    let format_decider = combostew::processor::encoding_format::EncodingFormatDecider::default();
    export(out, &format_decider, &options)
}

pub fn run_display_licenses(config: &Config) -> Result<(), String> {
    let license_display_processor =
        combostew::processor::license_display::LicenseDisplayProcessor::new(
            LICENSE_SELF,
            LICENSE_DEPS,
        );

    license_display_processor.process(&config);

    Ok(())
}

pub fn run_display_help(config: &Config) -> Result<(), String> {
    let help = crate::sic_processor::help_display::HelpDisplayProcessor::default();
    help.process(config);

    Ok(())
}
