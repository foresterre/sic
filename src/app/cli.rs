use std::collections::BTreeMap;
use std::str::FromStr;

use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches};
use sic_image_engine::engine::Program;

use crate::app::config::{validate_jpeg_quality, Config, ConfigBuilder, SelectedLicenses};
use crate::app::img_op_arg::{tree_extend_unifiable, IndexTree, IndexedOps, Op, OperationId};
use crate::get_tool_name;
use arg_names::*;

const HELP_OPERATIONS_AVAILABLE: &str = include_str!("../../docs/cli_help_script.txt");

// table of argument names
pub(crate) mod arg_names {
    // cli - possible arguments
    pub(crate) const ARG_FORCED_OUTPUT_FORMAT: &str = "forced_output_format";
    pub(crate) const ARG_LICENSE: &str = "license";
    pub(crate) const ARG_DEP_LICENSES: &str = "dep_licenses";
    pub(crate) const ARG_JPEG_ENCODING_QUALITY: &str = "jpeg_encoding_quality";
    pub(crate) const ARG_PNM_ENCODING_ASCII: &str = "pnm_encoding_ascii";
    pub(crate) const ARG_DISABLE_AUTOMATIC_COLOR_TYPE_ADJUSTMENT: &str =
        "disable_automatic_color_type_adjustment";
    pub(crate) const ARG_INPUT: &str = "input";
    pub(crate) const ARG_OUTPUT: &str = "output";
    pub(crate) const ARG_USER_MANUAL: &str = "user_manual";
    pub(crate) const ARG_INPUT_FILE: &str = "input_file";
    pub(crate) const ARG_OUTPUT_FILE: &str = "output_file";

    pub(crate) const ARG_APPLY_OPERATIONS: &str = "script";

