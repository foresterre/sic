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

pub fn command_unsplit_with_features(
    input: &str,
    output: &str,
    args: &[&str],
    features: &[&str],
) -> Child {
    command_unsplit_impl(input, output, args, features)
}

pub fn command_unsplit(input: &str, output: &str, args: &[&str]) -> Child {
    command_unsplit_impl(input, output, args, &[])
}

fn command_unsplit_impl(input: &str, output: &str, args: &[&str], features: &[&str]) -> Child {
    let input = setup_input_path(&input);
    let input = input.to_str().unwrap();
    let output = setup_output_path(&output);
    let output = output.to_str().unwrap();

    let mut command = Command::new("cargo");
    let mut arguments = Vec::with_capacity(128);
    arguments.push("run");

    if !features.is_empty() {
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
    arguments.extend(args);
    command.args(arguments);

    command.spawn().expect("Couldn't spawn child process.")
}

/// In and output path prefixes are pre-defined.
pub fn command(input: &str, output: &str, args: &str) -> Child {
    let input = setup_input_path(&input);
    let input = input.to_str().unwrap();
    let output = setup_output_path(&output);
    let output = output.to_str().unwrap();

    let mut command = Command::new("cargo");
    let mut arguments = Vec::with_capacity(128);
    arguments.push("run");

    arguments.push("--");
    arguments.push("-i");
    arguments.push(input);
    arguments.push("-o");
    arguments.push(output);

    arguments.extend(args.split_whitespace());

    command.args(arguments);
    command.spawn().expect("Couldn't spawn child process.")
}

pub const DEFAULT_IN: &str = "rainbow_8x6.bmp";

#[allow(unused)]
macro_rules! assert_not {
    ($e:expr) => {
        assert!(!$e)
    };
}
