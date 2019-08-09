use std::collections::BTreeMap;
use std::str::FromStr;

use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches};
use sic_image_engine::engine::Program;

use crate::app::config::{validate_jpeg_quality, Config, ConfigBuilder, SelectedLicenses};
use crate::get_tool_name;

const HELP_OPERATIONS_AVAILABLE: &str = include_str!("../../docs/cli_help_script.txt");

pub(crate) const ARG_APPLY_OPERATIONS: &str = "script";

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
use crate::app::cli::OperationId::FlipH;
use arg_names::*;
use std::convert::TryFrom;
use std::ops::IndexMut;

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
            ])
            .conflicts_with(ARG_APPLY_OPERATIONS)
            .multiple(true))
        .arg(Arg::with_name(OP_BLUR)
            .help("Operation: blur.")
            .long("--blur")
            .takes_value(true)
            .value_name("fp")
            .number_of_values(1)
            .multiple(true))
        .arg(Arg::with_name(OP_BRIGHTEN)
            .help("Operation: brighten.")
            .long("--brighten")
            .takes_value(true)
            .value_name("int")
            .number_of_values(1)
            .multiple(true))
        .arg(Arg::with_name(OP_CONTRAST)
            .help("Operation: contrast.")
            .long("--contrast")
            .takes_value(true)
            .value_name("fp")
            .number_of_values(1)
            .multiple(true))
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
            .multiple(true))
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
            .value_name("by")
            .number_of_values(1)
            .multiple(true))
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
            .multiple(true))
}

// todo: below
//
// (1) figure out the related provided arguments, Complete!
// (2) figure out the order of the provided arguments, Complete!
// (3) parse each argument value (re-use sic parser where possible? make trait / functions for operation value parsing there?)
// (4) add Statement for each parsed argument to `program`
//
// todo: above

#[derive(Debug, Copy, Clone)]
enum OperationId {
    Blur,
    Brighten,
    Contrast,
    Crop,
    Filter3x3,
    FlipH,
    FlipV,
    Grayscale,
    HueRotate,
    Invert,
    Resize,
    Rotate90,
    Rotate180,
    Rotate270,
    Unsharpen,
}

type Index = usize;

/// Represents an image operation which was obtained from CLI image operation commands.
///
/// Index := Position of an argument value.
/// OperationId := Type of operation we are dealing with, e.g. Blur or Rotate90.
/// Vec<String> := Vector of unverified string arguments; initially with multiple arguments
///              we will receive multiple [Op] as Clap provides multiple arguments individually.
///              The multiple Op will be unified where applicable.
///
/// The operation values are not parsed yet within this structure.
#[derive(Debug, Clone)]
enum Op {
    WithValues(Index, OperationId, Vec<String>),
    Bare(Index, OperationId),
}

/// An IndexTree represents the decided order in which operations should be applied.
/// Because the underlying data structure is a BTree, we can conveniently add
/// [Op] by their provided indices.
/// Note that unified [Op] could be given any index of the values they were originally unified
/// from.
type IndexTree = BTreeMap<Index, Op>;

/// Nodes which contain tuples with arity 2, where the first value is the Index,
/// and the second value is an Operation
type IndexedOps = Vec<(Index, Op)>;

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

/// This tree extension function should be used if an image operation cli arguments takes more than 1
/// value.
/// Clap provides all values for an argument as a linear container (like a vector with values).
/// However, we don't want to allow --crop 0 0 --crop 1 1, but we do want --crop 0 0 1 1.
/// The resulting value we get from Clap do not differ for the two examples above.
/// Both will give a container along the lines of vec!("0", "0", "1", "1").
/// To ensure we only allow, for an image operation cli arguments which takes 4 values, 4 values
/// all at once, we use the indices of the values (which Clap does provide) to check whether they
/// are incrementally correct (i.e. index_{i+1} = index_{i} + 1).
///
/// Arguments:
///     tree: The BTree which we will extend with nodes for an operation type.
///     op_values: If the operation is not provided, we don't extend the tree. If it is,
///                we'll use the separate nodes and try to unify them into nodes which contain
///                each `size` values.
///     size: The amount of values which an image operation requires.
///
fn tree_extend_unifiable(
    tree: &mut IndexTree,
    op_values: Option<IndexedOps>,
    size: usize,
) -> Result<(), String> {
    match op_values {
        Some(nodes) => {
            let unified = unify_multiple_values(nodes, size)?;
            tree.extend(unified);
            Ok(())
        }
        // No image operation cli argument of this type given
        None => Ok(()),
    }
}

/// Chunk provided values and try to unify each chunk to a single [Op].
/// Requires each chunk to be of the size of the `size` argument.
fn unify_multiple_values(container: IndexedOps, size: usize) -> Result<IndexedOps, String> {
    assert_ne!(size, 0);

    let chunks = container.chunks(size).clone();
    let mut vec: IndexedOps = Vec::new();

    for chunk in chunks {
        if chunk.len() != size {
            return Err(format!(
                "Unification of multi valued argument(s) failed: arguments could't be \
                 partitioned in correct chunk sizes. Length of chunk: {}",
                chunk.len()
            ));
        }

        let unified_chunk = unify_chunk(chunk, None, size);
        vec.push(unified_chunk?);
    }

    Ok(vec)
}

