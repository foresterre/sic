use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::SicImage;

pub struct Filter3x3<'kernel> {
    kernel: &'kernel [f32; 9],
}

impl<'kernel> Filter3x3<'kernel> {
    pub fn new(kernel: &'kernel [f32; 9]) -> Self {
        Self { kernel }
    }
}

impl<'kernel> ImageOperation for Filter3x3<'kernel> {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => *image = image.filter3x3(self.kernel),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}
