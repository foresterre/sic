use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use sic_core::image::{DynamicImage, GenericImageView, RgbaImage};
use sic_core::{image, SicImage};

type GapT = u32;

/// In it's current form, the operation draws a white grid with a certain gap, starting from the top
/// left of the image.
///
/// TODO: enhance with drawing options
pub struct DrawGrid {
    gap: GapT,
}

impl DrawGrid {
    pub fn new(gap: GapT) -> Self {
        Self { gap }
    }
}

impl ImageOperation for DrawGrid {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => image.draw_grid(self.gap),
            SicImage::Animated(image) => draw_grid_animated_image(image.frames_mut(), self.gap),
        }

        Ok(())
    }
}

macro_rules! draw_grid {
    ($image:expr, $f:expr, $gap:expr) => {{
        let (width, height) = $image.dimensions();

        for x in 0..width {
            for y in 0..height {
                if x % $gap == 0 || y % $gap == 0 {
                    $f($image, x, y);
                }
            }
        }
    }};
}

trait DrawGridExt {
    fn draw_grid(&mut self, gap: GapT);
}

impl DrawGridExt for DynamicImage {
    fn draw_grid(&mut self, gap: GapT) {
        draw_grid!(self, put_white_pixel, gap);
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

fn draw_grid_animated_image(frames: &mut [image::Frame], gap: GapT) {
    frames.par_iter_mut().for_each(|frame| {
        draw_grid!(frame.buffer_mut(), put_white_pixel_rgba, gap);
    });
}

// specialized version for animated images
fn put_white_pixel_rgba(image: &mut RgbaImage, x: u32, y: u32) {
    image.put_pixel(x, y, image::Rgba([u8::MAX, u8::MAX, u8::MAX, u8::MAX]));
}
