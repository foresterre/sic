use image::DynamicImage;

#[cfg(test)]
const _TEST_IMAGE_PATH: &str = "resources/unsplash_763569_cropped.jpg";

#[cfg(test)]
pub fn _setup() -> DynamicImage {
    use std::path::Path;
    image::open(&Path::new(_TEST_IMAGE_PATH)).unwrap()
}

#[cfg(test)]
pub fn _manual_inspection(img: &DynamicImage, path: &str) {
    if !cfg!(feature = "dont-run-on-ci") {
        let _ = img.save(path);
    }
}
