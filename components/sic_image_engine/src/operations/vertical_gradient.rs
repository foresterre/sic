use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use crate::wrapper::gradient_input::GradientInput;
use sic_core::SicImage;

use crate::wrapper::gradient_fn;
use sic_core::image::imageops;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct VerticalGradient {
    pub colors: GradientInput,
}

impl VerticalGradient {
    pub fn new(colors: GradientInput) -> Self {
        Self { colors }
    }
}

impl ImageOperation for VerticalGradient {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => {
                gradient_fn::gradient_static_image(image, self.colors, imageops::vertical_gradient)
            }
            SicImage::Animated(image) => gradient_fn::gradient_animated_image(
                image.frames_mut(),
                self.colors,
                imageops::vertical_gradient,
            ),
        }

        Ok(())
    }
}
