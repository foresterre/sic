use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct Contrast {
    f: f32,
}

impl Contrast {
    pub fn new(f: f32) -> Self {
        Self { f }
    }
}

impl ImageOperation for Contrast {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => *image = image.adjust_contrast(self.f),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
