use std::path::{Path, PathBuf};

#[cfg(test)]
pub(crate) fn setup_test_image(test_image_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("resources")
        .join(test_image_path)
}

#[cfg(test)]
pub(crate) fn setup_output_path(test_output_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target")
        .join(test_output_path)
}

#[cfg(test)]
pub(crate) fn clean_up_output_path(test_output_path: &str) {
    std::fs::remove_file(setup_output_path(test_output_path))
        .expect("Unable to remove file after test.");
}
