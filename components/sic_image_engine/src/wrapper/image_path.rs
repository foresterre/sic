use std::path::PathBuf;

use sic_io::load::{file_reader, load_image, ImportConfig};

use crate::errors::SicImageEngineError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageFromPath {
    path: PathBuf,
}

impl ImageFromPath {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn open_image(&self) -> Result<sic_core::image::DynamicImage, SicImageEngineError> {
        file_reader(self.path.as_path())
            .and_then(|mut file| load_image(&mut file, &ImportConfig::default()))
            .map_err(|_err| SicImageEngineError::LoadImageFromPath)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sic_testing::{image_eq, in_, open_test_image};

    #[test]
    fn open_from_path() {
        let path = in_!("palette_4x4.png");
        let image_from_path = ImageFromPath::new(PathBuf::from(path));

        let actual = image_from_path.open_image().unwrap();
        assert!(image_eq(actual, open_test_image(path)));
    }
}
