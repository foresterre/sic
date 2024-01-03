use std::path::Path;
use std::process::Command;
use std::str;

const DEP_LICENSES_DIR: &str = "thanks";
const DEP_LICENSES_IGNORE: &str = "thanks/.gitignore";
const DEP_LICENSES_PATH: &str = "thanks/licenses.html";
const PROGRAM: &str = "cargo-about";

// This script updates the dependency licenses file by using cargo-about to generate the
// dependencies, this project relies on.
//
// Usage:
// To update the dependency licenses file, run from the project root folder:
// `cargo run --example update_dep_licenses`
fn main() {
    println!("Starting the update process of the dependency licenses file.");

    // Check if cargo-about is available in our PATH.
    let license_tool_might_be_installed = if cfg!(windows) {
        Command::new("where.exe")
            .args([PROGRAM])
            .output()
            .expect("`where.exe` unavailable.")
            .stdout
    } else {
        Command::new("which")
            .args([PROGRAM])
            .output()
            .expect("`which` unavailable.")
            .stdout
    };

    // Convert to str
    let str_path = str::from_utf8(license_tool_might_be_installed.as_slice())
        .expect("Unable to convert path.")
        .trim();

    // Convert to a path and check if it exists;
    // If it does; cargo-about has been found and doesn't need to be installed first.
    let path = Path::new(str_path);

    if !path.exists() {
        let installation_code = Command::new("cargo")
            .args(["install", PROGRAM])
            .status()
            .expect("Unable to get status of cargo-about install.");

        // installation failed
        if !installation_code.success() {
            panic!("Unable to install license listing tool.");
        }
    } else {
        println!("license listing tool path found at: {:?}", path);
    }

    std::fs::create_dir_all(DEP_LICENSES_DIR).expect("Unable to create 'thanks' directory");
    std::fs::write(DEP_LICENSES_IGNORE, "*").expect("Unable to write ignore file");

    // Now cargo-about should be installed and in our PATH.
    // Next, we will use it to generate the licenses from our dependencies.
    // These will be saved under <crate>/target/DEP_LICENSES.
    let output = Command::new("cargo")
        .args(["about", "generate", "about.hbs"])
        .output().expect(
        "Unable to read `cargo about` output; `cargo-about` and `cargo` should be in your path!",
    );

    if !output.stderr.is_empty() {
        panic!(
            "Unable to generate list of dependency licenses: {}\n",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let dep_licenses_in_bytes = output.stdout;

    std::fs::write(DEP_LICENSES_PATH, dep_licenses_in_bytes)
        .expect("Unable to write license texts to license file.");

    println!("Completed the update process of the dependency licenses file.");
}
