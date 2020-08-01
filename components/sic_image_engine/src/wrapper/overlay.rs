use crate::wrapper::image_path::ImageFromPath;

#[derive(Clone, Debug)]
pub struct OverlayInputs(ImageFromPath, (u32, u32));

impl OverlayInputs {
    pub fn new(image_path: ImageFromPath, pos: (u32, u32)) -> Self {
        OverlayInputs(image_path, pos)
    }

    pub fn image_path(&self) -> &ImageFromPath {
        &self.0
    }

    pub fn position(&self) -> (u32, u32) {
        self.1
    }
}

impl PartialEq for OverlayInputs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
