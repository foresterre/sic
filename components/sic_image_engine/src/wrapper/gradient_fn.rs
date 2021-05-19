use crate::wrapper::gradient_input::GradientInput;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use sic_core::image::{self, imageops, DynamicImage, GenericImageView, Pixel, Rgba, RgbaImage};

#[inline]
pub(crate) fn apply_vertical_gradient_on_static(img: &mut DynamicImage, input: GradientInput) {
    gradient_static_image(img, input, sic_core::image::imageops::horizontal_gradient)
}

#[inline]
pub(crate) fn apply_vertical_gradient_on_frames(img: &mut [image::Frame], input: GradientInput) {
    gradient_animated_image(img, input, sic_core::image::imageops::horizontal_gradient)
}

#[inline]
pub(crate) fn apply_horizontal_gradient_on_static(img: &mut DynamicImage, input: GradientInput) {
    gradient_static_image(img, input, imageops::vertical_gradient)
}

#[inline]
pub(crate) fn apply_horizontal_gradient_on_frames(img: &mut [image::Frame], input: GradientInput) {
    gradient_animated_image(img, input, imageops::vertical_gradient)
}

/// Applies a 2 input gradient over a static image
fn gradient_static_image<F>(img: &mut DynamicImage, input: GradientInput, f_gradient: F)
where
    F: Fn(&mut RgbaImage, &Rgba<u8>, &Rgba<u8>),
{
    let (left_color, right_color) = input.colors();
    let (width, height) = img.dimensions();
    let mut gradient_buffer = RgbaImage::new(width, height);
    f_gradient(&mut gradient_buffer, &left_color, &right_color);
    blend_static_image(img, &gradient_buffer);
}

fn blend_static_image(img: &mut DynamicImage, layer: &RgbaImage) {
    let mut blended_buffer = img.to_rgba8();
    blended_buffer
        .pixels_mut()
        .zip(layer.pixels())
        .for_each(|(source_pixel, gradient_pixel)| source_pixel.blend(gradient_pixel));
    *img = DynamicImage::ImageRgba8(blended_buffer);
}

/// Applies a 2 input gradient over a static image frame
fn gradient_animated_image<F>(frames: &mut [image::Frame], input: GradientInput, f_gradient: F)
where
    F: Fn(&mut RgbaImage, &Rgba<u8>, &Rgba<u8>),
{
    let (left_color, right_color) = input.colors();
    let (width, height) = frames[0].buffer().dimensions();
    let mut gradient_buffer = RgbaImage::new(width, height);
    f_gradient(&mut gradient_buffer, &left_color, &right_color);

    frames.par_iter_mut().for_each(|frame| {
        blend_frame(frame, &gradient_buffer);
    });
}

fn blend_frame(frame: &mut image::Frame, layer: &RgbaImage) {
    frame
        .buffer_mut()
        .pixels_mut()
        .zip(layer.pixels())
        .for_each(|(source_pixel, gradient_pixel)| source_pixel.blend(gradient_pixel));
}
