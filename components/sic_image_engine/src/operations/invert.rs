use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct Invert;

impl Invert {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImageOperation for Invert {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => image.invert(),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
