use crate::wrapper::font_options::FontOptions;

#[derive(Debug, Clone, PartialEq)]
pub struct DrawTextInner {
    text: String,
    coord: (i32, i32),
    font_options: FontOptions,
}

impl DrawTextInner {
    pub fn new(text: String, coord: (i32, i32), font_options: FontOptions) -> Self {
        DrawTextInner {
            text,
            coord,
            font_options,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn coords(&self) -> (i32, i32) {
        self.coord
    }

    pub fn font_options(&self) -> &FontOptions {
        &self.font_options
    }
}
