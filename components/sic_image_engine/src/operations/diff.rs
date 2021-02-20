use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use crate::wrapper::image_path::ImageFromPath;
use sic_core::image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use sic_core::SicImage;
use std::cmp;
use std::convert::TryFrom;

pub struct Diff<'image> {
    path: &'image ImageFromPath,
}

impl<'image> Diff<'image> {
    pub fn new(path: &'image ImageFromPath) -> Self {
        Self { path }
    }
}

impl<'image> ImageOperation for Diff<'image> {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => diff_impl(image, self.path),
            SicImage::Animated(_) => unimplemented!(),
        }
    }
}

fn diff_impl(image: &mut DynamicImage, path: &ImageFromPath) -> Result<(), SicImageEngineError> {
    let cmp = path.open_image()?;
    let cmp = DynamicImage::try_from(cmp)?;

    *image = produce_image_diff(&image, &cmp)?;

    Ok(())
}

// same -> white pixel
pub(crate) const DIFF_PX_SAME: Rgba<u8> = Rgba([255, 255, 255, 255]);
// different -> coloured pixel
pub(crate) const DIFF_PX_DIFF: Rgba<u8> = Rgba([255, 0, 0, 255]);
// non overlapping -> transparent pixel
pub(crate) const DIFF_PX_NO_OVERLAP: Rgba<u8> = Rgba([0, 0, 0, 0]);

/// Takes the diff of two images.
///
/// If a pixel at `(x, y)` in the image `this` (`P`) compared to the pixel at `(x, y)` in the image `that` (`Q`):
/// * is the same: the output image will colour that pixel white.
/// * differs: the output image will colour that pixel red.
///
/// The output image (`R`) will have width `w=max(width(this), width(that))` and height
/// `h=max(height(this), height(that))`.
///
/// In case that two images when overlapped differ inversely in both width and height, so
/// `(P_width > Q_width ∧ P_height < Q_height) ⊕ (P_width < Q_width ∧ P_height > Q_height)` then
/// there will be pixels in `R`, for which for some pixels `p_{i, j} ∈ R | p_{i, j} ∉ P ∨ p_{i, j} ∉ Q`.
/// That is, the part of output image which isn't part of either of the two original input images.
/// These pixels will be 'coloured' black but with an alpha value of 0, so they will be transparent
/// as to show they were not part of the input images.
fn produce_image_diff(
    this: &DynamicImage,
    other: &DynamicImage,
) -> Result<DynamicImage, SicImageEngineError> {
    let (lw, lh) = this.dimensions();
    let (rw, rh) = other.dimensions();

    let w = cmp::max(lw, rw);
    let h = cmp::max(lh, rh);

    let mut buffer = ImageBuffer::new(w, h);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        if this.in_bounds(x, y) && other.in_bounds(x, y) {
            if this.get_pixel(x, y) == other.get_pixel(x, y) {
                *pixel = DIFF_PX_SAME;
            } else {
                *pixel = DIFF_PX_DIFF;
            }
        } else if this.in_bounds(x, y) || other.in_bounds(x, y) {
            *pixel = DIFF_PX_DIFF;
        } else {
            *pixel = DIFF_PX_NO_OVERLAP;
        }
    }

    Ok(DynamicImage::ImageRgba8(buffer))
}
