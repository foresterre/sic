use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct Rotate90;

impl Rotate90 {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImageOperation for Rotate90 {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => *image = image.rotate90(),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
