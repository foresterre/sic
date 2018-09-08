use image;

#[macro_use]
extern crate pest_derive;

use std::path::Path;
use std::process;

use clap::{App, Arg};

mod conversion;
mod help;
mod operations;

const SIC_LICENSE: &str = include_str!("../LICENSE");
const DEP_LICENSES: &str = include_str!("../LICENSES_DEPENDENCIES");

const HELP_OPERATIONS_AVAILABLE: &str = include_str!("../docs/cli_help_script.txt");

fn main() {
    let matches = App::new("Simple Image Converter")
        .version("0.7.2")
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
            .help("Displays the license of the `sic` software.")
            .takes_value(false))
        .arg(Arg::with_name("dep-licenses")
            .long("dep-licenses")
            .help("Displays the licenses of the dependencies on which this software relies.")
            .takes_value(false))
        .arg(Arg::with_name("user-manual")
            .long("user-manual")
            .short("H")
            .help("Displays help text for different topics such as each supported script operation. Run `sic -H index` to display a list of available topics.")
            .value_name("TOPIC")
            .takes_value(true))
        .arg(Arg::with_name("script")
            .long("script")
            .help(HELP_OPERATIONS_AVAILABLE)
            .value_name("SCRIPT")
            .takes_value(true))
        .arg(Arg::with_name("input_file")
            .help("Sets the input file")
            .value_name("INPUT_FILE")
            .required_unless_one(&["license", "dep-licenses", "user-manual"])
            .index(1))
        .arg(Arg::with_name("output_file")
            .help("Sets the output file")
            .value_name("OUTPUT_FILE")
            .required_unless_one(&["license", "dep-licenses", "user-manual"])
            .index(2))
        .get_matches();

    match (
        matches.is_present("license"),
        matches.is_present("dep-licenses"),
        matches.is_present("user-manual"),
    ) {
        (true, true, _) => {
            println!(
                "Simple Image Converter license:\n{} \n\n{}",
                SIC_LICENSE, DEP_LICENSES
            );
            process::exit(0);
        }
        (true, _, _) => {
            println!("{}", SIC_LICENSE);
            process::exit(0);
        }
        (_, true, _) => {
            println!("{}", DEP_LICENSES);
            process::exit(0);
        }
        (false, false, true) => {
            if let Some(topic) = matches.value_of("user-manual") {
                let help = help::HelpIndex::new();
                let page = help.get_topic(&*topic.to_lowercase());

                match page {
                    Some(it) => println!("{}", it.help_text),
                    None => println!("This topic is unavailable in the user manual. The following topics are available: \n\t* {}", help.get_available_topics()),
                }

                process::exit(0);
            }
        }
        _ => {}
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
        Some(script) => {
            println!("Preparing to apply image operations: `{}`", script);
            image_buffer.map_err(|err| err.to_string()).and_then(|img| {
                println!("Applying image operations.");
                operations::parse_and_apply_script(img, script)
            })
        }
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
        Ok(_) => println!("Operations complete."),
        Err(err) => println!("Operations ended with an Error: {}", err),
    }
}
