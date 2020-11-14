use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, ExitStatus, Output};
use zip::write::FileOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let about = about()?;
    let (exit, out, err) = output_to_string(&about);
    print_output(exit, out.as_ref(), err.as_ref());

    for toolchain in get_toolchains(out.as_ref()) {
        //
        cargo_build(&toolchain)?;
    }

    Ok(())
}

fn cargo_build(toolchain: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::current_dir()?;
    let path = path
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| std::io::Error::new(io::ErrorKind::NotFound, "sic directory not found"))?;

    println!("cwd: {}", &path.display());

    // create output directory
    let output_dir = path.join(toolchain);
    std::fs::create_dir_all(&output_dir)?;

    // build executable
    let build_output = std::process::Command::new("cargo")
        .current_dir(&path)
        .arg(format!("+{}", toolchain))
        .arg("build")
        .arg("--target-dir")
        .arg(&output_dir)
        .arg("--release")
        .output()?;

    let (exit, out, err) = output_to_string(&build_output);
    print_output(exit, out.as_ref(), err.as_ref());

    // zip output
    let exe = &output_dir.join("release").join("sic.exe");
    let version = sic_version(&exe)?;
    let zip = format!(
        "{}-{}.zip",
        version.trim(),
        toolchain.trim().replace("stable-", "")
    );

    write_zip(&exe, Path::new(&zip))?;

    // remove output directory
    let _ = std::fs::remove_dir_all(&output_dir)?;

    Ok(())
}

fn sic_version(exe: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new(exe).arg("--version").output()?;
    let output = String::from_utf8_lossy(&output.stdout);

    Ok(output.replace(' ', "-"))
}

fn write_zip(exe: &Path, destination: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("exe: {}", exe.display());
    println!("destination: {}", destination.display());

    let zip_file = File::create(&destination)?;
    let mut zip = zip::ZipWriter::new(zip_file);
    let mut buffer = Vec::<u8>::with_capacity(10 * 1000 * 1000);
    let mut exe_file = File::open(exe)?;
    let size = exe_file.read_to_end(&mut buffer)?;

    println!("exe size: {}", size);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    zip.start_file("sic.exe", options)?;
    zip.write_all(&buffer)?;

    zip.finish()?;

    Ok(())
}

fn about() -> io::Result<Output> {
    Command::new("rustup").arg("toolchain").arg("list").output()
}

fn output_to_string(output: &Output) -> (ExitStatus, Cow<'_, str>, Cow<'_, str>) {
    (
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    )
}

fn get_toolchains(toolchain_info: &str) -> Vec<String> {
    toolchain_info
        .lines()
        .map(|line| line.split_ascii_whitespace().take(1).collect::<String>())
        .filter(|s| s.starts_with("stable"))
        .collect::<Vec<String>>()
}

fn print_output(exit_code: ExitStatus, stdout: &str, stderr: &str) {
    println!("status: {}", exit_code);
    println!("stdout: {}", stdout);
    eprintln!("stderr: {}", stderr);
}
