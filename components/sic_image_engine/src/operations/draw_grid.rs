use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use sic_core::image::{DynamicImage, GenericImageView, RgbaImage};
use sic_core::{image, SicImage};

/// In it's current form, the operation draws a white grid with a certain gap, starting from the top
/// left of the image.
///
/// TODO: enhance with drawing options
pub struct DrawGrid {
    lines: (u32, u32),
}

impl DrawGrid {
    pub fn new(lines: (u32, u32)) -> Self {
        Self { lines }
    }
}

impl ImageOperation for DrawGrid {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => image.draw_grid(self.lines),
            SicImage::Animated(image) => draw_grid_animated_image(image.frames_mut(), self.lines),
        }

        Ok(())
    }
}

macro_rules! draw_grid {
    ($image:expr, $f:expr, $lines:expr) => {{
        let (width, height) = $image.dimensions();

        let gap_x = (width as f32 / $lines.0 as f32).round() as u32;
        let gap_y = (height as f32 / $lines.1 as f32).round() as u32;

        for x in 0..width {
            for y in 0..height {
                if (x != 0 && x != width - 1 && x % gap_x == 0)
                    || (y != 0 && y != height - 1 && y % gap_y == 0)
                {
                    $f($image, x, y);
                }
            }
        }
    }};
}

trait DrawGridExt {
    fn draw_grid(&mut self, lines: (u32, u32));
}

impl DrawGridExt for DynamicImage {
    fn draw_grid(&mut self, lines: (u32, u32)) {
        draw_grid!(self, put_white_pixel, lines);
    }
}

fn put_white_pixel(image: &mut DynamicImage, x: u32, y: u32) {
    match image {
        DynamicImage::ImageLuma8(inner) => inner.put_pixel(x, y, image::Luma([u8::MAX])),
        DynamicImage::ImageLumaA8(inner) => inner.put_pixel(x, y, image::LumaA([u8::MAX, u8::MAX])),
        DynamicImage::ImageRgb8(inner) => {
            inner.put_pixel(x, y, image::Rgb([u8::MAX, u8::MAX, u8::MAX]))
        }
        DynamicImage::ImageRgba8(inner) => {
            inner.put_pixel(x, y, image::Rgba([u8::MAX, u8::MAX, u8::MAX, u8::MAX]))
        }
        DynamicImage::ImageBgr8(inner) => {
            inner.put_pixel(x, y, image::Bgr([u8::MAX, u8::MAX, u8::MAX]))
        }
        DynamicImage::ImageBgra8(inner) => {
            inner.put_pixel(x, y, image::Bgra([u8::MAX, u8::MAX, u8::MAX, u8::MAX]))
        }
        DynamicImage::ImageLuma16(inner) => inner.put_pixel(x, y, image::Luma([u16::MAX])),
        DynamicImage::ImageLumaA16(inner) => {
            inner.put_pixel(x, y, image::LumaA([u16::MAX, u16::MAX]))
        }
        DynamicImage::ImageRgb16(inner) => {
            inner.put_pixel(x, y, image::Rgb([u16::MAX, u16::MAX, u16::MAX]))
        }
        DynamicImage::ImageRgba16(inner) => {
            inner.put_pixel(x, y, image::Rgba([u16::MAX, u16::MAX, u16::MAX, u16::MAX]))
        }
    }
}

fn draw_grid_animated_image(frames: &mut [image::Frame], lines: (u32, u32)) {
    frames.par_iter_mut().for_each(|frame| {
        draw_grid!(frame.buffer_mut(), put_white_pixel_rgba, lines);
    });
}

// specialized version for animated images
fn put_white_pixel_rgba(image: &mut RgbaImage, x: u32, y: u32) {
    image.put_pixel(x, y, image::Rgba([u8::MAX, u8::MAX, u8::MAX, u8::MAX]));
}
