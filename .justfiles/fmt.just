fmt: run

run:
    cargo fmt --all --manifest-path {{ justfile_directory() / 'Cargo.toml'}}

check:
    cargo fmt --all --manifest-path {{ justfile_directory() / 'Cargo.toml'}}  -- --check
