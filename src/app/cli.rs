use clap::{App, AppSettings, Arg, ArgMatches};

use crate::app::config::{validate_jpeg_quality, Config, ConfigBuilder, SelectedLicenses};
use crate::get_tool_name;
use std::str::FromStr;

const HELP_OPERATIONS_AVAILABLE: &str = include_str!("../../docs/cli_help_script.txt");

pub fn cli() -> App<'static, 'static> {
    App::new(get_tool_name())
        .version(env!("CARGO_PKG_VERSION"))
        .about("An image tool app front-end which can convert images to different formats, and transform \
                images by applying image operations.\n\n\
                Supported input (decoding) formats are:  BMP, GIF, ICO, JPEG, PNG, PBM, PGM, PPM,\n\
                PAM and TIFF and WebP.\n\
                Supported output (encoding) formats are: BMP, GIF, ICO, JPEG, PNG, PBM, PGM, PPM \n\
                and PAM.\n\
                Limitations may apply for both input and output formats. For compatibility information see:[1].\n\n\
                The image conversion and image operations are made possible by the awesome 'image' library [2].\n\
                Run `sic --help` for all available flags and options and `sic --user-manual <OPERATION>`\n\
                for help on the image operations supported by the `--apply-operations \"<OPERATION(S)>\"`` option.\n\n\
                [1] https://github.com/PistonDevelopers/image/tree/13372d52ad7ca96da1bb1ca148c57d402bf4c8c0#21-supported-image-formats\n\
                [2] image library by PistonDevelopers: https://github.com/PistonDevelopers/image\n\n\
                ")
        .after_help("For more information, visit: https://github.com/foresterre/sic")
        .author("Martijn Gribnau <garm@ilumeo.com>")

        // Settings
        .setting(AppSettings::NextLineHelp)

        // Base arguments shared between `sic` and `stew`.
        .arg(Arg::with_name("forced_output_format")
            .short("f")
            .long("output-format")
            .value_name("FORMAT")
            .help("Force the output image format to use FORMAT, regardless of the (if any) extension of the given output file path. \
                Output formats (FORMAT values) supported: BMP, GIF, ICO, JPEG, PNG, PBM, PGM, PPM and PAM.")
            .takes_value(true))
        .arg(Arg::with_name("license")
            .long("license")
            .help("Displays the license of this piece of software (`stew`).")
            .takes_value(false))
        .arg(Arg::with_name("dep_licenses")
            .long("dep-licenses")
            .help("Displays the licenses of the dependencies on which this software relies.")
            .takes_value(false))
        .arg(Arg::with_name("jpeg_encoding_quality")
            .long("jpeg-encoding-quality")
            .help("Set the jpeg quality to QUALITY. Valid values are natural numbers from 1 up to and including 100. Will only be used when the output format is determined to be jpeg.")
            .value_name("QUALITY")
            .takes_value(true))
        .arg(Arg::with_name("pnm_encoding_ascii")
            .long("pnm-encoding-ascii")
            .help("Use ascii based encoding when using a PNM image output format (pbm, pgm or ppm). Doesn't apply to 'pam' (PNM Arbitrary Map)."))
        .arg(Arg::with_name("disable_automatic_color_type_adjustment")
            .long("disable-automatic-color-type-adjustment")
            .help("Some image output formats do not support the color type of the image buffer prior to encoding. By default Stew tries to adjust the color type. If this flag is provided, sic will not try to adjust the color type."))
        .arg(Arg::with_name("input")
            .long("input")
            .short("i")
            .value_name("FILE_INPUT")
            .takes_value(true)
            .help("Input image path. When using this option, input piped from stdin will be ignored."))
        .arg(Arg::with_name("output")
            .long("output")
            .short("o")
            .value_name("FILE_OUTPUT")
            .takes_value(true)
            .help("Output image path. When using this option, output won't be piped to stdout."))

        // Selective arguments for `sic`.
        .arg(Arg::with_name("user_manual")
            .long("user-manual")
            .short("H")
            .help("Displays help text for different topics such as each supported script operation. Run `sic -H index` to display a list of available topics.")
            .value_name("TOPIC")
            .takes_value(true))
        .arg(Arg::with_name("script")
            .long("apply-operations")
            .short("x")
            .alias("A")
            .help(HELP_OPERATIONS_AVAILABLE)
            .value_name("OPERATIONS")
            .takes_value(true))
        .arg(Arg::with_name("input_file")
            .help("Sets the input file")
            .value_name("INPUT_FILE")
            .required_unless_one(&["input", "license", "dep_licenses", "user_manual"])
            .index(1))
        .arg(Arg::with_name("output_file")
            .help("Sets the desired output file")
            .value_name("OUTPUT_FILE")
            .required_unless_one(&["output", "license", "dep_licenses", "user_manual"])
            .index(2))
}

// Here any argument should not panic when invalid.
// Previously, it was allowed to panic within Config, but this is no longer the case.
pub fn build_app_config<'a>(matches: &'a ArgMatches) -> Result<Config<'a>, String> {
    let mut builder = ConfigBuilder::new();

    // next setting.
    let texts_requested = (
        matches.is_present("license"),
        matches.is_present("dep_licenses"),
    );

    match texts_requested {
        (true, false) => {
            builder = builder.show_license_text_of(SelectedLicenses::ThisSoftware);
        }
        (false, true) => {
            builder = builder.show_license_text_of(SelectedLicenses::Dependencies);
        }
        (true, true) => {
            builder = builder.show_license_text_of(SelectedLicenses::ThisSoftwarePlusDependencies);
        }
        (false, false) => (),
    };

    // next setting.
    if let Some(format) = matches.value_of("forced_output_format") {
        builder = builder.forced_output_format(format);
    }

    // next setting.
    if matches.is_present("disable_automatic_color_type_adjustment") {
        builder = builder.disable_automatic_color_type_adjustment(true);
    }

    // next setting.
    if let Some(value) = matches.value_of("jpeg_encoding_quality") {
        let requested_jpeg_quality = u8::from_str(value)
            .map_err(|_| {
                "JPEG Encoding quality should be a value between 1 and 100 (inclusive).".to_string()
            })
            .and_then(validate_jpeg_quality)?;
        builder = builder.jpeg_quality(requested_jpeg_quality);
    }

    // next setting.
    if matches.is_present("pnm_encoding_ascii") {
        builder = builder.pnm_format_type(true);
    }

    // next setting.
    if let Some(path) = matches
        .value_of("output")
        .or_else(|| matches.value_of("output_file"))
    {
        builder = builder.output_path(path);
    }

    // next setting.
    if let Some(script) = matches.value_of("script") {
        builder = builder.image_operations_script(script);
    }

    // next setting.
    if let Some(topic) = matches.value_of("user_manual") {
        builder = builder.image_operations_manual_keyword(topic);
    }

    Ok(builder.build())
}
