use std::io::{Seek, Write};
use std::path::Path;

use sic_core::image::ImageEncoder;
use sic_core::{image, AnimatedImage, SicImage};

use crate::errors::{EncodingError, SicIoError};
use crate::format::DynamicEncoder;
use crate::preprocessor::Preprocessors;

pub struct SicImageEncoder {
    preprocessors: Preprocessors,
}

impl SicImageEncoder {
    pub fn new(preprocessors: Preprocessors) -> Self {
        Self { preprocessors }
    }

    pub fn encode<W: Write + Seek>(
        &self,
        image: SicImage,
        dynamic_encoder: DynamicEncoder<W>,
    ) -> Result<(), SicIoError> {
        let preprocessed_image = self
            .preprocessors
            .iter()
            .try_fold(image, |image, preprocessor| preprocessor.preprocess(image))?;

        encode(dynamic_encoder, &preprocessed_image)
    }
}

fn encode<W: Write + Seek>(encoder: DynamicEncoder<W>, image: &SicImage) -> Result<(), SicIoError> {
    match image {
        SicImage::Static(img) => encode_static_image(encoder, img),
        SicImage::Animated(img) => encode_animated_image(encoder, img),
    }
}

fn encode_static_image<W: Write + Seek>(
    encoder: DynamicEncoder<W>,
    image: &image::DynamicImage,
) -> Result<(), SicIoError> {
    encoder
        .write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color().into(),
        )
        .map_err(SicIoError::ImageError)
}

fn encode_animated_image<W: Write + Seek>(
    encoder: DynamicEncoder<W>,
    image: &AnimatedImage,
) -> Result<(), SicIoError> {
    let frames = image.collect_frames();

    match encoder {
        DynamicEncoder::Gif(mut enc) => enc.encode_frames(frames).map_err(SicIoError::ImageError),
        // Use SingleFramePreprocessor to avoid this error, by picking a single frame
        // from the animated image instead.
        enc => Err(SicIoError::Encoding(
            EncodingError::AnimatedImageUnsupported(enc.image_format()),
        )),
    }
}

pub struct EmptyPath;

impl AsRef<Path> for EmptyPath {
    fn as_ref(&self) -> &Path {
        Path::new("")
    }
}
