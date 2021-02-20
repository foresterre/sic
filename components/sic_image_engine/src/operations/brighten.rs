use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct Brighten {
    amount: i32,
}

impl Brighten {
    pub fn new(amount: i32) -> Self {
        Self { amount }
    }
}

impl ImageOperation for Brighten {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => *image = image.brighten(self.amount),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
