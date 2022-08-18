#![allow(clippy::unnecessary_wraps)]

use crate::actions::{
    cargo_build, generate_shell_completions, get_stable_toolchains, rustup_toolchains,
    update_dep_licenses::update_dep_licenses,
};
use xshell::{PushEnv, Shell};

pub mod actions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shell = Shell::new()?;

    // Inherit the PATH from the caller; returns a RAII guard which must be maintained until we
    // completed the build
    let _guard = inherit_path(&shell);

    // Update the dependency licenses
    let dep_licenses = update_dep_licenses(&shell);

    // Get a list of available toolchains
    let toolchain_list = rustup_toolchains(&shell);

    // Package a release for each stable toolchain
    for toolchain in skip_windows_gnu(get_stable_toolchains(&toolchain_list)) {
        cargo_build(&shell, &toolchain, dep_licenses.as_ref());
    }

    generate_shell_completions(&shell, "shell_completions", "shell_completions.zip");

    Ok(())
}

type EnvGuard<'shell> = PushEnv<'shell>;

fn inherit_path(shell: &Shell) -> Option<EnvGuard> {
    option_env!("PATH").map(|envs| shell.push_env("PATH", envs))
}

// Don't build against Windows GNU toolchain
// Workaround for nasty build errors which are popping up again while building `rav1e 0.4.1`:
// ```
// error: failed to run custom build command for `rav1e v0.4.1
// [snip]
// thread 'main' panicked at 'NASM build failed. Make sure you have nasm installed. https://nasm.us: "failed to spawn process: program not found"'
// ````
// While nasm is available on the PATH:
// ```
// ❯ nasm --version
// NASM version 2.15.05 compiled on Aug 28 2020
// ```
fn skip_windows_gnu(
    toolchains: impl IntoIterator<Item = String>,
) -> impl IntoIterator<Item = String> {
    toolchains
        .into_iter()
        .filter(|toolchain| !toolchain.ends_with("windows-gnu"))
}
