use std::collections::BTreeMap;
use std::str::FromStr;

use anyhow::{anyhow, bail};
use arg_names::*;
use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches};
use sic_cli_ops::build_ast_from_matches;
use sic_cli_ops::operations::{IndexTree, OperationId};
use sic_io::load::FrameIndex;

use crate::cli::config::{
    validate_jpeg_quality, Config, ConfigBuilder, InputOutputModeType, SelectedLicenses,
};

// table of argument names
pub(crate) mod arg_names {
    // cli - possible arguments

    // organisational:
    pub(crate) const ARG_LICENSE: &str = "license";
    pub(crate) const ARG_DEP_LICENSES: &str = "dep_licenses";

    // io(input, output):
    pub(crate) const ARG_INPUT: &str = "input";
    pub(crate) const ARG_OUTPUT: &str = "output";

    pub(crate) const ARG_MODE: &str = "mode";

    // config(in):
    pub(crate) const ARG_SELECT_FRAME: &str = "select_frame";

    // config(out):
    pub(crate) const ARG_DISABLE_AUTOMATIC_COLOR_TYPE_ADJUSTMENT: &str =
        "disable_automatic_color_type_adjustment";
    pub(crate) const ARG_FORCED_OUTPUT_FORMAT: &str = "forced_output_format";
    pub(crate) const ARG_JPEG_ENCODING_QUALITY: &str = "jpeg_encoding_quality";

    pub(crate) const ARG_PNM_ENCODING_ASCII: &str = "pnm_encoding_ascii";

    // image-operations(script):
    pub(crate) const ARG_APPLY_OPERATIONS: &str = "script";

    // image-operations(cli-arguments):
    pub(crate) const GROUP_IMAGE_OPERATIONS: &str = "group";
}

