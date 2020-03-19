use std::path::{Path, PathBuf};

// re-export parameterized macro's
pub use parameterized::ide;
pub use parameterized::parameterized as pm;

// just enough, absolute tolerance, floating point comparison.
#[macro_export]
macro_rules! approx_eq_f32 {
    ($input:expr, $expected:expr) => {
        assert!(($input - $expected).abs() <= std::f32::EPSILON);
    };
}

#[macro_export]
macro_rules! out_ {
    ($path:expr) => {
        &[env!("CARGO_MANIFEST_DIR"), "/../../target/", $path].concat()
    };
}

#[macro_export]
macro_rules! in_ {
    ($path:expr) => {
        &[env!("CARGO_MANIFEST_DIR"), "/../../resources/", $path].concat()
    };
}

/// TODO{issue#128}: rework to provide flexibility and consistency, so all modules can use this;
pub fn setup_test_image(test_image_path: &str) -> PathBuf {
    Path::new("").join(in_!(test_image_path))
}

pub fn setup_output_path(test_output_path: &str) -> PathBuf {
    Path::new("").join(out_!(test_output_path))
}

pub fn clean_up_output_path(test_output_path: &str) {
    std::fs::remove_file(setup_output_path(test_output_path))
        .expect("Unable to remove file after test.");
}

pub fn open_test_image<P: AsRef<Path>>(path: P) -> sic_core::image::DynamicImage {
    sic_core::image::open(path.as_ref()).unwrap()
}

pub fn image_eq<T: Into<sic_core::image::DynamicImage>>(left: T, right: T) -> bool {
    use sic_core::image::GenericImageView;

    let left = left.into();
    let right = right.into();

    left.dimensions() == right.dimensions()
        && left
            .pixels()
            .zip(right.pixels())
            .all(|(l, r)| l.0 == r.0 && l.1 == r.1 && l.2 == r.2)
}
