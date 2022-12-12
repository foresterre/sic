#[macro_use]
pub mod common;

use crate::common::*;

#[test]
fn enable_output_decider_fallback_enabled() {
    let mut process = SicTestCommandBuilder::new()
        .input_from_resources(DEFAULT_IN)
        .output_in_target("decider_fallback.ff")
        .with_args(["--enable-output-format-decider-fallback"])
        .spawn_child();
    let result = process.wait();
    assert!(result.unwrap().success());
}

#[test]
fn enable_output_decider_fallback_not_enabled() {
    let mut process = SicTestCommandBuilder::new()
        .input_from_resources(DEFAULT_IN)
        .output_in_target("decider_fallback.ff")
        .spawn_child();
    let result = process.wait();
    assert_not!(result.unwrap().success());
}
