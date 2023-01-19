use std::path::{Path, PathBuf};
use std::str;
use xshell::Shell;

const DEP_LICENSES_FOLDER: &str = "thanks";
const DEP_LICENSES_IGNORE: &str = "thanks/.gitignore";
const DEP_LICENSES_PATH: &str = "thanks/licenses.html";
const PROGRAM: &str = "cargo-about";

// This script updates the dependency licenses file by using cargo-about to generate the
// dependencies, this project relies on.
pub fn update_dep_licenses(shell: &Shell) -> impl AsRef<Path> {
    println!("Starting the update process of the dependency licenses file.");

    // Check if cargo-about is available in our PATH.
    let which = if cfg!(windows) {
        xshell::cmd!(shell, "where.exe {PROGRAM}")
            .read()
            .expect("Unable to read from where.exe")
    } else {
        xshell::cmd!(shell, "which {PROGRAM}")
            .read()
            .expect("Unable to read from which")
    };

    // Convert to a path and check if it exists;
    // If it does; cargo-about has been found and doesn't need to be installed first.
    let path = Path::new(&which);

    if !path.exists() {
        if let Err(_err) = xshell::cmd!(shell, "cargo install {PROGRAM}").run() {
            panic!("Unable to install license listing tool.");
        }
    } else {
        println!("license listing tool path found at: {:?}", path);
    }

    let about = xshell::cmd!(shell, "cargo about generate about.hbs")
        .read()
        .expect("Unable to run and read `cargo about` output ");

    shell
        .create_dir(DEP_LICENSES_FOLDER)
        .expect("Unable to create output folder");
    shell
        .write_file(DEP_LICENSES_IGNORE, "*")
        .expect("Unable to write ignore file");

    shell
        .write_file(DEP_LICENSES_PATH, about)
        .expect("Unable to write license texts to license file.");

    println!("Completed the update process of the dependency licenses file.");

    PathBuf::from(DEP_LICENSES_PATH)
}
