use image::DynamicImage;

#[cfg(test)]
const _TEST_IMAGE_PATH: &str = "resources/unsplash_763569_cropped.jpg";

pub fn _setup_with(image: &str) -> DynamicImage {
    use std::path::Path;
    image::open(&Path::new(image)).unwrap()
}

#[cfg(test)]
pub fn _setup() -> DynamicImage {
    _setup_with(_TEST_IMAGE_PATH)
}

#[cfg(test)]
pub fn _manual_inspection(img: &DynamicImage, path: &str) {
    if !cfg!(feature = "dont-run-on-ci") {
        let _ = img.save(path);
    }
}
