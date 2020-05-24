#[macro_use]
pub mod common;

use common::{command, DEFAULT_IN};

#[test]
fn script_from_file_ok() {
    let script = r#""emboss.sic""#;

    let mut process = command(
        DEFAULT_IN,
        "cios_from_file_ok.png",
        &format!("--operations-script {}", script),
    );
    let result = process.wait();
    assert!(result.is_ok());
    assert!(result.unwrap().success());
}

#[test]
fn script_from_file_where_file_not_found() {
    let script = r#""emboss-2.sic""#;

    let mut process = command(
        DEFAULT_IN,
        "cios_from_file__file_not_found.png",
        &format!("--operations-script {}", script),
    );
    let result = process.wait();
    assert!(result.is_ok());
    assert_not!(result.unwrap().success());
}
