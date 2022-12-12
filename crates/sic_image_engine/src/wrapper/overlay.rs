use crate::wrapper::image_path::ImageFromPath;

#[derive(Clone, Debug)]
pub struct OverlayInputs(ImageFromPath, (i64, i64));

impl OverlayInputs {
    pub fn new(image_path: ImageFromPath, pos: (i64, i64)) -> Self {
        OverlayInputs(image_path, pos)
    }

    pub fn image_path(&self) -> &ImageFromPath {
        &self.0
    }

    pub fn position(&self) -> (i64, i64) {
        self.1
    }
}

impl PartialEq for OverlayInputs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
