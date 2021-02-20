use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct Unsharpen {
    sigma: f32,
    threshold: i32,
}

impl Unsharpen {
    pub fn new(sigma: f32, threshold: i32) -> Self {
        Self { sigma, threshold }
    }
}

impl ImageOperation for Unsharpen {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => *image = image.unsharpen(self.sigma, self.threshold),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
