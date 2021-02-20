use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use sic_core::image::imageops::FilterType;
use sic_core::image::DynamicImage;
use sic_core::SicImage;

#[derive(Debug)]
pub struct Resize {
    x: u32,
    y: u32,
    preserve_aspect_ratio: bool,
    filter_type: FilterType,
}

impl Resize {
    pub fn new(x: u32, y: u32, preserve_aspect_ratio: bool, filter_type: FilterType) -> Self {
        Self {
            x,
            y,
            preserve_aspect_ratio,
            filter_type,
        }
    }
}

impl ImageOperation for Resize {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => resize_impl(image, self),
            SicImage::Animated(_) => unimplemented!(),
        }

        Ok(())
    }
}

fn resize_impl(image: &mut DynamicImage, cfg: &Resize) {
    if cfg.preserve_aspect_ratio {
        resize_with_preserve_aspect_ratio(image, cfg.x, cfg.y, cfg.filter_type)
    } else {
        resize_regularly(image, cfg.x, cfg.y, cfg.filter_type)
    }
}

fn resize_regularly(image: &mut DynamicImage, x: u32, y: u32, filter_type: FilterType) {
    *image = image.resize_exact(x, y, filter_type);
}

fn resize_with_preserve_aspect_ratio(
    image: &mut DynamicImage,
    x: u32,
    y: u32,
    filter_type: FilterType,
) {
    *image = image.resize(x, y, filter_type);
}