const FAILED_UNIFICATION_MESSAGE: &str =
    "Unification of multi valued argument(s) failed: \
     When using an image operation cli argument which requires n values, \
     all values should be provided at once. For example, `--crop` takes 4 values \
     so, n=4. Now, `--crop 0 0 1 1` would be valid, but `--crop 0 0 --crop 1 1` would not.";

/// Try to unify a chunk of values to a single value.
fn unify_chunk(
    left: &[(usize, Op)],
    last: Option<(usize, Op)>,
    size: usize,
) -> Result<(usize, Op), String> {
    // stop: complete unification of the chunk
    if left.is_empty() {
        match last {
            Some(ret) => Ok(ret),
            None => Err(FAILED_UNIFICATION_MESSAGE.to_string()),
        }
    } else {
        // continue (left.len() > 0):
        let current: (usize, Op) = left[0].clone();

        match last {
            Some(node) => {
                // is it incremental based on the last index <- (index, _) ?
                if (node.0 + 1) == current.0 {
                    // Here we create an [IndexedOpNode] tuple.
                    // with (index, op),
                    // `index` will be the index of the current node (i.e. de new last)
                    // `op` will be the node in unification, which is based on the last node,
                    // but here we add an un-parsed string value to the value vec.
                    let unified_op = node.1;
                    let current_op = current.1;
                    match (unified_op, current_op) {
                        (
                            Op::WithValues(index, id, mut values),
                            Op::WithValues(index2, id2, values2),
                        ) => {
                            values.extend(values2);

                            // the [Op] consist of:
                            // (
                            //   Index := update the index to the current (this could also be the first
                            //            index, or any node which was unified with the `last` node,
                            //            however we choose to update the index with the last, so the
                            //            index of the IndexedOpNode and Op will be consistent
                            //   OperationId := operation id, again from the first added node
                            //   Vec<String> := we add the value (new) of the `current` [Op] to the values
                            //                  already part of the unified [Op]; i.e. we extend
                            //                  the unified [Op] with a new provided image operation
                            //                  argument
                            // )
                            let updated_op = Op::WithValues(current.0, id, values);

                            // Package it as an [IndexedOpNode]
                            let new_last = (current.0, updated_op);

                            unify_chunk(left[1..].as_ref(), Some(new_last), size)
                        }
                        _ => Err("Can't unify bare values".to_string()),
                    }
                } else {
                    Err(FAILED_UNIFICATION_MESSAGE.to_string())
                }
            }
            // first value
            None => unify_chunk(left[1..].as_ref(), Some(current), size),
        }
    }
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

        // huerotate, # of arguments = 2
        let hue_rotate = op_with_values!(matches, OP_HUE_ROTATE, OperationId::HueRotate);
        tree_extend_unifiable(&mut tree, hue_rotate, 2)?;

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

fn make_program_from_cli_image_operations(matches: &ArgMatches) {}

#[cfg(test)]
mod tests {
    use super::*;

    mod cli_arg_values_unification {
        use super::*;
        use std::collections::BTreeMap;

        #[test]
        fn tree_extend_unifiable_n1() {
            let mut tree: IndexTree = BTreeMap::new();
            assert!(tree.is_empty());

            let blur: IndexedOps = vec![(
                0,
                Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
            )];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 1);

            assert!(res.is_ok());
            assert_eq!(tree.len(), 1);
        }

        #[test]
        fn tree_extend_unifiable_n4() {
            let mut tree: IndexTree = BTreeMap::new();

            assert!(tree.is_empty());

            // the `tree_extend_unifiable` function doesn't care about the operation id.
            // Nor about the Index within Op(index,..,..).
            let blur: IndexedOps = vec![
                (
                    0,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    1,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    3,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
            ];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 4);

            assert!(res.is_ok());
            assert_eq!(tree.len(), 1);
        }

        #[test]
        fn tree_extend_unifiable_n2_multiple() {
            let mut tree: IndexTree = BTreeMap::new();

            assert!(tree.is_empty());

            // the `tree_extend_unifiable` function doesn't care about the operation id.
            // Nor about the Index within Op(index,..,..).
            let blur: IndexedOps = vec![
                (
                    0,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    1,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    3,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
            ];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 2);

            assert!(res.is_ok());
            assert_eq!(tree.len(), 2);
        }

        #[test]
        fn tree_extend_unifiable_n4_fail() {
            let mut tree: IndexTree = BTreeMap::new();

            let blur: IndexedOps = vec![
                (
                    0,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    3,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
            ];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 4);

            assert!(res.is_err())
        }

        #[test]
        fn tree_extend_unifiable_n4_too_few_provided() {
            let mut tree: IndexTree = BTreeMap::new();

            let blur: IndexedOps = vec![
                (
                    0,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    1,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
            ];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 4);

            assert!(res.is_err())
        }

        #[test]
        fn tree_extend_unifiable_n4_too_many_provided() {
            let mut tree: IndexTree = BTreeMap::new();

            let blur: IndexedOps = vec![
                (
                    0,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    1,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    2,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    3,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
                (
                    4,
                    Op::WithValues(0, OperationId::Blur, vec!["1".to_string()]),
                ),
            ];
            let res = tree_extend_unifiable(&mut tree, Some(blur), 4);

            assert!(res.is_err())
        }
    }
}
