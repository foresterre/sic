#[macro_use]
pub mod common;

use crate::common::*;

#[test]
fn enable_output_decider_fallback_enabled() {
    let mut process = command(
        DEFAULT_IN,
        "decider_fallback.ff",
        "--enable-output-format-decider-fallback",
    );
    let result = process.wait();
    assert!(result.unwrap().success());
}

#[test]
fn enable_output_decider_fallback_not_enabled() {
    let mut process = command(DEFAULT_IN, "decider_fallback.ff", "");
    let result = process.wait();
    assert_not!(result.unwrap().success());
}
