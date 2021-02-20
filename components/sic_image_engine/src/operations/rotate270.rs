use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct Rotate270;

impl Rotate270 {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImageOperation for Rotate270 {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => *image = image.rotate270(),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
