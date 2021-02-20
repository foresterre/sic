use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct FlipVertical;

impl FlipVertical {
    pub fn new() -> Self {
        Self {}
    }
}

impl ImageOperation for FlipVertical {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => *image = image.flipv(),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
