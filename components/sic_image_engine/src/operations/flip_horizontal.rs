use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct FlipHorizontal;

impl FlipHorizontal {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImageOperation for FlipHorizontal {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => *image = image.fliph(),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
