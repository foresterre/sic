use std::error::Error;
use std::path::Path;

use clap::ArgMatches;
use sic_core::image;
use sic_image_engine::engine::ImageEngine;
use sic_io::conversion::AutomaticColorTypeAdjustment;
use sic_io::encoding_format::{
    DetermineEncodingFormat, EncodingFormatByIdentifier, EncodingFormatByMethod, JPEGQuality,
};
use sic_io::{export, import, ExportMethod, ExportSettings};

use sic_user_manual::user_manual_printer::UserManualPrinter;

use crate::app::config::Config;
use crate::app::license_display::PrintTextFor;

/// The run function runs the sic application, taking the matches found by Clap.
/// This function is separated from the main() function so that it can be used more easily in test cases.
/// This function consumes the matches provided.
pub fn run(matches: &ArgMatches, options: &Config) -> Result<(), String> {
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
        .ignite(&options.image_operations_program)
        .map_err(|err| err.to_string())?;

    let export_method =
        determine_export_method(options.output.as_ref()).map_err(|err| err.to_string())?;

    let encoding_format_determiner = DetermineEncodingFormat {
        pnm_sample_encoding: if options.encoding_settings.pnm_use_ascii_format {
            Some(image::pnm::SampleEncoding::Ascii)
        } else {
            Some(image::pnm::SampleEncoding::Binary)
        },
        jpeg_quality: {
            let quality = JPEGQuality::try_from(options.encoding_settings.jpeg_quality)
                .map_err(|err| err.to_string());

            Some(quality?)
        },
    };

    let encoding_format = match &options.forced_output_format {
        Some(format) => encoding_format_determiner.by_identifier(format),
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
    config
        .show_license_text_of
        .ok_or_else(|| "Unable to display license texts".to_string())
        .and_then(|license_text| license_text.print())
}

pub fn run_display_help(config: &Config) -> Result<(), String> {
    let help = UserManualPrinter::default();
    let page = config.image_operations_manual_topic;
    help.show(page).map(|_| ()).map_err(|err| err.to_string())
}
