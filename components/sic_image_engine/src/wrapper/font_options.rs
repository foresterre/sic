use rusttype::{Font, FontCollection, Scale};
use sic_core::image::Rgba;

type FontColor = Rgba<u8>;

#[derive(Debug, Clone)]
pub struct FontOptions {
    pub font: Font<'static>,
    pub color: FontColor,
    pub scale: Scale,
}

impl Default for FontOptions {
    fn default() -> Self {
        FontOptions {
            font: FontCollection::from_bytes(Vec::from(include_bytes!(
                "../../../../resources/font/Lato-Regular.ttf"
            ) as &[u8]))
            .expect("Unable to load font")
            .into_font()
            .expect("Unable to load font"),

            color: Rgba([255u8, 255u8, 77u8, 250u8]),
            scale: Scale { x: 16.0, y: 16.0 },
        }
    }
}

impl PartialEq for FontOptions {
    fn eq(&self, other: &Self) -> bool {
        unimplemented!()
    }
}
