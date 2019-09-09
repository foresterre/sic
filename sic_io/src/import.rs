use std::env::args;
use std::io::{stdin, Read};
use std::path::Path;

use sic_core::image;

pub fn import<P: AsRef<Path>>(
    maybe_path: Option<P>,
    import_settings: ImportSettings,
) -> Result<image::DynamicImage, String> {
    maybe_path.map_or_else(
        || import_from_input_stream_sync(&import_settings),
        |path| import_from_file_sync(path, &import_settings),
    )
}

pub struct ImportSettings {
    pub gif_frame: GIFFrameSelection,
}

pub enum GIFFrameSelection {
    First,
    Last,
    Nth(usize),
}

fn import_from_input_stream_sync(
    import_settings: &ImportSettings,
) -> Result<image::DynamicImage, String> {
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

    // TODO: here gif frame selection if gif

    panic!("inprogress-todo");
    image::load_from_memory(&buffer).map_err(|err| err.to_string())
}

fn import_from_file_sync<P: AsRef<Path>>(
    path: P,
    import_settings: &ImportSettings,
) -> Result<image::DynamicImage, String> {
    load_image(path.as_ref(), import_settings)
}

fn load_image(
    path: &Path,
    import_settings: &ImportSettings,
) -> Result<image::DynamicImage, String> {
    let extension = path.extension().and_then(|v| v.to_str());

    match extension {
        Some(ext) if ext == "gif" => {
            // TODO
            // * probably ImportSettings { gif-frame: First |  Last | Nth(i) }
            panic!("inprogress-todo");
        }
        _ => image::open(path).map_err(|err| err.to_string()),
    }
}

fn gif_select_frame<R: Read>(image_source: R) -> Result<image::DynamicImage, String> {
    // TODO:
    // - gif frame selection requires a custom created decoder
    // - thus we should first decide on which format we have
    //    - easy with extension,
    //    - slightly harder and a bit guessy (using magic number + guessing)
    // - important though, before we call image::open or image::load_from_memory
    // - since we cant use the above (which contains the guessing part, we'll need to do it ourselves)
    // +++++
    // - Additionally, see where we can use R: Read trait
    // +++++
    // - And see if we can clean up load from buffer / open a bit
    // +++++
    // - Finally use the fn: decoder_to_image to create a DynamicImage using our custom created gif decoder
    // +++++
    // oh we'll also need to create some cli stuff like --gif-select-frame { first | last | {n: usize}}

    Err("TODO".to_string())
}

fn starts_with_gif_magic_number(buffer: &[u8]) {
    if buffer.starts_with(b"GIF87a") || buffer.starts_with(b"GIF89a") {
        // it is likely to be a gif, but upon loading we'll see if we can load it
    }
}
