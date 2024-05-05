use crate::errors::SicIoError;
use crate::preprocessor::Preprocess;
use sic_core::image::buffer::ConvertBuffer;
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

    fn preprocess(self, image: SicImage) -> Result<SicImage, SicIoError> {
        match image {
            SicImage::Static(image) if self.format == image::ImageFormat::Farbfeld => {
                // A remaining open question: does a user expect for an image to be able to convert to a format even if the color type is not supported?
                // And even if the user does, should we?
                // I suspect that users expect that color type conversions should happen automatically.
                //
                // Testing also showed that even bmp with full black full white pixels do not convert correctly as of now. Why exactly is unclear;
                // Perhaps the color type of the bmp formatted test image?
                let out = DynamicImage::ImageRgba16(image.to_rgba8().convert());
                Ok(SicImage::Static(out))
            }
            // We must pre-process when the image format is Gif, since image::Frame only supports
            // RgbaImage = ImageBuffer<Rgba<u8>, Vec<u8>>, and our `DynamicEncoder` is unaware of the
            // underlying format
            SicImage::Static(image)
                if self.format == image::ImageFormat::Gif
                    && image.color() != image::ColorType::Rgba8 =>
            {
                Ok(SicImage::Static(DynamicImage::ImageRgba8(image.to_rgba8())))
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
