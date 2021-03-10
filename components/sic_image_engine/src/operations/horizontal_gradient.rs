use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use crate::wrapper::gradient_input::GradientInput;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use sic_core::{
    image::{self, DynamicImage},
    SicImage,
};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct HorizontalGradient {
    pub colors: GradientInput,
}

impl HorizontalGradient {
    pub fn new(colors: GradientInput) -> Self {
        Self { colors }
    }
}

impl ImageOperation for HorizontalGradient {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => gradient_static_image(image, self.colors),
            SicImage::Animated(image) => gradient_animated_image(image.frames_mut(), self.colors),
        }

        Ok(())
    }
}

fn gradient_static_image(img: &mut DynamicImage, input: GradientInput) {
    let right_color = &input.colors().0;
    let left_color = &input.colors().1;
    image::imageops::horizontal_gradient(img, right_color, left_color);
}

fn gradient_animated_image(frames: &mut [image::Frame], input: GradientInput) {
    let colors = input.colors();
    frames.par_iter_mut().for_each(|frame| {
        image::imageops::horizontal_gradient(frame.buffer_mut(), &colors.0, &colors.1);
    });
}
