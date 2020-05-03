use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use anyhow::{anyhow, bail, Context};
use sic_core::image;
use sic_image_engine::engine::ImageEngine;
use sic_io::conversion::AutomaticColorTypeAdjustment;
use sic_io::format::{
    DetermineEncodingFormat, EncodingFormatByExtension, EncodingFormatByIdentifier, JPEGQuality,
};

use sic_io::load;
use sic_io::save;

use crate::config::Config;
use crate::license::LicenseTexts;
use crate::license::PrintTextFor;

/// The run function runs the sic application, taking the matches found by Clap.
/// This function is separated from the main() function so that it can be used more easily in test cases.
/// This function consumes the matches provided.
pub fn run(options: &Config) -> anyhow::Result<()> {
    if options.output.is_none() {
        eprintln!(
            "Info: The default output format when using stdout output (the current output mode) is \
             BMP. Other formats can be use by providing --output-format <FORMAT>."
        );
    }

    let mut reader = create_reader(options.input)?;

    let img = load::load_image(
        &mut reader,
        &load::ImportConfig {
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
            None => Ok(image::ImageOutputFormat::Bmp),
        },
    }?;

    save::export(
        buffer,
        &mut export_writer,
        encoding_format,
        save::ExportSettings {
            adjust_color_type: AutomaticColorTypeAdjustment::default(),
        },
    )
    .with_context(|| "Unable to save image.")
}

/// Create a reader which will be used to load the image.
/// The reader can be a file or the stdin.
/// If no file path is provided, the stdin will be assumed.
fn create_reader(path: Option<&str>) -> anyhow::Result<Box<dyn Read>> {
    let reader = if let Some(path) = path {
        sic_io::load::file_reader(path)?
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

pub fn run_display_licenses(config: &Config, texts: &LicenseTexts) -> anyhow::Result<()> {
    config
        .show_license_text_of
        .ok_or_else(|| anyhow!("Unable to determine which license texts should be displayed."))
        .and_then(|license_text| license_text.print(texts))
}
