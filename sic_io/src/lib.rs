use std::env::args;
use std::io::stdin;
use std::io::Read;
use std::path::Path;

use sic_core::image;

use crate::conversion::{AutomaticColorTypeAdjustment, ConversionWriter};

pub mod conversion;
pub mod encoding_format;

pub fn import<P: AsRef<Path>>(maybe_path: Option<P>) -> Result<image::DynamicImage, String> {
    maybe_path.map_or_else(import_from_input_stream_sync, import_from_file_sync)
}

// TODO{foresterre}: Currently the method we use to read from the input stream is full blocking.
//  That is, as long as no terminating signal has been received, the stream will wait for input.
//  Perhaps we would like to read the stdin with tokio-io (async).
//  Then we can display an error and the help page instead if the 'Complete' event has been received,
//  but the buffer is empty.
fn import_from_input_stream_sync() -> Result<image::DynamicImage, String> {
    if cfg!(windows) {
        let program_name = args().nth(0).unwrap_or_default();

        eprintln!(
            "Warning: You are using stdin as input method on the \
             Windows platform. If you use PowerShell and the program errors with 'Unsupported image format', \
             PowerShell assumed the binary data was text. You can either use input files instead \
             (run {} with '--help' for more information), \
             or run the program from 'cmd.exe' (for example: \
             'type <INPUT_FILE> | {} -o <OUTPUT_FILE> <ARGS>').\n",
            program_name, program_name
        );
    }

    // We don't known the input size yet, so we allocate.
    let mut buffer = Vec::new();

    // Uses stderr because stdout is used to redirect the output image if no file is defined.
    eprintln!(
        "If stdin is empty, the programs waits for input until a termination \
         signal has been received (usually you can send it by pressing Ctrl+D in your terminal)."
    );

    stdin().lock().read_to_end(&mut buffer).map_err(|err| {
        format!(
            "Unable to read from the stdin. Message: {}",
            err.to_string()
        )
    })?;

    if buffer.is_empty() {
        return Err(
            "Stdin was empty. To display the help page, use the `--help` flag.".to_string(),
        );
    }

    // Uses stderr because stdout is used to redirect the output image if no file is defined.
    eprintln!("Read {} bytes. Continuing.", buffer.len());

    image::load_from_memory(&buffer).map_err(|err| err.to_string())
}

fn import_from_file_sync<P: AsRef<Path>>(path: P) -> Result<image::DynamicImage, String> {
    image::open(path).map_err(|err| err.to_string())
}

pub struct EmptyPath;

impl AsRef<Path> for EmptyPath {
    fn as_ref(&self) -> &Path {
        Path::new("")
    }
}

#[derive(Debug)]
pub enum ExportMethod<P: AsRef<Path>> {
    File(P),
    StdoutBytes,
}

// enum variants on type aliases are currently experimental, so we use a function here instead.
pub fn use_stdout_bytes_as_export_method() -> ExportMethod<EmptyPath> {
    ExportMethod::StdoutBytes
}

#[derive(Debug)]
pub struct ExportSettings {
    pub adjust_color_type: AutomaticColorTypeAdjustment,
}

pub fn export<P: AsRef<Path>>(
    image: &image::DynamicImage,
    method: ExportMethod<P>,
    format: image::ImageOutputFormat,
    export_settings: ExportSettings,
) -> Result<(), String> {
    let writer = ConversionWriter::new(image);
    writer.write(method, format, export_settings.adjust_color_type)
}