pub fn cli(
    version: &'static str,
    about: &'static str,
    help_ops: &'static str,
) -> App<'static, 'static> {
    App::new("sic")
        .version(version)
        .about(about)
        .after_help("For more information, visit: https://github.com/foresterre/sic")
        .author("Martijn Gribnau <garm@ilumeo.com>")

        // settings
        .global_setting(AppSettings::NextLineHelp)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::DontCollapseArgsInUsage)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .max_term_width(120)

        // cli arguments

        // organisational:
        .arg(Arg::with_name(ARG_LICENSE)
            .long("license")
            .help("Displays the license of this piece of software (`sic`).")
            .takes_value(false)
            .conflicts_with_all(&[ARG_DEP_LICENSES, ARG_INPUT, ARG_OUTPUT]))
        .arg(Arg::with_name(ARG_DEP_LICENSES)
            .long("dep-licenses")
            .help("Displays the licenses of the dependencies on which this software relies.")
            .takes_value(false)
            .conflicts_with_all(&[ARG_LICENSE, ARG_INPUT, ARG_OUTPUT]))

        // io(input):
        .arg(Arg::with_name(ARG_INPUT)
            .long("input")
            .short("i")
            .value_name("INPUT_PATH")
            .takes_value(true)
            .help("Input image path. When using this option, input piped from stdin will be ignored. \
                      In glob mode, depending on your shell you may need to add explicit quotation marks around the argument")
            .conflicts_with_all(&[ARG_LICENSE, ARG_DEP_LICENSES]))

        // io(output):
        .arg(Arg::with_name(ARG_OUTPUT)
            .long("output")
            .short("o")
            .value_name("OUTPUT_PATH")
            .takes_value(true)
            .help("Output image path. When using this option, output won't be piped to stdout.")
            .conflicts_with_all(&[ARG_LICENSE, ARG_DEP_LICENSES]))

        .arg(Arg::with_name(ARG_MODE)
            .long("mode")
            .value_name("MODE")
            .takes_value(true)
            .possible_values(&["simple", "glob"])
            .default_value("simple")
            .help("Use 'simple' mode when using a single input- and a single output-file; \
                      Use 'glob' mode when using glob patterns as input, the output path should take \
                      a root directory where output images will be copied, using a mirrored directory structure")
        )

        // config(in):
        .arg(Arg::with_name(ARG_SELECT_FRAME)
            .long("select-frame")
            .value_name("#FRAME")
            .help("Frame to be loaded as still image if the input image is an animated image.\
                      To pick the first and last frame respectively, you can provide 'first' and 'last' as arguments. \
                      Otherwise provide a single one-indexed positive number which corresponds with the frame index. \
                      For example, to select the first frame, the argument would be '1', for the second '2', etc.")
            .takes_value(true))

        // config(out):
        .arg(Arg::with_name(ARG_DISABLE_AUTOMATIC_COLOR_TYPE_ADJUSTMENT)
            .long("disable-automatic-color-type-adjustment")
            .help("Some image output formats do not support the color type of the image buffer prior to encoding. \
                      By default the program tries to adjust the color type. If this flag is provided, \
                      the program will not try to adjust the color type."))

        .arg(Arg::with_name(ARG_FORCED_OUTPUT_FORMAT)
            .short("f")
            .long("output-format")
            .value_name("FORMAT")
            .help("Force the output image format to use FORMAT, regardless of the (if any) extension of the given output file path. \
                      Output formats (FORMAT values) supported: BMP, GIF, ICO, JPEG, PNG, PBM, PGM, PPM and PAM.")
            .takes_value(true))

        .arg(Arg::with_name(ARG_JPEG_ENCODING_QUALITY)
            .long("jpeg-encoding-quality")
            .help("Set the jpeg quality to QUALITY. Valid values are positive numbers from 1 up to and including 100. Will only be used when the output format is determined to be jpeg.")
            .value_name("QUALITY")
            .takes_value(true))

        .arg(Arg::with_name(ARG_PNM_ENCODING_ASCII)
            .long("pnm-encoding-ascii")
            .help("Use ascii based encoding when using a PNM image output format (pbm, pgm or ppm). Doesn't apply to 'pam' (PNM Arbitrary Map)."))

        // image-operations(script):
        .arg(Arg::with_name(ARG_APPLY_OPERATIONS)
            .long("apply-operations")
            .short("x")
            .alias("A")
            .help(help_ops)
            .value_name("OPERATIONS")
            .takes_value(true))

        // image-operations(cli-arguments):
        .group(ArgGroup::with_name(GROUP_IMAGE_OPERATIONS)
            .args(&[
                OperationId::Blur.as_str(),
                OperationId::Brighten.as_str(),
                OperationId::Contrast.as_str(),
                OperationId::Crop.as_str(),
                OperationId::Diff.as_str(),
                OperationId::Filter3x3.as_str(),
                OperationId::FlipH.as_str(),
                OperationId::FlipV.as_str(),
                OperationId::Grayscale.as_str(),
                OperationId::HueRotate.as_str(),
                OperationId::Invert.as_str(),
                OperationId::Resize.as_str(),
                OperationId::Rotate90.as_str(),
                OperationId::Rotate180.as_str(),
                OperationId::Rotate270.as_str(),
                OperationId::Unsharpen.as_str(),

                OperationId::ModResizePreserveAspectRatio.as_str(),
                OperationId::ModResizeSamplingFilter.as_str(),
            ])
            .conflicts_with(ARG_APPLY_OPERATIONS)
            .multiple(true))
        .arg(Arg::with_name(OperationId::Blur.as_str())
            .help("Operation: blur.")
            .long("--blur")
            .takes_value(true)
            .value_name("fp")
            .number_of_values(1)
            .multiple(true)
            .allow_hyphen_values(true))
        .arg(Arg::with_name(OperationId::Brighten.as_str())
            .help("Operation: brighten.")
            .long("--brighten")
            .takes_value(true)
            .value_name("int")
            .number_of_values(1)
            .multiple(true)
            .allow_hyphen_values(true))
        .arg(Arg::with_name(OperationId::Contrast.as_str())
            .help("Operation: contrast.")
            .long("--contrast")
            .takes_value(true)
            .value_name("fp")
            .number_of_values(1)
            .multiple(true)
            .allow_hyphen_values(true))
        .arg(Arg::with_name(OperationId::Crop.as_str())
            .help("Operation: crop.")
            .long("--crop")
            .takes_value(true)
            .value_name("uint uint uint uint")
            .number_of_values(4)
            .multiple(true))
        .arg(Arg::with_name(OperationId::Diff.as_str())
            .help("Operation: diff.")
            .long("--diff")
            .takes_value(true)
            .value_name("path to image")
            .number_of_values(1)
            .multiple(true))
        .arg(Arg::with_name(OperationId::Filter3x3.as_str())
            .help("Operation: filter3x3.")
            .long("--filter3x3")
            .takes_value(true)
            .value_name("fp fp fp fp fp fp fp fp fp")
            .number_of_values(9)
            .multiple(true)
            .allow_hyphen_values(true))
        .arg(Arg::with_name(OperationId::FlipH.as_str())
            .help("Operation: flip horizontal.")
            .long("--flip-horizontal")
            .multiple(true))
        .arg(Arg::with_name(OperationId::FlipV.as_str())
            .help("Operation: flip vertical.")
            .long("--flip-vertical")
            .multiple(true))
        .arg(Arg::with_name(OperationId::Grayscale.as_str())
            .help("Operation: grayscale.")
            .long("--grayscale")
            .multiple(true))
        .arg(Arg::with_name(OperationId::Invert.as_str())
            .help("Operation: invert.")
            .long("--invert")
            .multiple(true))
        .arg(Arg::with_name(OperationId::HueRotate.as_str())
            .help("Operation: hue rotate.")
            .long("--hue-rotate")
            .takes_value(true)
            .value_name("int")
            .number_of_values(1)
            .multiple(true)
            .allow_hyphen_values(true))
        .arg(Arg::with_name(OperationId::Resize.as_str())
            .help("Operation: resize.")
            .long("--resize")
            .takes_value(true)
            .value_name("uint uint")
            .number_of_values(2)
            .multiple(true))
        .arg(Arg::with_name(OperationId::Rotate90.as_str())
            .help("Operation: rotate 90 degree.")
            .long("--rotate90")
            .multiple(true))
        .arg(Arg::with_name(OperationId::Rotate180.as_str())
            .help("Operation: rotate 180 degree.")
            .long("--rotate180")
            .multiple(true))
        .arg(Arg::with_name(OperationId::Rotate270.as_str())
            .help("Operation: rotate 270 degree.")
            .long("--rotate270")
            .multiple(true))
        .arg(Arg::with_name(OperationId::Unsharpen.as_str())
            .help("Operation: unsharpen.")
            .long("--unsharpen")
            .takes_value(true)
            .value_name("fp int")
            .number_of_values(2)
            .multiple(true)
            .allow_hyphen_values(true))

        // image-operations(cli-arguments/modifiers):
        .arg(Arg::with_name(OperationId::ModResizePreserveAspectRatio.as_str())
            .help("Operation modifier for: resize")
            .long("--preserve-aspect-ratio")
            .takes_value(true)
            .value_name("bool")
            .number_of_values(1)
            .multiple(true)
            .possible_values(&["true", "false"])
        )
        .arg(Arg::with_name(OperationId::ModResizeSamplingFilter.as_str())
            .help("Operation modifier for: resize")
            .long("--sampling-filter")
            .takes_value(true)
            .value_name("str")
            .number_of_values(1)
            .multiple(true)
            .possible_values(&["catmullrom", "gaussian", "lanczos3", "nearest", "triangle"])
        )
}

