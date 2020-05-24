#[macro_use]
pub mod common;

use common::{command_unsplit, DEFAULT_IN};

#[test]
fn script_from_file_ok() {
    let script_file = &[env!("CARGO_MANIFEST_DIR"), "/resources/script/emboss.sic"].concat();

    let mut process = command_unsplit(
        DEFAULT_IN,
        "cio_script_from_file__ok.png",
        &["--operations-script", script_file],
    );
    let result = process.wait();
    assert!(result.is_ok());
    assert!(result.unwrap().success());
}

#[test]
fn script_from_file_where_file_not_found() {
    let script = r#""_.sic""#;

    let mut process = command_unsplit(
        DEFAULT_IN,
        "cio_script_from_file__file_not_found.png",
        &["--operations-script", script],
    );
    let result = process.wait();
    assert!(result.is_ok());
    assert_not!(result.unwrap().success());
}

#[test]
fn script_from_file_conflicting_args() {
    let script_file = &[env!("CARGO_MANIFEST_DIR"), "/resources/script/emboss.sic"].concat();
    let script = r#""blur 1""#;

    let mut process = command_unsplit(
        DEFAULT_IN,
        "cio_script_from_file__conflicted_args.png",
        &[
            "--operations-script",
            script_file,
            "--apply-operations",
            script,
        ],
    );
    let result = process.wait();
    assert!(result.is_ok());
    assert_not!(result.unwrap().success());
}
