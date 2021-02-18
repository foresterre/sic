use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::write::FileOptions;

pub fn update_dep_licenses() {
    xshell::cmd!("cargo run --example update_dep_licenses")
        .run()
        .expect("Unable to update dep licenses")
}

pub fn cargo_build(toolchain: &str) {
    // get current directory, on top of which we'll create an output directory
    let path = std::env::current_dir().expect("Unable to get current directory");
    println!("cwd: {}", &path.display());

    // create the output directory
    let output_dir = path.join(toolchain);
    println!("building '{}' in '{}'", toolchain, &output_dir.display());
    xshell::mkdir_p(&output_dir).expect("Unable to create output directory");

    // build the project to the output directory
    xshell::cmd!("cargo +{toolchain} build --target-dir {output_dir} --release")
        .run()
        .expect("Unable to build");

    // zip output
    let exe = executable_path(&output_dir);
    let version = sic_version(&exe);
    let zip = format!(
        "{}-{}.zip",
        version.trim(),
        toolchain.trim().replace("stable-", "")
    );

    write_zip(&exe, Path::new(&zip));

    // remove output directory
    if option_env!("PACK_RELEASE_KEEP_OUTPUT").is_none() {
        let _ = std::fs::remove_dir_all(&output_dir);
    }
}

pub fn executable_path(output_dir: &Path) -> PathBuf {
    output_dir.join("release").join(executable_file())
}

pub const fn executable_file() -> &'static str {
    if cfg!(target_family = "windows") {
        "sic.exe"
    } else {
        "sic"
    }
}

pub fn sic_version(exe: &Path) -> String {
    xshell::cmd!("{exe} --version")
        .read()
        .expect("Unable to get the build sic version")
        .replace(' ', "-")
}

pub fn write_zip(exe: &Path, destination: &Path) {
    println!(
        "executable: {}\ndestination: {}",
        exe.display(),
        destination.display()
    );

    let zip_file = File::create(&destination).expect("Unable to create zip");
    let mut zip = zip::ZipWriter::new(zip_file);
    let mut buffer = Vec::<u8>::with_capacity(10 * 1000 * 1000);
    let mut exe_file = File::open(exe).expect("Unable to read executable for zipping");
    let size = exe_file
        .read_to_end(&mut buffer)
        .expect("Unable to read executable to memory");
    println!("executable size: {}", size);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    zip.start_file(executable_file(), options)
        .expect("Unable to start zip file");
    zip.write_all(&buffer).expect("Unable to zip release");

    zip.finish().expect("Unable to finish zipping release");
}

pub fn rustup_toolchains() -> String {
    xshell::cmd!("rustup toolchain list")
        .read()
        .expect("Unable to get toolchain list")
}

pub fn get_stable_toolchains(toolchain_info: &str) -> Vec<String> {
    toolchain_info
        .lines()
        .map(|line| line.split_ascii_whitespace().take(1).collect::<String>())
        .filter(|s| s.starts_with("stable"))
        .collect::<Vec<String>>()
}
