use std::io::Read;
use std::path::{Path, PathBuf};

use sic_lib::{get_app, run};

// copied from sic_lib::processor::mod_test_includes
// I prefered to not make that module public (2018-11-28)
// Originally named: setup_test_image.
fn setup_input_path(test_image_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join(test_image_path)
}

// copied from sic_lib::processor::mod_test_includes
// I prefered to not make that module public (2018-11-28)
fn setup_output_path(test_output_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(test_output_path)
}

fn path_buf_str<'a>(pb: &'a PathBuf) -> &'a str {
    pb.to_str().unwrap()
}

// copied from sic_lib::processor::mod_test_includes
// I prefered to not make that module public (2018-11-28)
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
    let complete = run(matches);

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

    let args = vec![
        "sic",
        our_input.to_str().unwrap(),
        path_buf_str(&our_output),
    ];
    let matches = get_app().get_matches_from(args);
    let complete = run(matches);

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

    let args = vec![
        "sic",
        our_input.to_str().unwrap(),
        path_buf_str(&our_output),
    ];
    let matches = get_app().get_matches_from(args);
    let complete = run(matches);

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

    let args = vec![
        "sic",
        our_input.to_str().unwrap(),
        path_buf_str(&our_output),
    ];
    let matches = get_app().get_matches_from(args);
    let complete = run(matches);

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

    let args = vec![
        "sic",
        our_input.to_str().unwrap(),
        path_buf_str(&our_output),
    ];
    let matches = get_app().get_matches_from(args);
    let complete = run(matches);

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

    let args = vec![
        "sic",
        our_input.to_str().unwrap(),
        path_buf_str(&our_output),
    ];
    let matches = get_app().get_matches_from(args);
    let complete = run(matches);

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

    let args = vec![
        "sic",
        our_input.to_str().unwrap(),
        path_buf_str(&our_output),
    ];
    let matches = get_app().get_matches_from(args);
    let complete = run(matches);

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

    let args = vec![
        "sic",
        our_input.to_str().unwrap(),
        path_buf_str(&our_output),
    ];
    let matches = get_app().get_matches_from(args);
    let complete = run(matches);

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

    let args = vec![
        "sic",
        our_input.to_str().unwrap(),
        path_buf_str(&our_output),
    ];
    let matches = get_app().get_matches_from(args);
    let complete = run(matches);

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

    let args = vec![
        "sic",
        our_input.to_str().unwrap(),
        path_buf_str(&our_output),
    ];
    let matches = get_app().get_matches_from(args);
    let complete = run(matches);

    assert_eq!(Ok(()), complete);
    assert!(our_output.exists());
    assert!(is_image_format(
        path_buf_str(&our_output),
        image::ImageFormat::PNM
    ));

    clean_up_output_path(path_buf_str(&our_output));
}
