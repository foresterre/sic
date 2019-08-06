use std::io::Read;
use std::path::{Path, PathBuf};

use sic_core::image;

use sic_lib::app::cli::{build_app_config, cli as get_app}; // build_app_config
use sic_lib::app::run_mode::run;
use std::process::{Child, Command};

// Wish list for Rust tests: parameterized tests
// Probably can be done with macro's too.

// copied from sic_lib::processor::mod_test_includes
// I preferred to not make that module public (2018-11-28)
// Originally named: setup_test_image.
fn setup_input_path(test_image_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join(test_image_path)
}

// copied from sic_lib::processor::mod_test_includes
// I preferred to not make that module public (2018-11-28)
fn setup_output_path(test_output_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(test_output_path)
}

fn path_buf_str(pb: &PathBuf) -> &str {
    pb.to_str().unwrap()
}

// copied from sic_lib::processor::mod_test_includes
// I preferred to not make that module public (2018-11-28)
fn clean_up_output_path(test_output_path: &str) {
    std::fs::remove_file(setup_output_path(test_output_path))
        .expect("Unable to remove file after test.");
}

// by image header
fn is_image_format(output_path: &str, hope: image::ImageFormat) -> bool {
    let mut file = std::fs::File::open(setup_output_path(output_path))
        .expect("Failed to find (produced) test image.");

    let mut bytes = vec![];

    file.read_to_end(&mut bytes)
        .expect("Failed to read (produced) test image.");

    hope == image::guess_format(&bytes).expect("Format could not be guessed.")
}

// convert_to_X_by_extension
// BMP, GIF, JPEG, PNG, ICO, PBM, PGM, PPM, PAM

