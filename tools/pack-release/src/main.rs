#![allow(clippy::unnecessary_wraps)]

use crate::actions::{
    cargo_build, generate_shell_completions, get_stable_toolchains, rustup_toolchains,
    update_dep_licenses::update_dep_licenses,
};

pub mod actions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Update the dependency licenses
    let dep_licenses = update_dep_licenses();

    // Get a list of available toolchains
    let toolchain_list = rustup_toolchains();

    // Package a release for each stable toolchain
    for toolchain in get_stable_toolchains(&toolchain_list) {
        cargo_build(&toolchain, dep_licenses.as_ref());
    }

    generate_shell_completions("shell_completions", "shell_completions.zip");

    Ok(())
}
