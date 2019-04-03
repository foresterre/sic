#[macro_use]
extern crate pest_derive;

use clap::App;
use combostew_cli::get_app_skeleton;

pub mod app_config;
pub mod help;
pub mod parser;
pub mod sic_processor;

pub fn get_tool_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

pub fn sic_app_skeleton(name: &str) -> App<'static, 'static> {
    get_app_skeleton(name)
        .version(env!("CARGO_PKG_VERSION"))
        .about("An image tool cli front-end which can convert images to different formats, and transform \
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
}
