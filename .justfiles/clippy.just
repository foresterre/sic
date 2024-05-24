clippy:
    cargo clippy --manifest-path  {{justfile_directory()}}/Cargo.toml --all-targets --all-features -- -D warnings
