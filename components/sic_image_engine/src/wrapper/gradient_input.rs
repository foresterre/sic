// can be reusable by vertical-gradient and horizontal-gradient

use sic_core::image::Rgba;

type Color = Rgba<u8>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientInput {
    colors: (Color, Color),
}

impl GradientInput {
    pub fn new(colors: (Color, Color)) -> Self {
        Self { colors }
    }

    pub fn colors(&self) -> (Color, Color) {
        self.colors
    }
}
