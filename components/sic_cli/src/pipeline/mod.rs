use std::fs::File;
use std::io::{self, Read, Write};

use anyhow::{anyhow, bail, Context};
use sic_core::image;
use sic_image_engine::engine::ImageEngine;
use sic_io::conversion::AutomaticColorTypeAdjustment;
use sic_io::format::{
    DetermineEncodingFormat, EncodingFormatByExtension, EncodingFormatByIdentifier, JPEGQuality,
};
use sic_io::load;
use sic_io::save;

use crate::config::{Config, InputOutputMode, PathVariant};
use crate::license::LicenseTexts;
use crate::license::PrintTextFor;

pub fn run_with_devices<'c>(
    in_and_output: InputOutputMode,
    config: &'c Config<'c>,
) -> anyhow::Result<()> {
    match in_and_output {
        InputOutputMode::Single { input, output } => {
            if output.is_std_stream() {
                warn_default_std_output_format();
            }

            run(
                || create_reader(&input),
                || create_writer(&output),
                || create_format_decider(&output, config),
                config,
            )
        }
        InputOutputMode::Batch {
            inputs,
            output_root_folder,
        } => {
            for (input, branch) in inputs.path_combinations() {
                let input = &PathVariant::Path(input.to_path_buf());

                let output = output_root_folder.join(branch);
                let output = &PathVariant::Path(output);

                run(
                    || create_reader(&input),
                    || create_writer(&output),
                    || create_format_decider(&output, config),
                    config,
                )?
            }

            Ok(())
        }
    }
}

fn warn_default_std_output_format() {
    eprintln!(
        "warn: The default output format when using stdout output (the current output mode) is \
             BMP. Other formats can be use by providing --output-format <FORMAT>."
    );
}

fn run<R, W, F>(
    supply_reader: R,
    supply_writer: W,
    format_decider: F,
    config: &Config,
) -> anyhow::Result<()>
where
    R: Fn() -> anyhow::Result<Box<dyn Read>>,
    W: Fn() -> anyhow::Result<Box<dyn Write>>,
    F: Fn() -> anyhow::Result<image::ImageOutputFormat>,
{
    let mut reader = supply_reader()?;
    let img = load::load_image(
        &mut reader,
        &load::ImportConfig {
            selected_frame: config.selected_frame,
        },
    )?;

    let mut image_engine = ImageEngine::new(img);
    let buffer = image_engine
        .ignite(&config.image_operations_program)
        .with_context(|| "Unable to apply image operations.")?;

    let mut export_writer = supply_writer()?;
    let encoding_format = format_decider()?;

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
fn create_reader(io_device: &PathVariant) -> anyhow::Result<Box<dyn Read>> {
    match io_device {
        PathVariant::StdStream if atty::is(atty::Stream::Stdin) => bail!(
            "An input image should be given by providing a path using the input argument or \
                 by piping an image to the stdin."
        ),
        PathVariant::StdStream => Ok(sic_io::load::stdin_reader()?),
        PathVariant::Path(path) => Ok(sic_io::load::file_reader(path)?),
    }
}

fn create_writer(io_device: &PathVariant) -> anyhow::Result<Box<dyn Write>> {
    match io_device {
        PathVariant::Path(out) => Ok(Box::new(File::create(out)?)),
        PathVariant::StdStream => Ok(Box::new(io::stdout())),
    }
}

fn create_format_decider(
    io_device: &PathVariant,
    config: &Config,
) -> anyhow::Result<image::ImageOutputFormat> {
    let encoding_format_determiner = DetermineEncodingFormat {
        pnm_sample_encoding: if config.encoding_settings.pnm_use_ascii_format {
            Some(image::pnm::SampleEncoding::Ascii)
        } else {
            Some(image::pnm::SampleEncoding::Binary)
        },
        jpeg_quality: {
            Some(JPEGQuality::try_from(
                config.encoding_settings.jpeg_quality,
            )?)
        },
    };

    let format = match &config.forced_output_format {
        Some(format) => encoding_format_determiner.by_identifier(format)?,
        None => match io_device {
            PathVariant::Path(out) => encoding_format_determiner.by_extension(out)?,
            PathVariant::StdStream => image::ImageOutputFormat::Bmp,
        },
    };

    Ok(format)
}

pub fn run_display_licenses(config: &Config, texts: &LicenseTexts) -> anyhow::Result<()> {
    config
        .show_license_text_of
        .ok_or_else(|| anyhow!("Unable to determine which license texts should be displayed."))
        .and_then(|license_text| license_text.print(texts))
}
