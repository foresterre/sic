use thiserror::Error;

#[derive(Debug, Error)]
pub enum SicCoreError {
    #[error("{0}")]
    ImageError(image::error::ImageError),

    #[error("A static image was required, but an animated image was given")]
    RequiresStaticImage,
}
