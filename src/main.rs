#![feature(match_default_bindings)]

extern crate clap;
extern crate image;

use std::fs::File;
use std::path::Path;

use clap::{App, Arg};

fn which_image_format(format: &str) -> image::ImageFormat {

    let format_in_uppercase: &str = &* format.to_string().to_uppercase();

    match format_in_uppercase {
        "JPEG" | "JPG" | "JPEG_BASELINE" => image::JPEG,
        "PNG" => image::PNG,
        "GIF" => image::GIF,
        "ICO" => image::ICO,
        "PPM" => image::PPM,
        _ => panic!("The requested image format is not available as an option.")
    }
}

fn main() {
    let matches = App::new("Simple Image Converter")
        .version("0.1.0")
        .author("foresterre <garm@ilumeo.com>")
        .about("Converts an image from one format to another.\n\n\
                Supported input formats are: PNG, JPEG (baseline and progressive), \
                GIF, BMP, ICO, TIFF (baseline without fax) + LZW + PackBits, \
                Webp Lossy (Luma channel only) and PPM. \n\
                The image conversion is actually done by the awesome 'image' crate. \n\
                Sic is thus only a small command line frontend which supports a part of the \
                operations supported by the 'image' library. \n\n\
                Sic' actual purpose is to try out another awesome library for Rust: 'clap'.")
        .arg(Arg::with_name("output-format")
            .short("f")
            .long("output-format")
            .value_name("FORMAT")
            .help("Output formats supported: JPEG, PNG, GIF, ICO, PPM")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the output file")
            .required(true)
            .index(2))
        .get_matches();

    let out_format = matches.value_of("output-format").unwrap_or("JPEG");
    println!("Value for out format: {}", out_format);

    // Can be unwrap because these values are required arguments.
    let in_file = matches.value_of("INPUT").unwrap();
    let out_file = matches.value_of("OUTPUT").unwrap();
    println!("Using input file: {}", in_file);
    println!("Using output file: {}",out_file);

    match image::open(&Path::new(in_file)) {
        Err(reason) => panic!("Unable to process input: {}. reason: {:?}", in_file, reason),
        Ok(img) => {
            println!("Converting to: {} and saving to: {}", out_format, out_file);

            let new_file = &mut File::create(&Path::new(out_file));

            match new_file {
                Err(reason) => panic!("Unable to create a file at: {}, reason: {:?}", out_file, reason),
                Ok(out) => {

                    let out_format_decided_on = which_image_format(out_format);

                    match img.save(out, out_format_decided_on) {
                        Err(reason) => panic!("Unable to save file to: {}, reason: {:?}", out_file, reason),
                        Ok(_) => println!("Done!"),
                    }
                },
            }
        },
    }

}
