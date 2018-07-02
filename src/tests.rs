extern crate image;

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


