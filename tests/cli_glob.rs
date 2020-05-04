#[allow(unused)]
#[macro_use]
pub mod common;

use common::{command, setup_output_path};

#[test]
fn keep_format() {
    const OUT: &str = "globtest/unmodified_format";

    let mut process = command("*.png", OUT, "--mode glob");
    let result = process.wait();
    assert!(result.is_ok());
    assert!(result.unwrap().success());

    let check_path = setup_output_path(OUT);

    let count = std::fs::read_dir(check_path)
        .unwrap()
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Some(ext) = entry.path().extension() {
                    if ext == "png" {
                        return Some(ext.to_os_string());
                    }
                }
            }
            None
        })
        .count();

    assert!(count >= 12)
}
#[test]
fn modify_format() {
    const OUT: &str = "globtest/modified_format_and_ext/";

    let mut process = command("*.png", OUT, "--mode glob --output-format jpg");
    let result = process.wait();
    assert!(result.is_ok());
    assert!(result.unwrap().success());

    let check_path = setup_output_path(OUT);

    let count = std::fs::read_dir(check_path)
        .unwrap()
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Some(ext) = entry.path().extension() {
                    if ext == "jpg" {
                        return Some(ext.to_os_string());
                    }
                }
            }
            None
        })
        .count();

    assert!(count >= 12)
}
