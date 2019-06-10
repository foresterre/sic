use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str;

const DEP_LICENSES_PATH: &str = "thankyou/licenses.txt";
const CARGO_LOCK: &str = "Cargo.lock";

// The `update_dep_licenses` script is no longer a build.rs, mandatory pre-build script.
// To solve issues with `cargo install` (1) and ensure that the licenses of dependencies are
// included in a release binary, the licenses will now always be included.
// This will be done by including a `LICENSES_DEPENDENCIES` file in the root of the repo.
// Every release, this file needs to be updated, and there this script comes into play.
//
// This script updates the `LICENSES_DEPENDENCIES` file by using cargo-bom to generate the
// dependencies, this project relies on.
//
// Usage:
// To update the `LICENSES_DEPENDENCIES` file, run from the project root folder:
// `cargo run --example update_dep_licenses`
//
// (1): https://github.com/foresterre/sic/issues/50
//
// >> The build script for `sic` primarily makes licenses of dependencies available to the installed
// >> executable.
// >> Previously we used cargo-make for this purpose, but in case `sic` is installed by running
// >> `cargo install --force sic`, `cargo make release` is not invoked.
// >> To fix that, we will now use this build script instead.
// >> As a bonus, it should now work both on Windows and Linux out of the box; i.e. on Windows it
// >> doesn't rely on some installed tools like which anymore.
fn main() {
    println!("Starting the update process of the dependency licenses file.");

    // Check if cargo-bom is available in our PATH.
    let cargo_bom_might_be_installed = if cfg!(windows) {
        Command::new("where.exe")
            .args(&["cargo-bom"])
            .output()
            .expect("`where.exe` unavailable.")
            .stdout
    } else {
        Command::new("which")
            .args(&["cargo-bom"])
            .output()
            .expect("`which` unavailable.")
            .stdout
    };

    // Convert to str
    let str_path = str::from_utf8(cargo_bom_might_be_installed.as_slice())
        .expect("Unable to convert path.")
        .trim();

    // Convert to a path and check if it exists;
    // If it does; cargo-bom has been found and doesn't need to be installed first.
    let path = Path::new(str_path);

    // In this case we install cargo-bom
    if !path.exists() {
        let installation_code = Command::new("cargo")
            .args(&["install", "cargo-bom"])
            .status()
            .expect("Unable to get status of cargo-bom install.");

        // installation failed
        if !installation_code.success() {
            panic!("Unable to install cargo-bom.");
        }
    } else {
        println!("cargo-bom path found at: {:?}", path);
    }

    // Now cargo-bom should be installed and in our PATH.
    // Next, we will use cargo-bom to generate the licenses from our dependencies.
    // These will be saved under <crate>/target/DEP_LICENSES.
    let dep_licenses_in_bytes = Command::new("cargo")
        .args(&["bom"])
        .output()
        .expect(
            "Unable to read `cargo bom` output; `cargo-bom` and `cargo` should be in your path!",
        )
        .stdout;

    write_file(DEP_LICENSES_PATH, &dep_licenses_in_bytes);

    println!("Completed the update process of the dependency licenses file.");

    if std::path::Path::new(CARGO_LOCK).exists() {
        std::fs::remove_file(CARGO_LOCK).expect("Unable to remove lock file.");
    }
}

fn write_file(path: &str, contents: &[u8]) {
    // Truncates the file if it exists; else creates it.
    let mut file = File::create(path).expect("Unable to create dependency license file.");

    file.write_all(contents)
        .expect("Unable to write license texts to license file.");
}
