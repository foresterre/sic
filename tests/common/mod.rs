use std::path::{Path, PathBuf};
use std::process::{Child, Command};

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
pub fn command(input: &str, output: &str, args: &str) -> Child {
    command_with_features(input, output, args, &[], true)
}

pub fn command_with_features(
    input: &str,
    output: &str,
    args: &str,
    features: &[&str],
    split: bool,
) -> Child {
    let input = setup_input_path(&input);
    let input = input.to_str().unwrap();
    let output = setup_output_path(&output);
    let output = output.to_str().unwrap();

    let mut command = Command::new("cargo");
    let mut arguments = Vec::with_capacity(128);
    arguments.push("run");

    if features.len() >= 1 {
        arguments.push("--features");

        for feature in features {
            arguments.push(feature);
        }
    }

    arguments.push("--");
    arguments.push("-i");
    arguments.push(input);
    arguments.push("-o");
    arguments.push(output);

    let provided: Vec<&str> = if split {
        args.split_whitespace().collect()
    } else {
        vec![args]
    };

    arguments.extend(provided);

    dbg!(&arguments);

    command.args(arguments);
    command.spawn().expect("Couldn't spawn child process.")
}

pub const DEFAULT_IN: &str = "rainbow_8x6.bmp";

macro_rules! assert_not {
    ($e:expr) => {
        assert!(!$e)
    };
}