// Here any argument should not panic when invalid.
// Previously, it was allowed to panic within Config, but this is no longer the case.
pub fn build_app_config<'a>(matches: &'a ArgMatches) -> anyhow::Result<Config<'a>> {
    let mut builder = ConfigBuilder::new();

    // organisational/licenses:
    let texts_requested = (
        matches.is_present(ARG_LICENSE),
        matches.is_present(ARG_DEP_LICENSES),
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

    let io_mode = match matches.value_of(ARG_MODE) {
        Some("glob") => InputOutputModeType::Batch,
        _ => InputOutputModeType::Simple,
    };

    builder = builder.mode(io_mode);

    // config(in)/gif-select-frame:
    if let Some(frame_in) = matches.value_of(ARG_SELECT_FRAME) {
        let frame_out = match frame_in {
            "first" => FrameIndex::First,
            "last" => FrameIndex::Last,
            n => {
                let pick = n.parse::<usize>().map_err(|_| {
                    anyhow!(
                        "Provided argument for --select-frame is not a valid option. \
                         Valid options are 'first', 'last' or a (one-indexed) positive number."
                    )
                })?;

                if pick == 0 {
                    bail!(
                        "Provided argument for --select-frame is not a valid option. \
                         If a number is provided, the number should be positive and larger than 0. \
                         To select the first frame, provide the argument '1'."
                    );
                }

                FrameIndex::Nth(pick - 1)
            }
        };

        builder = builder.select_frame(frame_out);
    }

    // config(out)/disable-automatic-color-type-adjustment:
    if matches.is_present(ARG_DISABLE_AUTOMATIC_COLOR_TYPE_ADJUSTMENT) {
        builder = builder.disable_automatic_color_type_adjustment(true);
    }

    // config(out)/output-format:
    if let Some(format) = matches.value_of(ARG_FORCED_OUTPUT_FORMAT) {
        builder = builder.forced_output_format(format);
    }

    // config(out)/jpeg-encoding-quality:
    if let Some(value) = matches.value_of(ARG_JPEG_ENCODING_QUALITY) {
        let requested_jpeg_quality = u8::from_str(value)
            .map_err(|_| {
                anyhow!("JPEG Encoding quality should be a value between 1 and 100 (inclusive).")
            })
            .and_then(validate_jpeg_quality)?;
        builder = builder.jpeg_quality(requested_jpeg_quality);
    }

    // config(out)/pnm-encoding-type:
    if matches.is_present(ARG_PNM_ENCODING_ASCII) {
        builder = builder.pnm_format_type(true);
    }

    // image-operations:
    //
    // Image operations are a bit more involved.
    // Thanks to clap, we know either ARG_APPLY_OPERATIONS xor GROUP_IMAGE_OPERATIONS
    // will be the method of providing an image operations program.
    //
    // However with Arg::multiple(true) and Arg::number_of_values(n) we can set to allow multiple
    // operations, like: --blur --blur (multiple is ok), and --crop 0 0 1 1 (number of values = 4),
    // but Clap also allows: --crop 0 0 --crop 1 1 (multiple is ok and number of values = 4).
    //
    // We want to set multiple to true, because image operations can be repeated and can have different
    // effects than the first time.
    // But since the effects can be different, we need to know the order in which arguments are
    // provided. Luckily Clap does tell us the indices of values if we ask for them.
    // If we use --crop 0 0 --blur 1 --crop 1 1, the order of the operations would be undefined, not to
    // say perhaps feel not logical for a user. Therefor, we enforce left to right ordering of
    // operations and require all values to be provided at once, after the operation argument.
    //
    // There is an edge case which we can't (as far as I am aware) handle without looking within the
    // argv ourselves: --crop 0 0 1 1 --crop, is valid according to Clap. However, since we do not
    // receive the amount of times --crop was defined, but rather all the separate provided values for
    // the name of the argument, we just know that for `crop` we have values 0,0,1,1.
    let program = if let Some(script) = matches.value_of(ARG_APPLY_OPERATIONS) {
        sic_parser::parse_script(script)?
    } else {
        let mut tree: IndexTree = BTreeMap::new();
        build_ast_from_matches(matches, &mut tree)?
    };
    builder = builder.image_operations_program(program);

    Ok(builder.build())
}
