use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct Blur {
    sigma: f32,
}

impl Blur {
    pub fn new(sigma: f32) -> Self {
        Self { sigma }
    }
}

impl ImageOperation for Blur {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => *image = image.blur(self.sigma),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
