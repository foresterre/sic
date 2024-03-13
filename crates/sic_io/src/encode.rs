use std::io::{Seek, Write};
use std::path::Path;

use sic_core::image::ImageEncoder;
use sic_core::{image, AnimatedImage, SicImage};

use crate::errors::SicIoError;
use crate::format::gif::RepeatAnimation;
use crate::format::DynamicEncoder;
use crate::preprocessor::color_type::{ColorTypeAdjustment, ColorTypePreprocessor};
use crate::preprocessor::Preprocess;

pub struct SicImageEncoder {
    pub adjust_color_type: ColorTypeAdjustment,
    pub gif_repeat: RepeatAnimation,
}

impl SicImageEncoder {
    pub fn new(adjust_color_type: ColorTypeAdjustment, gif_repeat: RepeatAnimation) -> Self {
        Self {
            adjust_color_type,
            gif_repeat,
        }
    }

    pub fn encode<W: Write + Seek>(
        &self,
        image: SicImage,
        dynamic_encoder: DynamicEncoder<W>,
    ) -> Result<(), SicIoError> {
        if self.adjust_color_type.is_enabled() {
            let image = ColorTypePreprocessor::new(dynamic_encoder.format()).preprocess(image)?;
            encode(dynamic_encoder, &image)
        } else {
            encode(dynamic_encoder, &image)
        }
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
    let fmt = encoder.format();

    match encoder {
        DynamicEncoder::Gif(mut enc) => enc.encode_frames(frames).map_err(SicIoError::ImageError),
        enc => {
            eprintln!("WARN: Unable to encode animated image buffer with format '{:?}': encoding first frame only", fmt);
            let image = AnimatedImage::from_frames(frames).try_into_static_image(0)?;
            encode_static_image(enc, &image)
        }
    }
}

pub struct EmptyPath;

impl AsRef<Path> for EmptyPath {
    fn as_ref(&self) -> &Path {
        Path::new("")
    }
}
