use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct HueRotate {
    degree: i32,
}

impl HueRotate {
    pub fn new(degree: i32) -> Self {
        Self { degree }
    }
}

impl ImageOperation for HueRotate {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => *image = image.huerotate(self.degree),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
