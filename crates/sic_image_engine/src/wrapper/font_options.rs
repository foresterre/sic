use sic_core::image::Rgba;
use sic_core::rusttype::Scale;
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
    pub scale: Scale,
}

impl FontOptions {
    pub fn new(font_path: PathBuf, color: FontColor, scale: FontScale) -> Self {
        Self {
            font_path,
            color,
            scale: match scale {
                FontScale::Uniform(value) => Scale::uniform(value),
                FontScale::Scaling(horizontal, vertical) => Scale {
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
