use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // Compress the thanks/dependency_licenses.txt file, because it's huge.
    let folder = env::var("OUT_DIR").expect("OUT_DIR not set");
    let path = Path::new(&folder).join("compressed_dep_licenses");
    let file = File::create(&path).unwrap();

    let text = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/thanks",
        "/dependency_licenses.txt"
    ));

    let mut encoder = DeflateEncoder::new(file, Compression::default());
    encoder
        .write_all(text)
        .expect("Unable to compress dep licenses tet");
}