    pub(crate) const GROUP_IMAGE_OPERATIONS: &str = "group";
    pub(crate) const OP_BLUR: &str = "op_blur";
    pub(crate) const OP_BRIGHTEN: &str = "op_brighten";
    pub(crate) const OP_CONTRAST: &str = "op_contrast";
    pub(crate) const OP_CROP: &str = "op_crop";
    pub(crate) const OP_FILTER3X3: &str = "op_filter3x3";
    pub(crate) const OP_FLIP_HORIZONTAL: &str = "op_fliph";
    pub(crate) const OP_FLIP_VERTICAL: &str = "op_flipv";
    pub(crate) const OP_GRAYSCALE: &str = "op_grayscale";
    pub(crate) const OP_HUE_ROTATE: &str = "op_huerotate";
    pub(crate) const OP_INVERT: &str = "op_invert";
    pub(crate) const OP_RESIZE: &str = "op_resize";
    pub(crate) const OP_ROTATE90: &str = "op_rot90";
    pub(crate) const OP_ROTATE180: &str = "op_rot180";
    pub(crate) const OP_ROTATE270: &str = "op_rot270";
    pub(crate) const OP_UNSHARPEN: &str = "op_unsharpen";

}

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
        .arg(Arg::with_name(ARG_FORCED_OUTPUT_FORMAT)
            .short("f")
            .long("output-format")
            .value_name("FORMAT")
            .help("Force the output image format to use FORMAT, regardless of the (if any) extension of the given output file path. \
                Output formats (FORMAT values) supported: BMP, GIF, ICO, JPEG, PNG, PBM, PGM, PPM and PAM.")
            .takes_value(true))
        .arg(Arg::with_name(ARG_LICENSE)
            .long("license")
            .help("Displays the license of this piece of software (`stew`).")
            .takes_value(false))
        .arg(Arg::with_name(ARG_DEP_LICENSES)
            .long("dep-licenses")
            .help("Displays the licenses of the dependencies on which this software relies.")
            .takes_value(false))
        .arg(Arg::with_name(ARG_JPEG_ENCODING_QUALITY)
            .long("jpeg-encoding-quality")
            .help("Set the jpeg quality to QUALITY. Valid values are natural numbers from 1 up to and including 100. Will only be used when the output format is determined to be jpeg.")
            .value_name("QUALITY")
            .takes_value(true))
        .arg(Arg::with_name(ARG_PNM_ENCODING_ASCII)
            .long("pnm-encoding-ascii")
            .help("Use ascii based encoding when using a PNM image output format (pbm, pgm or ppm). Doesn't apply to 'pam' (PNM Arbitrary Map)."))
        .arg(Arg::with_name(ARG_DISABLE_AUTOMATIC_COLOR_TYPE_ADJUSTMENT)
            .long("disable-automatic-color-type-adjustment")
            .help("Some image output formats do not support the color type of the image buffer prior to encoding. By default Stew tries to adjust the color type. If this flag is provided, sic will not try to adjust the color type."))
        .arg(Arg::with_name(ARG_INPUT)
            .long("input")
            .short("i")
            .value_name("FILE_INPUT")
            .takes_value(true)
            .help("Input image path. When using this option, input piped from stdin will be ignored.")
            .required_unless_one(&[ARG_INPUT_FILE, ARG_LICENSE, ARG_DEP_LICENSES, ARG_USER_MANUAL])
            .conflicts_with_all(&[ARG_INPUT_FILE, ARG_OUTPUT_FILE]))
        .arg(Arg::with_name(ARG_OUTPUT)
            .long("output")
            .short("o")
            .value_name("FILE_OUTPUT")
            .takes_value(true)
            .help("Output image path. When using this option, output won't be piped to stdout.")
            .required_unless_one(&[ARG_OUTPUT_FILE, ARG_LICENSE, ARG_DEP_LICENSES, ARG_USER_MANUAL])
            .conflicts_with_all(&[ARG_OUTPUT_FILE, ARG_INPUT_FILE]))

        // Selective arguments for `sic`.
        .arg(Arg::with_name(ARG_USER_MANUAL)
            .long("user-manual")
            .short("H")
            .help("Displays help text for different topics such as each supported script operation. Run `sic -H index` to display a list of available topics.")
            .value_name("TOPIC")
            .takes_value(true))
        .arg(Arg::with_name(ARG_APPLY_OPERATIONS)
            .long("apply-operations")
            .short("x")
            .alias("A")
            .help(HELP_OPERATIONS_AVAILABLE)
            .value_name("OPERATIONS")
            .takes_value(true))
        .arg(Arg::with_name(ARG_INPUT_FILE)
            .help("Sets the input file")
            .value_name("INPUT_FILE")
            .required_unless_one(&[ARG_INPUT, ARG_LICENSE, ARG_DEP_LICENSES, ARG_USER_MANUAL])
            .conflicts_with_all(&[ARG_INPUT, ARG_OUTPUT])
            .index(1))
        .arg(Arg::with_name(ARG_OUTPUT_FILE)
            .help("Sets the desired output file")
            .value_name("OUTPUT_FILE")
            .required_unless_one(&[ARG_OUTPUT, ARG_LICENSE, ARG_DEP_LICENSES, ARG_USER_MANUAL])
            .conflicts_with_all(&[ARG_OUTPUT, ARG_INPUT])
            .index(2))

        .group(ArgGroup::with_name(GROUP_IMAGE_OPERATIONS)
            .args(&[
                OP_BLUR,
                OP_BRIGHTEN,
                OP_CONTRAST,
                OP_CROP,
                OP_FILTER3X3,
                OP_FLIP_HORIZONTAL,
                OP_FLIP_VERTICAL,
                OP_GRAYSCALE,
                OP_HUE_ROTATE,
                OP_INVERT,
                OP_RESIZE,
                OP_ROTATE90,
                OP_ROTATE180,
                OP_ROTATE270,
                OP_UNSHARPEN,
            ])
            .conflicts_with(ARG_APPLY_OPERATIONS)
            .multiple(true))
        .arg(Arg::with_name(OP_BLUR)
            .help("Operation: blur.")
            .long("--blur")
            .takes_value(true)
            .value_name("fp")
            .number_of_values(1)
            .multiple(true)
            .allow_hyphen_values(true))
        .arg(Arg::with_name(OP_BRIGHTEN)
            .help("Operation: brighten.")
            .long("--brighten")
            .takes_value(true)
            .value_name("int")
            .number_of_values(1)
            .multiple(true)
            .allow_hyphen_values(true))
        .arg(Arg::with_name(OP_CONTRAST)
            .help("Operation: contrast.")
            .long("--contrast")
            .takes_value(true)
            .value_name("fp")
            .number_of_values(1)
            .multiple(true)
            .allow_hyphen_values(true))
        .arg(Arg::with_name(OP_CROP)
            .help("Operation: crop.")
            .long("--crop")
            .takes_value(true)
            .value_name("uint uint uint uint")
            .number_of_values(4)
            .multiple(true))
        .arg(Arg::with_name(OP_FILTER3X3)
            .help("Operation: filter3x3.")
            .long("--filter3x3")
            .takes_value(true)
            .value_name("fp fp fp fp fp fp fp fp fp")
            .number_of_values(9)
            .multiple(true)
            .allow_hyphen_values(true))
        .arg(Arg::with_name(OP_FLIP_HORIZONTAL)
            .help("Operation: flip horizontal.")
            .long("--flip-horizontal")
            .multiple(true))
        .arg(Arg::with_name(OP_FLIP_VERTICAL)
            .help("Operation: flip vertical.")
            .long("--flip-vertical")
            .multiple(true))
        .arg(Arg::with_name(OP_GRAYSCALE)
            .help("Operation: grayscale.")
            .long("--grayscale")
            .multiple(true))
        .arg(Arg::with_name(OP_INVERT)
            .help("Operation: invert.")
            .long("--invert")
            .multiple(true))
        .arg(Arg::with_name(OP_HUE_ROTATE)
            .help("Operation: hue rotate.")
            .long("--hue-rotate")
            .takes_value(true)
            .value_name("int")
            .number_of_values(1)
            .multiple(true)
            .allow_hyphen_values(true))
        .arg(Arg::with_name(OP_RESIZE)
            .help("Operation: resize.")
            .long("--resize")
            .takes_value(true)
            .value_name("uint uint")
            .number_of_values(2)
            .multiple(true))
        .arg(Arg::with_name(OP_ROTATE90)
            .help("Operation: rotate 90 degree.")
            .long("--rotate90")
            .multiple(true))
        .arg(Arg::with_name(OP_ROTATE180)
            .help("Operation: rotate 180 degree.")
            .long("--rotate180")
            .multiple(true))
        .arg(Arg::with_name(OP_ROTATE270)
            .help("Operation: rotate 270 degree.")
            .long("--rotate270")
            .multiple(true))
        .arg(Arg::with_name(OP_UNSHARPEN)
            .help("Operation: unsharpen.")
            .long("--unsharpen")
            .takes_value(true)
            .value_name("fp int")
            .number_of_values(2)
            .multiple(true)
            .allow_hyphen_values(true))
}

