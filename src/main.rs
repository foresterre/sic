#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use std::fs::File;
use std::path::Path;

use clap::{App, Arg};

#[cfg(test)]
mod tests;

fn main() {
    let matches = App::new("Simple Image Converter")
        .version("0.4.0")
        .author("foresterre <garm@ilumeo.com>")
        .about("Converts an image from one format to another.\n\n\
                Supported input formats are described BMP, GIF, ICO, JPEG, PNG, PPM (limitations may apply). \n\n\
                The image conversion is actually done by the awesome 'image' crate [1]. \n\
                Sic itself is a small command line frontend which supports a small part of the \
                conversion operations supported by the 'image' library. \n\n\
                [1] image crate by PistonDevelopers: https://github.com/PistonDevelopers/image \n\n\
                ")
        .arg(Arg::with_name("forced_output_format")
            .short("f")
            .long("force-format")
            .value_name("FORMAT")
            .help("Output formats supported: JPEG, PNG, GIF, ICO, PPM")
            .takes_value(true))
        .arg(Arg::with_name("input_file")
            .help("Sets the input file")
            .value_name("INPUT_FILE")
            .required(true)
            .index(1))
        .arg(Arg::with_name("output_file")
            .help("Sets the output file")
            .value_name("OUTPUT_FILE")
            .required(true)
            .index(2))
        .get_matches();

    // Can be unwrap because these values are required arguments.
    let input = matches.value_of("input_file").unwrap();
    let output = matches.value_of("output_file").unwrap();
    println!("Provided input file path: {}", input);
    println!("Provided output file path: {}", output);

    let image_buffer: Result<image::DynamicImage, String> =
        image::open(&Path::new(input)).map_err(|err| err.to_string());

    // encode
    let forced_format = matches.value_of("forced_output_format");
    let encode_buffer: Result<(), String> =
        image_buffer.map_err(|err| err.to_string()).and_then(|img| {
            forced_format.map_or_else(
                || convert_image_unforced(&img, output),
                |format| convert_image_forced(&img, output, format),
            )
        });

    match encode_buffer {
        Ok(_) => println!("Conversion complete."),
        Err(err) => println!("Conversion ended with an Error: {}", err),
    }
}

/// Determines the appropriate ImageOutputFormat based on a &str.
fn image_format_from_str(format: &str) -> Result<image::ImageOutputFormat, String> {
    let format_in_lower_case: &str = &*format.to_string().to_lowercase();

    match format_in_lower_case {
        "bmp" => Ok(image::ImageOutputFormat::BMP),
        "gif" => Ok(image::ImageOutputFormat::GIF),
        "ico" => Ok(image::ImageOutputFormat::ICO),
        "jpeg" | "jpg" => Ok(image::ImageOutputFormat::JPEG(80)),
        "png" => Ok(image::ImageOutputFormat::PNG),
        "ppm" => Ok(image::ImageOutputFormat::PNM(
            image::pnm::PNMSubtype::Pixmap(image::pnm::SampleEncoding::Binary),
        )),
        _ => Err("Image format unsupported.".to_string()),
    }
}

/// Converts an image (`input`) to a certain `format` regardless of the extension of the `output` file path.
fn convert_image_forced(
    img: &image::DynamicImage,
    output: &str,
    format: &str,
) -> Result<(), String> {
    image_format_from_str(format)
        .map_err(|err| err.to_string())
        .and_then(|image_format| {
            let mut out = File::create(&Path::new(output)).map_err(|err| err.to_string())?;

            img.write_to(&mut out, image_format)
                .map_err(|err| err.to_string())
        })
}

/// Converts an image (`input`) to a certain `format` based on the extension of the `output` file path.
fn convert_image_unforced(img: &image::DynamicImage, output: &str) -> Result<(), String> {
    img.save(output).map_err(|err| err.to_string())
}
