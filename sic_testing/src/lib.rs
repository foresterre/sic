use std::path::{Path, PathBuf};

macro_rules! out_ {
    ($path:expr) => {
        &[env!("CARGO_MANIFEST_DIR"), "/../target/", $path].concat()
    };
}

macro_rules! in_ {
    ($path:expr) => {
        &[env!("CARGO_MANIFEST_DIR"), "/../resources/", $path].concat()
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
