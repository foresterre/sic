use sic_core::image::DynamicImage;

#[cfg(test)]
const DEFAULT_TEST_IMAGE_PATH: &str = "unsplash_763569_cropped.jpg";

#[cfg(test)]
macro_rules! out_ {
    ($path:expr) => {
        &[env!("CARGO_MANIFEST_DIR"), "/../target/", $path].concat()
    };
}

#[cfg(test)]
macro_rules! in_ {
    ($path:expr) => {
        &[env!("CARGO_MANIFEST_DIR"), "/../resources/", $path].concat()
    };
}

#[cfg(test)]
pub fn setup_test_image(image: &str) -> DynamicImage {
    use sic_core::image::open;
    use std::path::Path;
    open(&Path::new(image)).unwrap()
}

#[cfg(test)]
pub fn setup_default_test_image() -> DynamicImage {
    setup_test_image(in_!(DEFAULT_TEST_IMAGE_PATH))
}

#[cfg(test)]
pub fn output_test_image_for_manual_inspection(img: &DynamicImage, path: &str) {
    if cfg!(feature = "output-test-images") {
        let _ = img.save(path);
    }
}
