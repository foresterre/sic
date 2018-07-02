extern crate clap;
extern crate image;

use std::fs::File;
use std::path::Path;

use clap::{App, Arg};

#[cfg(test)]
mod tests;

// TODO{code}: improve program flow; especially regarding errors/panics
// TODO{code}: use image.save() if not forcing an encoding
// TODO{test}: add tests with images
// TODO{code,test}: test on constraints of ICO format (256x256) before trying to convert
// TODO{code,test}: accept different versions of supported formats
// TODO{bug}: if an image can't be encoded, at this moment, an empty file will still be created


fn image_format_from_str(format: &str) -> Option<image::ImageOutputFormat> {

    let format_in_lower_case: &str = &* format.to_string().to_lowercase();

    match format_in_lower_case {
        "bmp" => Some(image::ImageOutputFormat::BMP),
        "gif" => Some(image::ImageOutputFormat::GIF),
        "ico" => Some(image::ImageOutputFormat::ICO),
        "jpeg" | "jpg" => Some(image::ImageOutputFormat::JPEG(80)),
        "png" => Some(image::ImageOutputFormat::PNG),
        "ppm" => Some(image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Pixmap(image::pnm::SampleEncoding::Binary))),
        _ => None,
    }
}

fn save_file_with_format(img: image::DynamicImage, out: &mut File, out_str: &str, format: image::ImageOutputFormat) {
    match img.write_to(out, format) {
        Err(reason) => panic!("Unable to save file to: {}, reason: {:?}", out_str, reason),
        Ok(_) => println!("Done!"),
    }
}

fn create_file_with_format(img: image::DynamicImage, out_str: &str, format: image::ImageOutputFormat) {
    let new_file = &mut File::create(&Path::new(out_str));

    match new_file {
        Err(reason) => panic!("Unable to create a file at: {}, reason: {:?}", out_str, reason),
        Ok(out) => {
            save_file_with_format(img, out, out_str, format)
        },
    }
}

fn convert_image(in_file: &str, out_str: &str, out_format: image::ImageOutputFormat) {
    match image::open(&Path::new(in_file)) {
        Err(reason) => panic!("Unable to process input: {}. reason: {:?}", in_file, reason),
        Ok(img) => {
            println!("Converting to: {:?} and saving to: {}", out_format, out_str);
            create_file_with_format(img, out_str, out_format)
        },
    }
}

fn get_extension(path: &str) -> Option<&str> {
    let split_by_dot: Vec<&str> = path.split('.').collect();
    let last = split_by_dot.last();

    let maybe_format = last.cloned();

    if maybe_format != Some(path) {
        maybe_format
    }
    else {
        None
    }
}

fn determine_format_by_extension(file_path: &str) -> Option<image::ImageOutputFormat>{
    let ext = get_extension(file_path);

    match ext {
        Some(name) => image_format_from_str(name),
        None => None,
    }
}

// TODO: replace with image::save() if not forcing format
fn determine_format(force_format: Option<&str>, out_file: &str) -> Option<image::ImageOutputFormat> {
    let final_format = match force_format {
        Some(format_str) => image_format_from_str(format_str),
        None => determine_format_by_extension(out_file),
    };

    final_format
}

fn main() {

    // TODO{docs}: supported input formats of the image crate.

    let matches = App::new("Simple Image Converter")
        .version("0.3.0")
        .author("foresterre <garm@ilumeo.com>")
        .about("Converts an image from one format to another.\n\n\
                Supported input formats are described BMP, GIF, ICO, JPEG, PNG, PPM (limitations may apply). \n\n\
                The image conversion is actually done by the awesome 'image' crate [1]. \n\
                Sic itself is a small command line frontend which supports a small part of the \
                conversion operations supported by the 'image' library. \n\n\
                [1] image crate by PistonDevelopers: https://github.com/PistonDevelopers/image \n\n\
                ")
        .arg(Arg::with_name("FORCED_OUTPUT_FORMAT")
            .short("f")
            .long("force-format")
            .value_name("FORMAT")
            .help("Output formats supported: JPEG, PNG, GIF, ICO, PPM")
            .takes_value(true))
        .arg(Arg::with_name("INPUT_FILE")
            .help("Sets the input file")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT_FILE")
            .help("Sets the output file")
            .required(true)
            .index(2))
        .get_matches();

    // Can be unwrap because these values are required arguments.
    let in_file = matches.value_of("INPUT_FILE").unwrap();
    let out_file = matches.value_of("OUTPUT_FILE").unwrap();
    println!("Provided input file: {}", in_file);
    println!("Provided output file: {}", out_file);

    let forced_format = matches.value_of("FORCED_OUTPUT_FORMAT");

    // if the forced format is set, try to use that if available, if not available, panic!
    // if forced format is not set (i.e. None)
    // - check if the file has an extension which is available if so use that, if not panic!
    //   the above removes support for a default format; so if no format is provided
    //   by means of either `-f` or as output file extension, the program panics.

    let final_format: Option<image::ImageOutputFormat> = determine_format(forced_format, out_file);

    if let Some(format) = final_format {
        convert_image(in_file, out_file, format)
    }
}