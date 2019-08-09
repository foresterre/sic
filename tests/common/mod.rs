use std::process::{Command, Child};
use std::path::{Path, PathBuf};

pub fn setup_input_path(test_image_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join(test_image_path)
}

pub fn setup_output_path(test_output_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(test_output_path)
}

/// In and output path prefixes are pre-defined.
pub(crate) fn command(input: &str, output: &str, args: &str) -> Child {
    let input = setup_input_path(&input);
    let input = input.to_str().unwrap();
    let output = setup_output_path(&output);
    let output = output.to_str().unwrap();

    let mut command = Command::new("cargo");

    let mut arguments = vec!["run", "--", "-i", input, "-o", output];
    let provided: Vec<&str> = args.split_whitespace().collect();
    arguments.extend(provided);


    command.args(arguments);
    command.spawn().expect("Couldn't spawn child process.")
}

pub(crate) const DEFAULT_IN: &str = "rainbow_8x6.bmp";

macro_rules! assert_not {
    ($e:expr) => {
        assert!(!$e)
    };
}