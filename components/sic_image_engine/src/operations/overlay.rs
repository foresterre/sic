use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use crate::wrapper::overlay::OverlayInputs;
use sic_core::image::{imageops, DynamicImage};
use sic_core::SicImage;
use std::convert::TryFrom;

pub struct Overlay<'overlay> {
    inputs: &'overlay OverlayInputs,
}

impl<'overlay> Overlay<'overlay> {
    pub fn new(inputs: &'overlay OverlayInputs) -> Self {
        Self { inputs }
    }
}

impl<'overlay> ImageOperation for Overlay<'overlay> {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => overlay_impl(image, self.inputs),
            SicImage::Animated(_) => unimplemented!(),
        }
    }
}

fn overlay_impl(
    image: &mut DynamicImage,
    overlay: &OverlayInputs,
) -> Result<(), SicImageEngineError> {
    let overlay_image = overlay.image_path().open_image()?;
    let overlay_image = DynamicImage::try_from(overlay_image)?;

    let (x, y) = overlay.position();
    imageops::overlay(image, &overlay_image, x, y);

    Ok(())
}
