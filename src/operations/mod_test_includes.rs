use image::DynamicImage;

#[cfg(test)]
const DEFAULT_TEST_IMAGE_PATH: &str = "resources/unsplash_763569_cropped.jpg";

#[cfg(test)]
pub fn setup_test_image(image: &str) -> DynamicImage {
    use std::path::Path;
    image::open(&Path::new(image)).unwrap()
}

#[cfg(test)]
pub fn setup_default_test_image() -> DynamicImage {
    setup_test_image(DEFAULT_TEST_IMAGE_PATH)
}

#[cfg(test)]
pub fn output_test_image_for_manual_inspection(img: &DynamicImage, path: &str) {
    if cfg!(feature = "output-test-images") {
        let _ = img.save(path);
    }
}
