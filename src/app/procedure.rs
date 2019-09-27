use std::error::Error;
use std::io::Read;
use std::path::Path;

use clap::ArgMatches;
use sic_core::image;
use sic_image_engine::engine::ImageEngine;
use sic_io::conversion::AutomaticColorTypeAdjustment;
use sic_io::format::{
    DetermineEncodingFormat, EncodingFormatByIdentifier, EncodingFormatByMethod, JPEGQuality,
};
use sic_io::load::{load_image, ImportConfig};
use sic_io::save::{export, ExportMethod, ExportSettings};

use crate::app::cli::arg_names::{ARG_INPUT, ARG_INPUT_FILE};
use crate::app::config::Config;
use crate::app::license::PrintTextFor;

const NO_INPUT_PATH_MSG: &str = "Input path was expected but could not be found.";

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

    let mut reader = mk_reader(matches)?;

    let img = load_image(
        &mut reader,
        &ImportConfig {
            selected_frame: options.selected_frame,
        },
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

/// Create a reader which will be used to load the image.
/// The reader can be a file or the stdin.
/// If no file path is provided, the stdin will be assumed.
fn mk_reader(matches: &ArgMatches) -> Result<Box<dyn Read>, String> {
    fn with_file_reader(matches: &ArgMatches, value_of: &str) -> Result<Box<dyn Read>, String> {
        Ok(sic_io::load::file_reader(
            matches
                .value_of(value_of)
                .ok_or_else(|| NO_INPUT_PATH_MSG.to_string())?,
        )?)
    };

    let reader = if matches.is_present(ARG_INPUT) {
        with_file_reader(matches, ARG_INPUT)?
    } else if matches.is_present(ARG_INPUT_FILE) {
        with_file_reader(matches, ARG_INPUT_FILE)?
    } else {
        if atty::is(atty::Stream::Stdin) {
            return Err(
                "An input image should be given by providing a path using the input argument or by \
                piping an image to the stdin.".to_string(),
            );
        }
        sic_io::load::stdin_reader()?
    };

    Ok(reader)
}

/// Determines what export method should be used.
/// The choices are the stdout or a file.
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
