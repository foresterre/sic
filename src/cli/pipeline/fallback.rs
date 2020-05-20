use std::path::Path;

use sic_core::image::error::{ImageFormatHint, UnsupportedError};
use sic_core::image::{ImageError, ImageFormat, ImageOutputFormat};
use sic_io::errors::SicIoError;

pub(crate) fn guess_output_by_identifier(id: &str) -> Result<ImageOutputFormat, SicIoError> {
    // HACK: image crate doesn't use identifiers, so we'll use an extension as identifier
    guess_output_by_path(Path::new(&format!("0.{}", id)))
}

pub(crate) fn guess_output_by_path<P: AsRef<Path>>(
    path: P,
) -> Result<ImageOutputFormat, SicIoError> {
    ImageFormat::from_path(path)
        .and_then(into_image_output_format)
        .map_err(SicIoError::ImageError)
}

fn into_image_output_format(format: ImageFormat) -> Result<ImageOutputFormat, ImageError> {
    let out = Into::<ImageOutputFormat>::into(format);

    // Assuming we'll never hit the __NonExhaustive marker workaround by the image crate
    match out {
        ImageOutputFormat::Unsupported(name) => Err(ImageError::Unsupported(
            UnsupportedError::from(ImageFormatHint::Name(name)),
        )),
        f => Ok(f),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    ide!();

    #[parameterized(input = {
        "jpg",
        "jpeg",
        "png",
        "gif",
        "bmp",
        "ico",
    }, expected = {
        ImageOutputFormat::Jpeg(75),
        ImageOutputFormat::Jpeg(75),
        ImageOutputFormat::Png,
        ImageOutputFormat::Gif,
        ImageOutputFormat::Bmp,
        ImageOutputFormat::Ico,
    })]
    fn formats_by_image_crate_ok(input: &str, expected: ImageOutputFormat) {
        let by_id = guess_output_by_identifier(input).unwrap();
        let by_path = guess_output_by_path(format!("my_file_name.{}", input)).unwrap();

        assert_eq!(by_id, by_path);
        assert_eq!(by_id, expected);
    }
    #[parameterized(input = {
        "dds",
        "hdr",
        "docx",
        "",
        "😀",
    })]
    fn formats_by_image_crate_err(input: &str) {
        let by_id = guess_output_by_identifier(input);
        let by_path = guess_output_by_path(format!("my_file_name.{}", input));

        assert!(by_id.is_err());
        assert!(by_path.is_err());
    }

    #[test]
    fn format_by_image_crate_err_no_ext() {
        let by_path = guess_output_by_path("my_file_name");

        assert!(by_path.is_err());
    }
}
