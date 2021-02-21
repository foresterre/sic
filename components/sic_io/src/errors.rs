use sic_core::errors::SicCoreError;
use sic_core::image::ImageError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SicIoError {
    #[error("{0}")]
    SicCoreError(#[from] SicCoreError),

    #[error("{0}")]
    ImageError(#[from] ImageError),

    #[error("{0}")]
    Io(std::io::Error),

    #[error("{0}")]
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
    UnknownImageIdentifier(String),

    #[error(
        "Unable to determine the image format from the file extension. The following path was given: {0}."
    )]
    UnableToDetermineImageFormatFromFileExtension(PathBuf),
}

#[derive(Debug, Error)]
pub enum FormatError {
    #[error("Unable to determine JPEG quality.")]
    JPEGQualityLevelNotSet,

    #[error("JPEG Quality should range between 1 and 100 (inclusive).")]
    JPEGQualityLevelNotInRange,

    #[error(
        "The GIF repeat value has to be either a positive integer < 65536, 'infinite' or 'never'"
    )]
    GIFRepeatInvalidValue,

    #[error("Using PNM requires the sample encoding to be set.")]
    PNMSamplingEncodingNotSet,
}
