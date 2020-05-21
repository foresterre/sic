use rusttype::Scale;
use sic_core::image::Rgba;
use std::path::PathBuf;

type FontColor = Rgba<u8>;

#[derive(Debug, Clone)]
pub struct FontOptions {
    pub font_path: PathBuf,
    pub color: FontColor,
    pub scale: Scale,
}

impl FontOptions {
    pub fn new(font_path: PathBuf, color: FontColor, scale: Scale) -> Self {
        Self {
            font_path,
            color,
            scale,
        }
    }
}

impl PartialEq for FontOptions {
    fn eq(&self, other: &Self) -> bool {
        // Equality for these font options is defined by the font, specifically its path.
        self.font_path.eq(&other.font_path)
    }
}
