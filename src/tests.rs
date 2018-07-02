extern crate image;

use super::*;

/// which_image_format

// general
#[test]
fn image_format_from_str_from_upper_case() {
    assert_eq!(
        Some(image::ImageOutputFormat::GIF),
        image_format_from_str("GIF")
    );
}

#[test]
fn image_format_from_str_from_mixed_case() {
    assert_eq!(
        Some(image::ImageOutputFormat::GIF),
        image_format_from_str("GiF")
    );
}

// BMP
#[test]
fn image_format_from_str_bmp() {
    assert_eq!(
        Some(image::ImageOutputFormat::BMP),
        image_format_from_str("bmp")
    );
}

// GIF
#[test]
fn image_format_from_str_gif() {
    assert_eq!(
        Some(image::ImageOutputFormat::GIF),
        image_format_from_str("gif")
    );
}

// ICO
#[test]
fn image_format_from_str_ico() {
    assert_eq!(
        Some(image::ImageOutputFormat::ICO),
        image_format_from_str("ico")
    );
}

// JPG/JPEG
#[test]
fn image_format_from_str_jpeg() {
    assert_eq!(
        Some(image::ImageOutputFormat::JPEG(80)),
        image_format_from_str("jpeg")
    );
}

#[test]
fn image_format_from_str_jpg() {
    assert_eq!(
        Some(image::ImageOutputFormat::JPEG(80)),
        image_format_from_str("jpg")
    );
}

// PNG
#[test]
fn image_format_from_str_png() {
    assert_eq!(
        Some(image::ImageOutputFormat::PNG),
        image_format_from_str("png")
    );
}

// PPM
#[test]
fn image_format_from_str_ppm() {
    assert_eq!(
        Some(image::ImageOutputFormat::PNM(
            image::pnm::PNMSubtype::Pixmap(image::pnm::SampleEncoding::Binary)
        )),
        image_format_from_str("ppm")
    );
}

/// determine_format_by_extension
#[test]
fn determine_format_by_extension_ok_path() {
    assert_eq!(
        Some(image::ImageOutputFormat::PNG),
        determine_format_by_extension("C:/users/some/path.png")
    );
}

#[test]
fn determine_format_by_extension_test_ok_file() {
    assert_eq!(
        Some(image::ImageOutputFormat::PNG),
        determine_format_by_extension("path.png")
    );
}

#[test]
fn determine_format_by_extension_test_no_ext_path() {
    assert_eq!(None, determine_format_by_extension("C:/users/some/png"));
}

#[test]
fn determine_format_by_extension_test_no_ext_file() {
    assert_eq!(None, determine_format_by_extension("png"));
}

/// get_extension
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
