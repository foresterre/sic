use crate::format;
use sic_core::image::ImageError;
use sic_core::SicCoreError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SicIoError {
    #[error(transparent)]
    SicCoreError(#[from] SicCoreError),

    #[error(transparent)]
    ImageError(#[from] ImageError),

    #[error(transparent)]
    Io(std::io::Error),

    #[error(transparent)]
    FormatError(FormatError),

    #[error(
        "An input image should be given by providing a path using the input argument or by \
         piping an image to the stdin."
    )]
    NoInputImage,

    #[error("Unable to extract frame {0} from the (animated) image; please use a frame index between 0 and {1}.")]
    NoSuchFrame(usize, usize),

    #[error("An animated image was expected, but a static image was given")]
    NotAnAnimatedImage,

    #[error(
        "No supported image output format was found. The following identifier was provided: {0}."
    )]
    UnknownImageFormat(String),

    #[error(
        "Unable to determine the image format from the file extension. The following path was given: {0}."
    )]
    UnknownImageFormatFromFileExtension(PathBuf),
}

#[cfg(test)]
impl PartialEq for SicIoError {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

#[derive(Debug, Error)]
pub enum FormatError {
    #[error(transparent)]
    JPEGQuality(format::jpeg::JpegQualityError),

    #[error(
        "The GIF repeat value has to be either a positive integer < 65536, 'infinite' or 'never'"
    )]
    GIFRepeatInvalidValue,
}
