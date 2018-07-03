extern crate clap;
extern crate image;

use std::fs::File;
use std::path::Path;

use clap::{App, Arg};

#[cfg(test)]
mod tests;

// TODO{test}: add tests with images
// TODO{code,test}: test on constraints of ICO format (256x256) before trying to convert
// TODO{code,test}: accept different versions of supported formats
// TODO{bug}: if an image can't be encoded, at this moment, an empty file will still be created

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

fn main() {
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
    let in_file_path = matches.value_of("INPUT_FILE").unwrap();
    let out_file_path = matches.value_of("OUTPUT_FILE").unwrap();
    println!("Provided input file: {}", in_file_path);
    println!("Provided output file: {}", out_file_path);

    let forced_format = matches.value_of("FORCED_OUTPUT_FORMAT");

    if forced_format.is_some() {
        let image = image::open(&Path::new(in_file_path)).expect("Unable to open input image.");
        let requested_image_format: &str = forced_format.expect("Unable to access requested image output format");
        let image_format = image_format_from_str(requested_image_format).expect("Requested image output format not supported.");

        let output_file = &mut File::create(&Path::new(out_file_path)).expect("Unable to save image result.");

        let _ = image.write_to(output_file, image_format);
    }
    else {
        match convert_image_unforced(in_file_path, out_file_path) {
            Ok(_) => println!("Complete."),
            Err(err) => println!("Error: {}", err),
        }
    }
}

// todo: how to deal with mutable borrows and closures
// todo: combine forced and unforced
// fn convert_image_forced(input: &str, output: &str, format: &str) -> Result<(), String> {
//     image_format_from_str(format)
//         .map_err(|err| err.to_string())
//         .and_then(|image_format| {
//             image::open(&Path::new(input))
//                 .map_err(|err| err.to_string())
//                 .map(|image| (image, image_format) )
//         })
//         .and_then(|(image, image_format)| {
//             let out = &mut File::create(&Path::new(output))
//                 .map_err(|err| err.to_string())
//                 .map(|fout| image.write_to(&mut fout, image_format))
//                     .map_err(|err| err.to_string());

//             out.map(|_| ())
//         })
// }

fn convert_image_unforced(input: &str, output: &str) -> Result<(), String> {
    image::open(&Path::new(input))
        .map_err(|err| err.to_string())
        .and_then(|image| image.save(output)
            .map_err(|err| err.to_string()))
}