// todo: below
//
// (1) figure out the related provided arguments, Complete!
// (2) figure out the order of the provided arguments, Complete!
// (3) parse each argument value (re-use sic parser where possible? make trait / functions for operation value parsing there?)
// (4) add Statement for each parsed argument to `program`
//
// todo: above

// Pair operations with the index, which can be used to find the order in which arguments were provided.
//
// usage:
//
// ```
// op_by_index!(matches, "clap arg name", OperationId::Blur)?;
// ```
macro_rules! op_with_values {
    ($matches:expr, $op_name:expr, $op_variant:expr) => {{
        let indices = $matches.indices_of($op_name);
        let values = $matches.values_of($op_name);
        let vec: Option<IndexedOps> = indices.and_then(|indices| {
            values.map(|values| {
                indices
                    .zip(values)
                    .map(|(i, v)| (i, Op::WithValues(i, $op_variant, vec![v.to_string()])))
                    .collect::<_>()
            })
        });

        vec
    }};
}

macro_rules! op_valueless {
    ($matches:expr, $op_name:expr, $op_variant:expr) => {{
        $matches.indices_of($op_name).map(|indices| {
            indices
                .map(|index| (index, Op::Bare(index, $op_variant)))
                .collect::<Vec<_>>()
        })
    }};
}

