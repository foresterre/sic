use crate::errors::SicIoError;
use crate::preprocessor::Preprocess;
use sic_core::image::DynamicImage;
use sic_core::{image, SicImage};

pub struct ColorTypePreprocessor {
    format: image::ImageFormat,
}

impl ColorTypePreprocessor {
    pub fn new(format: image::ImageFormat) -> Self {
        Self { format }
    }
}

impl Preprocess for ColorTypePreprocessor {
    type Err = SicIoError;

    fn preprocess(&self, image: SicImage) -> Result<SicImage, SicIoError> {
        match image {
            SicImage::Static(image) if self.format == image::ImageFormat::Farbfeld => {
                Ok(to_rgba16(image))
            }
            // Jpeg encoder only supports L8 or Rgb8. We currently only work with ColorType (by
            // virtue of using DynamicImage), not ExtendedColorType. L8 is part of the extended
            // color type.
            // https://github.com/image-rs/image/blob/d71046727387b627e63effe7c56790bd355ec5ba/src/codecs/jpeg/encoder.rs#L454
            SicImage::Static(image) if self.format == image::ImageFormat::Jpeg => {
                Ok(to_rgb8(image))
            }
            // We must pre-process when the image format is Gif, since image::Frame only supports
            // RgbaImage = ImageBuffer<Rgba<u8>, Vec<u8>>, and our `DynamicEncoder` is unaware of the
            // underlying format
            SicImage::Static(image) if self.format == image::ImageFormat::Gif => {
                Ok(to_rgba8(image))
            }
            elsy => Ok(elsy),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum ColorTypeAdjustment {
    // Usually the default
    #[default]
    Enabled,
    Disabled,
}

impl From<bool> for ColorTypeAdjustment {
    fn from(value: bool) -> Self {
        if value {
            ColorTypeAdjustment::Enabled
        } else {
            ColorTypeAdjustment::Disabled
        }
    }
}

impl ColorTypeAdjustment {
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }
}

fn to_rgb8(image: DynamicImage) -> SicImage {
    if let ok @ DynamicImage::ImageRgb8(_) = image {
        SicImage::Static(ok)
    } else {
        SicImage::Static(DynamicImage::ImageRgb8(image.to_rgb8()))
    }
}

fn to_rgba8(image: DynamicImage) -> SicImage {
    if let ok @ DynamicImage::ImageRgb8(_) = image {
        SicImage::Static(ok)
    } else {
        SicImage::Static(DynamicImage::ImageRgba8(image.to_rgba8()))
    }
}

fn to_rgba16(image: DynamicImage) -> SicImage {
    if let ok @ DynamicImage::ImageRgba16(_) = image {
        SicImage::Static(ok)
    } else {
        SicImage::Static(DynamicImage::ImageRgba16(image.to_rgba16()))
    }
}
