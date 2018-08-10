extern crate image;

use std::fs::File;
use std::path::Path;

/// Determines the appropriate ImageOutputFormat based on a &str.
pub fn image_format_from_str(format: &str) -> Result<image::ImageOutputFormat, String> {
    let format_in_lower_case: &str = &*format.to_string().to_lowercase();

    match format_in_lower_case {
        "bmp" => Ok(image::ImageOutputFormat::BMP),
        "gif" => Ok(image::ImageOutputFormat::GIF),
        "ico" => Ok(image::ImageOutputFormat::ICO),
        "jpeg" | "jpg" => Ok(image::ImageOutputFormat::JPEG(80)),
        "png" => Ok(image::ImageOutputFormat::PNG),
        "ppm" => Ok(image::ImageOutputFormat::PNM(
            image::pnm::PNMSubtype::Pixmap(image::pnm::SampleEncoding::Binary),
        )),
        _ => Err("Image format unsupported.".to_string()),
    }
}

/// Converts an image (`input`) to a certain `format` regardless of the extension of the `output` file path.
pub fn convert_image_forced(
    img: &image::DynamicImage,
    output: &str,
    format: &str,
) -> Result<(), String> {
    image_format_from_str(format)
        .map_err(|err| err.to_string())
        .and_then(|image_format| {
            let mut out = File::create(&Path::new(output)).map_err(|err| err.to_string())?;

            img.write_to(&mut out, image_format)
                .map_err(|err| err.to_string())
        })
}

/// Converts an image (`input`) to a certain `format` based on the extension of the `output` file path.
pub fn convert_image_unforced(img: &image::DynamicImage, output: &str) -> Result<(), String> {
    img.save(output).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

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

}