// Here any argument should not panic when invalid.
// Previously, it was allowed to panic within Config, but this is no longer the case.
pub fn build_app_config<'a>(matches: &'a ArgMatches) -> Result<Config<'a>, String> {
    let mut builder = ConfigBuilder::new();

    // next setting.
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

    // next setting.
    if let Some(format) = matches.value_of(ARG_FORCED_OUTPUT_FORMAT) {
        builder = builder.forced_output_format(format);
    }

    // next setting.
    if matches.is_present(ARG_DISABLE_AUTOMATIC_COLOR_TYPE_ADJUSTMENT) {
        builder = builder.disable_automatic_color_type_adjustment(true);
    }

    // next setting.
    if let Some(value) = matches.value_of(ARG_JPEG_ENCODING_QUALITY) {
        let requested_jpeg_quality = u8::from_str(value)
            .map_err(|_| {
                "JPEG Encoding quality should be a value between 1 and 100 (inclusive).".to_string()
            })
            .and_then(validate_jpeg_quality)?;
        builder = builder.jpeg_quality(requested_jpeg_quality);
    }

    // next setting.
    if matches.is_present(ARG_PNM_ENCODING_ASCII) {
        builder = builder.pnm_format_type(true);
    }

    // next setting.
    if let Some(path) = matches
        .value_of(ARG_OUTPUT)
        .or_else(|| matches.value_of(ARG_OUTPUT_FILE))
    {
        builder = builder.output_path(path);
    }

    // Image Operations
    // ----------------
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

    // next setting.
    if let Some(script) = matches.value_of(ARG_APPLY_OPERATIONS) {
        let program = sic_parser::parse_script(script)?;
        builder = builder.image_operations_program(program);
    } else {
        let program: Program = Vec::new();

        let mut tree: IndexTree = BTreeMap::new();
        let blur: Option<IndexedOps> = op_with_values!(matches, OP_BLUR, OperationId::Blur);
        let _ = blur.map(|nodes| tree.extend(nodes));

        let brighten = op_with_values!(matches, OP_BRIGHTEN, OperationId::Brighten);
        let _ = brighten.map(|nodes| tree.extend(nodes));

        let contrast = op_with_values!(matches, OP_CONTRAST, OperationId::Contrast);
        let _ = contrast.map(|nodes| tree.extend(nodes));

        // crop, # of arguments = 4
        let crop = op_with_values!(matches, OP_CROP, OperationId::Crop);
        tree_extend_unifiable(&mut tree, crop, 4)?;

        // filter3x3, # of arguments = 9
        let filter3x3 = op_with_values!(matches, OP_FILTER3X3, OperationId::Filter3x3);
        tree_extend_unifiable(&mut tree, filter3x3, 9)?;

        let fliph = op_valueless!(matches, OP_FLIP_HORIZONTAL, OperationId::FlipH);
        let _ = fliph.map(|nodes| tree.extend(nodes));

        let flipv = op_valueless!(matches, OP_FLIP_VERTICAL, OperationId::FlipV);
        let _ = flipv.map(|nodes| tree.extend(nodes));

        let grayscale = op_valueless!(matches, OP_GRAYSCALE, OperationId::Grayscale);
        let _ = grayscale.map(|nodes| tree.extend(nodes));

        // huerotate, # of arguments = 1
        let hue_rotate = op_with_values!(matches, OP_HUE_ROTATE, OperationId::HueRotate);
        tree_extend_unifiable(&mut tree, hue_rotate, 1)?;

        // invert
        let invert = op_valueless!(matches, OP_INVERT, OperationId::Invert);
        let _ = invert.map(|nodes| tree.extend(nodes));

        // resize, # of arguments = 2
        let resize = op_with_values!(matches, OP_RESIZE, OperationId::Resize);
        tree_extend_unifiable(&mut tree, resize, 2)?;

        // rotate90
        let rotate90 = op_valueless!(matches, OP_ROTATE90, OperationId::Rotate90);
        let _ = rotate90.map(|nodes| tree.extend(nodes));

        // rotate180
        let rotate180 = op_valueless!(matches, OP_ROTATE180, OperationId::Rotate180);
        let _ = rotate180.map(|nodes| tree.extend(nodes));

        // rotate270
        let rotate270 = op_valueless!(matches, OP_ROTATE270, OperationId::Rotate270);
        let _ = rotate270.map(|nodes| tree.extend(nodes));

        // unsharpen, # of arguments = 4
        let unsharpen = op_with_values!(matches, OP_UNSHARPEN, OperationId::Unsharpen);
        tree_extend_unifiable(&mut tree, unsharpen, 2)?;

        dbg!(tree);

        builder = builder.image_operations_program(program);
    }

    // next setting.
    if let Some(topic) = matches.value_of("user_manual") {
        builder = builder.image_operations_manual_keyword(topic);
    }

    Ok(builder.build())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sic_testing::{setup_output_path, setup_test_image};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn setup(cmd: &str) -> (ArgMatches, String) {
        let out = output(cmd);

        let command = format!("sic -i {} -o {} {}", input().as_str(), out, cmd);
        dbg!(command.clone());

        let split = command.split_ascii_whitespace().collect::<Vec<_>>();
        (cli().get_matches_from_safe(&split).unwrap(), out)
    }

    fn input() -> String {
        setup_test_image("rainbow_8x6.bmp")
            .to_string_lossy()
            .to_string()
    }

    fn output<T: Hash>(id: T) -> String {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        let likely_unique = hasher.finish();
        setup_output_path(&format!("{}.bmp", likely_unique))
            .to_string_lossy()
            .to_string()
    }

    // 1) Individual uses of: op_with_values! and op_valueless!
    // FIXME(foresterre): Quite a bit duplication currently.

    mod case_blur {
        use super::*;

        #[test]
        fn blur_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--blur 1.5");
            let blur: Option<IndexedOps> = op_with_values!(setup.0, OP_BLUR, OperationId::Blur);
            let has = blur.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Blur);
                    assert_eq!(*values, vec!["1.5".to_string()]);
                }
                _ => panic!("test err"),
            }
        }

        #[test]
        fn blur_x1_neg() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--blur -1.5");
            let blur: Option<IndexedOps> = op_with_values!(setup.0, OP_BLUR, OperationId::Blur);
            let has = blur.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Blur);
                    assert_eq!(*values, vec!["-1.5".to_string()]);
                }
                _ => panic!("test err"),
            }
        }
    }

    mod case_brighten {
        use super::*;

        #[test]
        fn brighten_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--brighten 1");
            let matches = setup.0;
            let brighten = op_with_values!(matches, OP_BRIGHTEN, OperationId::Brighten);
            let has = brighten.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Brighten);
                    assert_eq!(*values, vec!["1".to_string()]);
                }
                _ => panic!("test err"),
            }
        }

        #[test]
        fn brighten_x1_neg() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--brighten -1");
            let matches = setup.0;
            let brighten = op_with_values!(matches, OP_BRIGHTEN, OperationId::Brighten);
            let has = brighten.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Brighten);
                    assert_eq!(*values, vec!["-1".to_string()]);
                }
                _ => panic!("test err"),
            }
        }
    }

    mod case_contrast {
        #[test]
        fn contrast_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--contrast 1.5");
            let matches = setup.0;
            let contrast = op_with_values!(matches, OP_CONTRAST, OperationId::Contrast);
            let has = contrast.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Contrast);
                    assert_eq!(*values, vec!["1.5".to_string()]);
                }
                _ => panic!("test err"),
            }
        }
        use super::*;

        #[test]
        fn contrast_x1_neg() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--contrast -1.5");
            let matches = setup.0;
            let contrast = op_with_values!(matches, OP_CONTRAST, OperationId::Contrast);
            let has = contrast.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());
            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Contrast);
                    assert_eq!(*values, vec!["-1.5".to_string()]);
                }
                _ => panic!("test err"),
            }
        }
    }

    mod case_crop {
        use super::*;

        #[test]
        fn crop_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--crop 1 2 3 4");
            let matches = setup.0;
            let crop = op_with_values!(matches, OP_CROP, OperationId::Crop);
            tree_extend_unifiable(&mut tree, crop, 4).unwrap();

            let out = tree.iter().next().unwrap();

            match out.1 {
                Op::WithValues(_, id, values) => {
                    assert_eq!(*id, OperationId::Crop);
                    assert_eq!(
                        *values,
                        vec!["1", "2", "3", "4"]
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                    );
                }
                _ => panic!("test err"),
            }
        }

        #[test]
        #[should_panic]
        fn crop_x1_neg() {
            setup("--crop -1 -2 -3 -4");
        }
    }

    mod case_filter3x3 {
        use super::*;

        #[test]
        fn filter3x3_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--filter3x3 1 2 3 4 5.5 6 7 8 9");
            let matches = setup.0;
            let filter3x3 = op_with_values!(matches, OP_FILTER3X3, OperationId::Filter3x3);
            tree_extend_unifiable(&mut tree, filter3x3, 9).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Filter3x3);
            assert_eq!(
                *values,
                vec!["1", "2", "3", "4", "5.5", "6", "7", "8", "9"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn filter3x3_x1_neg() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--filter3x3 1 2 3 4 -5.5 6 7 8 9");
            let matches = setup.0;
            let filter3x3 = op_with_values!(matches, OP_FILTER3X3, OperationId::Filter3x3);
            tree_extend_unifiable(&mut tree, filter3x3, 9).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Filter3x3);
            assert_eq!(
                *values,
                vec!["1", "2", "3", "4", "-5.5", "6", "7", "8", "9"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }
    }

    mod case_flip_horizontal {
        use super::*;

        #[test]
        fn flip_horizontal_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--flip-horizontal");
            let matches = setup.0;
            let fliph = op_valueless!(matches, OP_FLIP_HORIZONTAL, OperationId::FlipH);
            let has = fliph.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };

            assert_eq!(id, OperationId::FlipH);
        }
    }

    mod case_flip_vertical {
        use super::*;

        #[test]
        fn flip_vertical_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--flip-vertical");
            let matches = setup.0;
            let flipv = op_valueless!(matches, OP_FLIP_VERTICAL, OperationId::FlipV);
            let has = flipv.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };

            assert_eq!(id, OperationId::FlipV);
        }

    }

    mod case_grayscale {
        use super::*;

        #[test]
        fn grayscale_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--grayscale");
            let matches = setup.0;
            let grayscale = op_valueless!(matches, OP_GRAYSCALE, OperationId::Grayscale);
            let has = grayscale.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };

            assert_eq!(id, OperationId::Grayscale);
        }
    }

    mod case_hue_rotate {
        use super::*;

        #[test]
        fn hue_rotate_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--hue-rotate 1");
            let matches = setup.0;
            let hue_rotate = op_with_values!(matches, OP_HUE_ROTATE, OperationId::HueRotate);
            tree_extend_unifiable(&mut tree, hue_rotate, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::HueRotate);
            assert_eq!(
                *values,
                vec!["1"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn hue_rotate_x1_neg() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--hue-rotate -1");
            let matches = setup.0;
            let hue_rotate = op_with_values!(matches, OP_HUE_ROTATE, OperationId::HueRotate);
            tree_extend_unifiable(&mut tree, hue_rotate, 1).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::HueRotate);
            assert_eq!(
                *values,
                vec!["-1"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }
    }

    mod case_invert {
        use super::*;

        #[test]
        fn invert_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let op_id = OperationId::Invert;
            let setup = setup("--invert");
            let matches = setup.0;
            let grayscale = op_valueless!(matches, OP_INVERT, op_id);
            let has = grayscale.map(|nodes| tree.extend(nodes));

            assert!(has.is_some());

            let out = tree.iter().next().unwrap();

            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };

            assert_eq!(id, op_id);
        }
    }

    mod case_resize {
        use super::*;

        #[test]
        fn resize_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--resize 1 2");
            let matches = setup.0;
            let resize = op_with_values!(matches, OP_RESIZE, OperationId::Resize);
            tree_extend_unifiable(&mut tree, resize, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Resize);
            assert_eq!(
                *values,
                vec!["1", "2"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        #[should_panic]
        fn resize_x1_neg() {
            setup("--resize -1 2");
        }

        #[test]
        #[should_panic]
        fn resize_x1_neg2() {
            setup("--resize 1 -2");
        }

        #[test]
        #[should_panic]
        fn resize_x1_neg3() {
            setup("--resize -1 -2");
        }

    }

    mod case_rotate90 {
        use super::*;

        #[test]
        fn rotate90_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let op_id = OperationId::Rotate90;
            let setup = setup("--rotate90");
            let matches = setup.0;

            let rotate90 = op_valueless!(matches, OP_ROTATE90, OperationId::Rotate90);
            let has = rotate90.map(|nodes| tree.extend(nodes));
            assert!(has.is_some());

            let out = tree.iter().next().unwrap();
            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };
            assert_eq!(id, op_id);
        }
    }

    mod case_rotate180 {
        use super::*;

        #[test]
        fn rotate180_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let op_id = OperationId::Rotate180;
            let setup = setup("--rotate180");
            let matches = setup.0;

            let rotate180 = op_valueless!(matches, OP_ROTATE180, OperationId::Rotate180);
            let has = rotate180.map(|nodes| tree.extend(nodes));
            assert!(has.is_some());

            let out = tree.iter().next().unwrap();
            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };
            assert_eq!(id, op_id);
        }
    }

    mod case_rotate270 {
        use super::*;

        #[test]
        fn rotate270_x1() {
            let mut tree: IndexTree = BTreeMap::new();
            let op_id = OperationId::Rotate270;
            let setup = setup("--rotate270");
            let matches = setup.0;

            let rotate270 = op_valueless!(matches, OP_ROTATE270, OperationId::Rotate270);
            let has = rotate270.map(|nodes| tree.extend(nodes));
            assert!(has.is_some());

            let out = tree.iter().next().unwrap();
            let id = match out {
                (_, Op::Bare(_, id)) => *id,
                _ => panic!("unexpected test error"),
            };
            assert_eq!(id, op_id);
        }
    }

    mod case_unsharpen {
        use super::*;

        #[test]
        fn unsharpen_x1_pos() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--unsharpen 1 2");
            let matches = setup.0;
            let unsharpen = op_with_values!(matches, OP_UNSHARPEN, OperationId::Unsharpen);
            tree_extend_unifiable(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Unsharpen);
            assert_eq!(
                *values,
                vec!["1", "2"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn unsharpen_x1_neg1() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--unsharpen -1.5 2");
            let matches = setup.0;
            let unsharpen = op_with_values!(matches, OP_UNSHARPEN, OperationId::Unsharpen);
            tree_extend_unifiable(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Unsharpen);
            assert_eq!(
                *values,
                vec!["-1.5", "2"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn unsharpen_x1_neg2() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--unsharpen 1.5 -2");
            let matches = setup.0;
            let unsharpen = op_with_values!(matches, OP_UNSHARPEN, OperationId::Unsharpen);
            tree_extend_unifiable(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Unsharpen);
            assert_eq!(
                *values,
                vec!["1.5", "-2"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

        #[test]
        fn unsharpen_x1_neg3() {
            let mut tree: IndexTree = BTreeMap::new();
            let setup = setup("--unsharpen -1.5 -2");
            let matches = setup.0;
            let unsharpen = op_with_values!(matches, OP_UNSHARPEN, OperationId::Unsharpen);
            tree_extend_unifiable(&mut tree, unsharpen, 2).unwrap();

            let out = tree.iter().next().unwrap();

            let (id, values) = match out {
                (_, Op::WithValues(_, id, values)) => (id, values),
                _ => panic!("unexpected test error"),
            };

            assert_eq!(*id, OperationId::Unsharpen);
            assert_eq!(
                *values,
                vec!["-1.5", "-2"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            );
        }

    }

}
