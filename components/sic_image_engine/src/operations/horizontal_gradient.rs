use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use crate::wrapper::gradient_input::GradientInput;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use sic_core::{
    image::{self, DynamicImage, Frame, GenericImageView, Pixel, RgbaImage},
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
    let (left_color, right_color) = &input.colors();
    let (width, height) = img.dimensions();
    let mut gradient_buffer = RgbaImage::new(width, height);
    image::imageops::horizontal_gradient(&mut gradient_buffer, left_color, right_color);
    blend_static_image(img, &gradient_buffer);
}

fn gradient_animated_image(frames: &mut [image::Frame], input: GradientInput) {
    let (left_color, right_color) = &input.colors();
    let (width, height) = frames[0].buffer().dimensions();
    let mut gradient_buffer = RgbaImage::new(width, height);
    image::imageops::horizontal_gradient(&mut gradient_buffer, left_color, right_color);

    frames.par_iter_mut().for_each(|frame| {
        blend_frame(frame, &gradient_buffer);
    });
}

fn blend_static_image(img: &mut DynamicImage, layer: &RgbaImage) {
    let mut blended_buffer = img.to_rgba8();
    blended_buffer
        .pixels_mut()
        .zip(layer.pixels())
        .for_each(|(source_pixel, gradient_pixel)| source_pixel.blend(gradient_pixel));
    *img = DynamicImage::ImageRgba8(blended_buffer);
}

fn blend_frame(frame: &mut Frame, layer: &RgbaImage) {
    frame
        .buffer_mut()
        .pixels_mut()
        .zip(layer.pixels())
        .for_each(|(source_pixel, gradient_pixel)| source_pixel.blend(gradient_pixel));
}
