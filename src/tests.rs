extern crate image;

use super::*;

/// which_image_format

// general
#[test]
fn image_format_from_str_from_upper_case() {
    assert_eq!(
        Ok(image::ImageOutputFormat::GIF),
        image_format_from_str("GIF")
    );
}

#[test]
fn image_format_from_str_from_mixed_case() {
    assert_eq!(
        Ok(image::ImageOutputFormat::GIF),
        image_format_from_str("GiF")
    );
}

#[test]
fn image_format_from_str_none() {
    assert_eq!(
        Err("Image format unsupported.".to_string()),
        image_format_from_str("boop")
    );
}

// BMP
#[test]
fn image_format_from_str_bmp() {
    assert_eq!(
        Ok(image::ImageOutputFormat::BMP),
        image_format_from_str("bmp")
    );
}

// GIF
#[test]
fn image_format_from_str_gif() {
    assert_eq!(
        Ok(image::ImageOutputFormat::GIF),
        image_format_from_str("gif")
    );
}

// ICO
#[test]
fn image_format_from_str_ico() {
    assert_eq!(
        Ok(image::ImageOutputFormat::ICO),
        image_format_from_str("ico")
    );
}

// JPG/JPEG
#[test]
fn image_format_from_str_jpeg() {
    assert_eq!(
        Ok(image::ImageOutputFormat::JPEG(80)),
        image_format_from_str("jpeg")
    );
}

#[test]
fn image_format_from_str_jpg() {
    assert_eq!(
        Ok(image::ImageOutputFormat::JPEG(80)),
        image_format_from_str("jpg")
    );
}

// PNG
#[test]
fn image_format_from_str_png() {
    assert_eq!(
        Ok(image::ImageOutputFormat::PNG),
        image_format_from_str("png")
    );
}

// PPM
#[test]
fn image_format_from_str_ppm() {
    assert_eq!(
        Ok(image::ImageOutputFormat::PNM(
            image::pnm::PNMSubtype::Pixmap(image::pnm::SampleEncoding::Binary)
        )),
        image_format_from_str("ppm")
    );
}
