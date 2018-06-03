extern crate clap;
extern crate image;

use std::fs::File;
use std::path::Path;

use clap::{App, Arg};

// TODO{code}: improve program flow; especially regarding errors/panics
// TODO{test}: improve testing
// TODO{code,test}: test on constraints of ICO format (256x256) before trying to convert
// TODO{code,test}: accept different versions of supported formats, i.e.
// TODO{bug}: if an image can't be encoded, at this moment, an empty file will still be created

fn image_format_from_str(format: &str) -> Option<image::ImageFormat> {

    let format_in_uppercase: &str = &* format.to_string().to_uppercase();

    match format_in_uppercase {
        "BMP" => Some(image::BMP),
        "GIF" => Some(image::GIF),
        "ICO" => Some(image::ICO),
        "JPEG" | "JPG" => Some(image::JPEG),
        "PNG" => Some(image::PNG),
        "PPM" => Some(image::PPM),
        _ => None,
    }
}

fn save_file_with_format(img: image::DynamicImage, out: &mut File, out_str: &str, format: image::ImageFormat) {
    match img.save(out, format) {
        Err(reason) => panic!("Unable to save file to: {}, reason: {:?}", out_str, reason),
        Ok(_) => println!("Done!"),
    }
}

fn create_file_with_format(img: image::DynamicImage, out_str: &str, format: image::ImageFormat) {
    let new_file = &mut File::create(&Path::new(out_str));

    match new_file {
        Err(reason) => panic!("Unable to create a file at: {}, reason: {:?}", out_str, reason),
        Ok(out) => {
            save_file_with_format(img, out, out_str, format)
        },
    }
}

fn convert_image(in_file: &str, out_str: &str, out_format: image::ImageFormat) {
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

fn determine_format_by_extension(file_path: &str) -> Option<image::ImageFormat>{
    let ext = get_extension(file_path);

    match ext {
        Some(name) => image_format_from_str(name),
        None => None,
    }
}

fn determine_format(force_format: Option<&str>, out_file: &str) -> Option<image::ImageFormat> {
    let final_format = match force_format {
        Some(format_str) => image_format_from_str(format_str),
        None => determine_format_by_extension(out_file),
    };

    final_format
}

fn main() {

    // TODO{docs}: supported input formats of the image crate.

    let matches = App::new("Simple Image Converter")
        .version("0.2.0")
        .author("foresterre <garm@ilumeo.com>")
        .about("Converts an image from one format to another.\n\n\
                Supported input formats are PNG, JPEG (baseline, progressive), GIF, BMP\
                ICO, TIFF (Baseline(no fax support) + LZW + PackBits), Webp (Lossy(Luma channel only))\
                PNM (PBM, PGM, PPM, standard PAM) [1] \n\n\
                The image conversion is actually done by the awesome 'image' crate. \n\
                Sic is only a command line frontend which supports a part of the \
                conversion operations supported by the 'image' library. \n\n\
                [1] source: https://github.com/PistonDevelopers/image#2-supported-image-formats \n\n\
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

    let final_format = determine_format(forced_format, out_file);

    if let Some(format) = final_format {
        convert_image(in_file, out_file, format)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // which_image_format

    // GIF

    #[test]
    fn image_format_from_str_gif_uc() {
        assert_eq!(Some(image::GIF), image_format_from_str("GIF"));
    }

    #[test]
    fn image_format_from_str_gif_lc() {
        assert_eq!(Some(image::GIF), image_format_from_str("gif"));
    }

    #[test]
    fn image_format_from_str_gif_mc() {
        assert_eq!(Some(image::GIF), image_format_from_str("gIF"));
    }

    // ICO

    #[test]
    fn image_format_from_str_ico_uc() {
        assert_eq!(Some(image::ICO), image_format_from_str("ICO"));
    }

    #[test]
    fn image_format_from_str_ico_lc() {
        assert_eq!(Some(image::ICO), image_format_from_str("ico"));
    }

    #[test]
    fn image_format_from_str_ico_mc() {
        assert_eq!(Some(image::ICO), image_format_from_str("icO"));
    }

    // JPG/JPEG

    #[test]
    fn image_format_from_str_jpeg_uc() {
        assert_eq!(Some(image::JPEG), image_format_from_str("JPEG"));
    }

    #[test]
    fn image_format_from_str_jpeg_lc() {
        assert_eq!(Some(image::JPEG), image_format_from_str("jpeg"));
    }

    #[test]
    fn image_format_from_str_jpeg_mc() {
        assert_eq!(Some(image::JPEG), image_format_from_str("jPeG"));
    }

    #[test]
    fn image_format_from_str_jpg_uc() {
        assert_eq!(Some(image::JPEG), image_format_from_str("JPG"));
    }

    #[test]
    fn image_format_from_str_jpg_lc() {
        assert_eq!(Some(image::JPEG), image_format_from_str("jpg"));
    }

    #[test]
    fn image_format_from_str_jpg_mc() {
        assert_eq!(Some(image::JPEG), image_format_from_str("jPG"));
    }


    // PNG

    #[test]
    fn image_format_from_str_png_uc() {
        assert_eq!(Some(image::PNG), image_format_from_str("PNG"));
    }

    #[test]
    fn image_format_from_str_png_lc() {
        assert_eq!(Some(image::PNG), image_format_from_str("png"));
    }

    #[test]
    fn image_format_from_str_png_mc() {
        assert_eq!(Some(image::PNG), image_format_from_str("pNg"));
    }

    // PPM

    #[test]
    fn image_format_from_str_ppm_uc() {
        assert_eq!(Some(image::PPM), image_format_from_str("PPM"));
    }

    #[test]
    fn image_format_from_str_ppm_lc() {
        assert_eq!(Some(image::PPM), image_format_from_str("ppm"));
    }

    #[test]
    fn image_format_from_str_ppm_mc() {
        assert_eq!(Some(image::PPM), image_format_from_str("pPm"));
    }

    // determine_format_by_extension

    #[test]
    fn determine_format_by_extension_ok_path() {
        assert_eq!(Some(image::PNG), determine_format_by_extension("C:/users/some/path.png"));
    }

    #[test]
    fn determine_format_by_extension_test_ok_file() {
        assert_eq!(Some(image::PNG), determine_format_by_extension("path.png"));
    }

    #[test]
    fn determine_format_by_extension_test_no_ext_path() {
        assert_eq!(None, determine_format_by_extension("C:/users/some/png"));
    }

    #[test]
    fn determine_format_by_extension_test_no_ext_file() {
        assert_eq!(None, determine_format_by_extension("png"));
    }

    // get_extension

    #[test]
    fn get_extension_ok_path() {
        assert_eq!(Some("png"), get_extension("C:/users/some/path.png"));
    }

    #[test]
    fn get_extension_test_ok_file() {
        assert_eq!(Some("png"), get_extension("path.png"));
    }

    #[test]
    fn get_extension_test_no_ext_path() {
        assert_eq!(None, get_extension("C:/users/some/png"));
    }

    #[test]
    fn get_extension_test_no_ext_file() {
        assert_eq!(None, get_extension("png"));
    }

}

