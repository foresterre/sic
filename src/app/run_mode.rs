use std::error::Error;
use std::path::Path;

use clap::ArgMatches;
use sic_config::Config;
use sic_core::image;
use sic_image_engine::engine::{ImageEngine, Program};
use sic_io::conversion::AutomaticColorTypeAdjustment;
use sic_io::encoding_format::{
    DetermineEncodingFormat, EncodingFormatByIdentifier, EncodingFormatByMethod, JPEGQuality,
};
use sic_io::{export, import, ExportMethod, ExportSettings};

use sic_user_manual::user_manual_printer::UserManualPrinter;

use crate::app::custom_config::manual_arg;
use crate::app::license_display::LicenseDisplayProcessor;

const LICENSE_SELF: &str = include_str!("../../LICENSE");
const LICENSE_DEPS: &str = include_str!("../../thanks/dependency_licenses.txt");

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
    let buffer = image_engine
        .ignite(program)
        .map_err(|err| err.to_string())?;

    let export_method =
        determine_export_method(options.output.as_ref()).map_err(|err| err.to_string())?;

    let encoding_format_determiner = DetermineEncodingFormat {
        pnm_sample_encoding: if options.encoding_settings.pnm_settings.ascii {
            Some(image::pnm::SampleEncoding::Ascii)
        } else {
            Some(image::pnm::SampleEncoding::Binary)
        },
        jpeg_quality: {
            let quality = JPEGQuality::try_from(options.encoding_settings.jpeg_settings.quality)
                .map_err(|err| err.to_string());

            Some(quality?)
        },
    };

    let encoding_format = match &options.forced_output_format {
        Some(format) => encoding_format_determiner.by_identifier(format.as_str()),
        None => encoding_format_determiner.by_method(&export_method),
    }
    .map_err(|err| err.to_string())?;

    export(
        buffer,
        export_method,
        encoding_format,
        ExportSettings {
            adjust_color_type: AutomaticColorTypeAdjustment::default(),
        },
    )
}

fn determine_export_method<P: AsRef<Path>>(
    output_path: Option<P>,
) -> Result<ExportMethod<P>, Box<dyn Error>> {
    let method = if output_path.is_none() {
        ExportMethod::StdoutBytes
    } else {
        let path = output_path
            .ok_or_else(|| "The export method 'file' requires an output file path.".to_string());
        ExportMethod::File(path?)
    };

    Ok(method)
}

pub fn run_display_licenses(config: &Config) -> Result<(), String> {
    let license_display_processor = LicenseDisplayProcessor::new(LICENSE_SELF, LICENSE_DEPS);

    license_display_processor.process(&config);

    Ok(())
}

pub fn run_display_help(config: &Config) -> Result<(), String> {
    let help = UserManualPrinter::default();
    let page = manual_arg(&config.application_specific);
    help.show(page).map(|_| ()).map_err(|err| err.to_string())
}
