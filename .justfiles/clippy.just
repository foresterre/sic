clippy:
    cargo clippy  --manifest-path  {{ join(justfile_directory(), 'Cargo.toml') }} --all-targets --all-features -- -D warnings