#[test]
fn convert_to_bmp_by_extension() {
    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path("out_01.bmp");

    let args = vec![
        "sic",
        our_input.to_str().unwrap(),
        path_buf_str(&our_output),
    ];
    let matches = get_app().get_matches_from(args);

    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::BMP
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_gif_by_extension() {
    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path("out_01.gif");

    let args = vec!["sic", path_buf_str(&our_input), path_buf_str(&our_output)];
    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::GIF
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_jpg_by_extension() {
    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path("out_01.jpg");

    let args = vec!["sic", path_buf_str(&our_input), path_buf_str(&our_output)];
    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::JPEG
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_jpeg_by_extension() {
    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path("out_01.jpeg");

    let args = vec!["sic", path_buf_str(&our_input), path_buf_str(&our_output)];
    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::JPEG
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_png_by_extension() {
    let our_input = setup_input_path("rainbow_8x6.bmp");
    let our_output = setup_output_path("out_01.png");

    let args = vec!["sic", path_buf_str(&our_input), path_buf_str(&our_output)];
    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::PNG
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_ico_by_extension() {
    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path("out_01.ico");

    let args = vec!["sic", path_buf_str(&our_input), path_buf_str(&our_output)];
    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::ICO
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_pbm_by_extension() {
    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path("out_01.pbm");

    let args = vec!["sic", path_buf_str(&our_input), path_buf_str(&our_output)];
    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::PNM
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_pgm_by_extension() {
    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path("out_01.pgm");

    let args = vec!["sic", path_buf_str(&our_input), path_buf_str(&our_output)];
    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::PNM
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_ppm_by_extension() {
    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path("out_01.ppm");

    let args = vec!["sic", path_buf_str(&our_input), path_buf_str(&our_output)];
    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::PNM
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_pam_by_extension() {
    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path("out_01.pam");

    let args = vec!["sic", path_buf_str(&our_input), path_buf_str(&our_output)];
    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::PNM
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

fn convert_to_x_by_ff_args<'a>(
    which: &'a str,
    input: &'a PathBuf,
    output: &'a PathBuf,
) -> Vec<&'a str> {
    vec![
        "sic",
        "--output-format",
        which,
        path_buf_str(&input),
        path_buf_str(&output),
    ]
}

#[test]
fn convert_to_bmp_by_ff() {
    let which = "bmp";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_01_{}", which));

    let args = convert_to_x_by_ff_args(which, &our_input, &our_output);

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::BMP
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_gif_by_ff() {
    let which = "gif";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_01_{}", which));

    let args = convert_to_x_by_ff_args(which, &our_input, &our_output);

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::GIF
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_ico_by_ff() {
    let which = "ico";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_01_{}", which));

    let args = convert_to_x_by_ff_args(which, &our_input, &our_output);

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::ICO
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_jpg_by_ff() {
    let which = "jpg";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_01_{}", which));

    let args = convert_to_x_by_ff_args(which, &our_input, &our_output);

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::JPEG
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_jpeg_by_ff() {
    let which = "jpeg";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_01_{}", which));

    let args = convert_to_x_by_ff_args(which, &our_input, &our_output);

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::JPEG
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_png_by_ff() {
    let which = "png";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_01_{}", which));

    let args = convert_to_x_by_ff_args(which, &our_input, &our_output);

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::PNG
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_pbm_by_ff() {
    let which = "pbm";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_01_{}", which));

    let args = convert_to_x_by_ff_args(which, &our_input, &our_output);

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::PNM
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_pgm_by_ff() {
    let which = "pgm";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_01_{}", which));

    let args = convert_to_x_by_ff_args(which, &our_input, &our_output);

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::PNM
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_ppm_by_ff() {
    let which = "ppm";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_01_{}", which));

    let args = convert_to_x_by_ff_args(which, &our_input, &our_output);

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::PNM
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_to_pam_by_ff() {
    let which = "pam";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_01_{}", which));

    let args = convert_to_x_by_ff_args(which, &our_input, &our_output);

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::PNM
    ));

    clean_up_output_path(path_buf_str(&our_output));
}

// Try to determine that PBM, PGM, PPM in ascii mode (P1, P2, P3 resp.) are ascii encoded
// and if they are 'binary' encoded (P4, P5, P6), they are obviously not ascii encoded.

fn read_file_to_bytes<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut f = std::fs::File::open(path).unwrap();
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).unwrap();

    buffer
}

fn guess_is_ascii_encoded(input: &[u8]) -> bool {
    // The character P in ascii encoding
    let is_ascii_p = |c: u8| c == 0x50;

    // P1, P2, P3 are ascii
    // Checks for numbers 1, 2, 3, binary known as: 0x31, 0x32, 0x33 respectively
    let is_ascii_magic_number = |c| c == 0x31 || c == 0x32 || c == 0x33;

    let mut iter = input.iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();

    // check also to be sure that every character in the file can be ascii encoded
    is_ascii_p(*first) && is_ascii_magic_number(*second) && iter.all(|c| *c <= 0x7F)
}

#[test]
fn convert_pbm_ascii() {
    let which = "pbm";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_02a_{}", which));

    let args = vec![
        "sic",
        "--pnm-encoding-ascii",
        "--output-format",
        which,
        path_buf_str(&our_input),
        path_buf_str(&our_output),
    ];

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());

    // read file contents
    let contents = read_file_to_bytes(path_buf_str(&our_output));

    // is it just ascii?
    assert!(guess_is_ascii_encoded(&contents));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_pgm_ascii() {
    let which = "pgm";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_02a_{}", which));

    let args = vec![
        "sic",
        "--pnm-encoding-ascii",
        "--output-format",
        which,
        path_buf_str(&our_input),
        path_buf_str(&our_output),
    ];

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());

    // read file contents
    let contents = read_file_to_bytes(path_buf_str(&our_output));

    // is it just ascii?
    assert!(guess_is_ascii_encoded(&contents));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_ppm_ascii() {
    let which = "ppm";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_02a_{}", which));

    let args = vec![
        "sic",
        "--pnm-encoding-ascii",
        "--output-format",
        which,
        path_buf_str(&our_input),
        path_buf_str(&our_output),
    ];

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());

    // read file contents
    let contents = read_file_to_bytes(path_buf_str(&our_output));

    // is it just ascii?
    assert!(guess_is_ascii_encoded(&contents));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_pbm_not_ascii() {
    let which = "pbm";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_02b_{}", which));

    let args = vec![
        "sic",
        "--output-format",
        which,
        path_buf_str(&our_input),
        path_buf_str(&our_output),
    ];

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());

    // read file contents
    let contents = read_file_to_bytes(path_buf_str(&our_output));

    // is it just ascii?
    assert!(!guess_is_ascii_encoded(&contents));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_pgm_not_ascii() {
    let which = "pgm";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_02b_{}", which));

    let args = vec![
        "sic",
        "--output-format",
        which,
        path_buf_str(&our_input),
        path_buf_str(&our_output),
    ];

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());

    // read file contents
    let contents = read_file_to_bytes(path_buf_str(&our_output));

    // is it just ascii?
    assert!(!guess_is_ascii_encoded(&contents));

    clean_up_output_path(path_buf_str(&our_output));
}

#[test]
fn convert_ppm_not_ascii() {
    let which = "ppm";

    let our_input = setup_input_path("palette_4x4.png");
    let our_output = setup_output_path(&format!("out_02b_{}", which));

    let args = vec![
        "sic",
        "--output-format",
        which,
        path_buf_str(&our_input),
        path_buf_str(&our_output),
    ];

    let matches = get_app().get_matches_from(args);
    let complete = run(&matches, vec![], &build_app_config(&matches).unwrap());

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());

    // read file contents
    let contents = read_file_to_bytes(path_buf_str(&our_output));

    // is it just ascii?
    assert!(!guess_is_ascii_encoded(&contents));

    clean_up_output_path(path_buf_str(&our_output));
}

// JPEG different quality
// Currently just tested by default 80 ?!= not(80)
// Can we do better?

#[test]
fn convert_jpeg_quality_different() {
    let which = "jpeg";

    let our_input = setup_input_path("palette_4x4.png");
    let out1 = setup_output_path("out_02_jpeg_1.jpeg");
    let out2 = setup_output_path("out_02_jpeg_2.jpeg");

    let args1 = vec![
        "sic",
        "--output-format",
        which,
        path_buf_str(&our_input),
        path_buf_str(&out1),
    ];

    let args2 = vec![
        "sic",
        "--jpeg-encoding-quality",
        "81",
        "--output-format",
        which,
        path_buf_str(&our_input),
        path_buf_str(&out2),
    ];

    let matches1 = get_app().get_matches_from(args1);
    let complete1 = run(&matches1, vec![], &build_app_config(&matches1).unwrap());

    let matches2 = get_app().get_matches_from(args2);
    let complete2 = run(&matches2, vec![], &build_app_config(&matches2).unwrap());

    assert_eq!((Ok(()), Ok(())), (complete1, complete2));
    assert!(out1.exists() && out2.exists());

    // read file contents
    let contents1 = read_file_to_bytes(path_buf_str(&out1));
    let contents2 = read_file_to_bytes(path_buf_str(&out2));

    assert_eq!(contents1, contents1);
    assert_ne!(contents1, contents2);

    clean_up_output_path(path_buf_str(&out1));
    clean_up_output_path(path_buf_str(&out2));
}
