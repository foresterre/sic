use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use crate::wrapper::gradient_input::GradientInput;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use sic_core::{
    image::{self, DynamicImage, Frame, Pixel},
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
    let (width, height) = img.to_rgba8().dimensions();
    let mut gradient_dynamic = DynamicImage::new_rgba8(width, height);
    image::imageops::horizontal_gradient(&mut gradient_dynamic, left_color, right_color);
    blend_static_image(img, gradient_dynamic);
}

fn gradient_animated_image(frames: &mut [image::Frame], input: GradientInput) {
    let (left_color, right_color) = &input.colors();
    let (width, height) = DynamicImage::ImageRgba8(frames[0].buffer().clone())
        .to_rgba8()
        .dimensions();
    let mut gradient_dynamic = DynamicImage::new_rgba8(width, height);
    image::imageops::horizontal_gradient(&mut gradient_dynamic, left_color, right_color);
    let gradient_buffer = gradient_dynamic.to_rgba8();

    frames.par_iter_mut().for_each(|frame| {
        blend_frame(frame, &gradient_buffer);
    });
}

fn blend_frame(frame: &mut Frame, layer: &image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    let mut gradient_iter = layer.pixels();
    let mut blended_buffer = DynamicImage::ImageRgba8(frame.buffer().clone()).into_rgba8();

    blended_buffer
        .pixels_mut()
        .for_each(|pixel| pixel.blend(gradient_iter.next().unwrap()));
    *frame = Frame::new(blended_buffer);
}

fn blend_static_image(img: &mut DynamicImage, layer: DynamicImage) {
    let gradient_rgba = layer.to_rgba8();
    let mut gradient_iter = gradient_rgba.pixels();
    let mut blended_buffer = img.to_rgba8();

    blended_buffer
        .pixels_mut()
        .for_each(|pixel| pixel.blend(gradient_iter.next().unwrap()));
    *img = DynamicImage::ImageRgba8(blended_buffer);
}
