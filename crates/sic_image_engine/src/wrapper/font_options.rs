use sic_core::ab_glyph;
use sic_core::image::Rgba;
use std::path::PathBuf;

type FontColor = Rgba<u8>;

pub enum FontScale {
    Uniform(f32),
    Scaling(f32, f32),
}

#[derive(Debug, Clone)]
pub struct FontOptions {
    pub font_path: PathBuf,
    pub color: FontColor,
    pub scale: ab_glyph::PxScale,
}

impl FontOptions {
    pub fn new(font_path: PathBuf, color: FontColor, scale: FontScale) -> Self {
        Self {
            font_path,
            color,
            scale: match scale {
                FontScale::Uniform(value) => ab_glyph::PxScale::from(value),
                FontScale::Scaling(horizontal, vertical) => ab_glyph::PxScale {
                    x: horizontal,
                    y: vertical,
                },
            },
        }
    }
}

impl PartialEq for FontOptions {
    fn eq(&self, other: &Self) -> bool {
        // Equality for these font options is defined by the font, specifically its path.
        self.font_path.eq(&other.font_path)
    }
}
