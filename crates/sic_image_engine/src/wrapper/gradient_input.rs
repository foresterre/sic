// can be reusable by vertical-gradient and horizontal-gradient

use sic_core::image::Rgba;

type Color = Rgba<u8>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
