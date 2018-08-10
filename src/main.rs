#![feature(extern_prelude)]

extern crate clap;
extern crate image;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::{App, Arg};

use std::path::Path;
use std::process;

mod conversion;
mod operations;

const SIC_LICENSE: &str = include_str!("../LICENSE");

fn main() {
    let matches = App::new("Simple Image Converter")
        .version("0.5.1")
        .author("Martijn Gribnau <garm@ilumeo.com>")
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
        .arg(Arg::with_name("license")
            .long("license")
            .value_name("LICENSE")
            .help("Displays the license of the software.")
            .takes_value(false))
        .arg(Arg::with_name("script")
            .long("script")
            .help("Apply image operations on the input image.\n\
                   Supported operations: \n\
                   1. blur <uint>;\n\
                   2. flip_horizontal;\n\
                   3. flip_vertical;\n\
                   4. resize <uint> <uint>;\n\n\
                   Operation separators (';') are optional.\n\n\
                   Example 1: `sic input.png output.png --script \"resize 250 250; blur 5;\"`\n\
                   Example 2: `sic input.png output.jpg --script \"flip_horizontal resize 10 5 blur 100\"`")
            .value_name("SCRIPT")
            .takes_value(true))
        .arg(Arg::with_name("input_file")
            .help("Sets the input file")
            .value_name("INPUT_FILE")
            .required_unless("license")
            .index(1))
        .arg(Arg::with_name("output_file")
            .help("Sets the output file")
            .value_name("OUTPUT_FILE")
            .required_unless("license")
            .index(2))
        .get_matches();

    if matches.is_present("license") {
        println!("{}", SIC_LICENSE);
        process::exit(0);
    }

    // Can be unwrap because these values are required arguments.
    let input = matches.value_of("input_file").unwrap();
    let output = matches.value_of("output_file").unwrap();
    println!("Provided input file path: {}", input);
    println!("Provided output file path: {}", output);

    let image_buffer: Result<image::DynamicImage, String> =
        image::open(&Path::new(input)).map_err(|err| err.to_string());

    // perform image operations
    let operated_buffer = match matches.value_of("script") {
        Some(script) => image_buffer
            .map_err(|err| err.to_string())
            .and_then(|img| operations::parse_and_apply_script(img, script)),
        None => image_buffer,
    };

    // encode
    let forced_format = matches.value_of("forced_output_format");
    let encode_buffer: Result<(), String> = operated_buffer
        .map_err(|err| err.to_string())
        .and_then(|img| {
            forced_format.map_or_else(
                || conversion::convert_image_unforced(&img, output),
                |format| conversion::convert_image_forced(&img, output, format),
            )
        });

    match encode_buffer {
        Ok(_) => println!("Conversion complete."),
        Err(err) => println!("Conversion ended with an Error: {}", err),
    }
}
