use crate::errors::SicImageEngineError;
use crate::operations::ImageOperation;
use crate::wrapper::draw_text_inner::DrawTextInner;
use sic_core::image::DynamicImage;
use sic_core::SicImage;

pub struct DrawText<'dt> {
    text: &'dt DrawTextInner,
}

impl<'dt> DrawText<'dt> {
    pub fn new(text: &'dt DrawTextInner) -> Self {
        Self { text }
    }
}

impl ImageOperation for DrawText<'_> {
    fn apply_operation(&self, image: &mut SicImage) -> Result<(), SicImageEngineError> {
        match image {
            SicImage::Static(image) => draw_impl(image, self.text),
            SicImage::Animated(_) => unimplemented!(),
        }
    }
}

fn draw_impl(image: &mut DynamicImage, inner: &DrawTextInner) -> Result<(), SicImageEngineError> {
    let text = inner.text();
    let coords = inner.coords();
    let font_options = inner.font_options();

    let font_file =
        std::fs::read(&font_options.font_path).map_err(SicImageEngineError::FontFileLoadError)?;

    let font = rusttype::Font::try_from_bytes(&font_file).ok_or(SicImageEngineError::FontError)?;

    *image = DynamicImage::ImageRgba8(imageproc::drawing::draw_text(
        image,
        font_options.color,
        coords.0,
        coords.1,
        font_options.scale,
        &font,
        text,
    ));

    Ok(())
}
