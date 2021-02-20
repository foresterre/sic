use thiserror::Error;

#[derive(Debug, Error)]
pub enum SicCoreError {
    #[error("{0}")]
    ImageError(image::error::ImageError),

    #[error("Invalid frame index: index (is {index}) should be < len (is {len}) ")]
    InvalidFrameIndex { index: usize, len: usize },

    #[error("A static image was required, but an animated image was given")]
    RequiresStaticImage,
}
