use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str;

const DEP_LICENSES_PATH: &str = "target/DEP_LICENSES";
const IF_DEBUG_TEXT: &str = "This is a debug build. \
                             It should not be used by users. \
                             Please obtain a release build instead.";

// The build script for `sic` primarily makes licenses of dependencies available to the installed
// executable.
// Previously we used cargo-make for this purpose, but in case `sic` is installed by running
// `cargo install --force sic`, `cargo make release` is not invoked.
// To fix that, we will now use this build script instead.
// As a bonus, it should now work both on Windows and Linux out of the box; i.e. on Windows it
// doesn't rely on some installed tools like which anymore.
fn main() {
    println!("Starting the pre-processing of a `sic` build.");

    // If we are not creating a release; do not complicate the build.
    if cfg!(debug_assertions) {
        write(DEP_LICENSES_PATH, IF_DEBUG_TEXT.as_bytes());
        return;
    }

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
        .expect("Unable to read `cargo bom` output.")
        .stdout;

    write(DEP_LICENSES_PATH, &dep_licenses_in_bytes);

    println!("Completed the pre-processing of a `sic` build.");
}

fn write(path: &str, contents: &[u8]) {
    // Truncates the file if it exists; else creates it.
    let mut file = File::create(path).expect("Unable to create dependency license file.");

    file.write_all(contents)
        .expect("Unable to write license texts to license file.");
}
