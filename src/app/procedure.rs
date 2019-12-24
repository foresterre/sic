use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use anyhow::{anyhow, bail, Context};
use clap::ArgMatches;
use sic_core::image;
use sic_image_engine::engine::ImageEngine;
use sic_io::conversion::AutomaticColorTypeAdjustment;
use sic_io::format::{
    DetermineEncodingFormat, EncodingFormatByExtension, EncodingFormatByIdentifier, JPEGQuality,
};
use sic_io::load::{load_image, ImportConfig};
use sic_io::save::{export, ExportSettings};

use crate::app::cli::arg_names::{ARG_INPUT, ARG_INPUT_FILE};
use crate::app::config::Config;
use crate::app::license::PrintTextFor;

/// The run function runs the sic application, taking the matches found by Clap.
/// This function is separated from the main() function so that it can be used more easily in test cases.
/// This function consumes the matches provided.
pub fn run(matches: &ArgMatches, options: &Config) -> anyhow::Result<()> {
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
        .with_context(|| "Unable to apply image operations.")?;

    let mut export_writer = mk_export_writer(options.output.as_ref())?;

    let encoding_format_determiner = DetermineEncodingFormat {
        pnm_sample_encoding: if options.encoding_settings.pnm_use_ascii_format {
            Some(image::pnm::SampleEncoding::Ascii)
        } else {
            Some(image::pnm::SampleEncoding::Binary)
        },
        jpeg_quality: {
            Some(JPEGQuality::try_from(
                options.encoding_settings.jpeg_quality,
            )?)
        },
    };

    let encoding_format = match &options.forced_output_format {
        Some(format) => encoding_format_determiner.by_identifier(format),
        None => match options.output {
            Some(out) => encoding_format_determiner.by_extension(out),
            None => Ok(image::ImageOutputFormat::BMP),
        },
    }?;

    export(
        buffer,
        &mut export_writer,
        encoding_format,
        ExportSettings {
            adjust_color_type: AutomaticColorTypeAdjustment::default(),
        },
    )
    .with_context(|| "Unable to save image.")
}

/// Create a reader which will be used to load the image.
/// The reader can be a file or the stdin.
/// If no file path is provided, the stdin will be assumed.
fn mk_reader(matches: &ArgMatches) -> anyhow::Result<Box<dyn Read>> {
    fn with_file_reader(matches: &ArgMatches, value_of: &str) -> anyhow::Result<Box<dyn Read>> {
        sic_io::load::file_reader(
            matches
                .value_of(value_of)
                .with_context(|| format!("No such value: {}", value_of))?,
        )
        .with_context(|| "No matching file reader could be found.")
    }

    let reader = if matches.is_present(ARG_INPUT) {
        with_file_reader(matches, ARG_INPUT)?
    } else if matches.is_present(ARG_INPUT_FILE) {
        with_file_reader(matches, ARG_INPUT_FILE)?
    } else {
        if atty::is(atty::Stream::Stdin) {
            bail!(
                "An input image should be given by providing a path using the input argument or \
                 by piping an image to the stdin."
            )
        }
        sic_io::load::stdin_reader()?
    };

    Ok(reader)
}

/// Make an export writer.
/// The choices are the stdout or a file.
fn mk_export_writer<P: AsRef<Path>>(output_path: Option<P>) -> anyhow::Result<Box<dyn Write>> {
    match output_path {
        Some(out) => Ok(Box::new(File::create(out)?)),
        None => Ok(Box::new(io::stdout())),
    }
}

pub fn run_display_licenses(config: &Config) -> anyhow::Result<()> {
    config
        .show_license_text_of
        .ok_or_else(|| anyhow!("Unable to determine which license texts should be displayed."))
        .and_then(|license_text| license_text.print())
}
