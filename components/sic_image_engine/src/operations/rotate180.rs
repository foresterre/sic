use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct Rotate180;

impl Rotate180 {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImageOperation for Rotate180 {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => *image = image.rotate180(),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
