msrv: verify

run: install-cargo-msrv
    cargo msrv --manifest-path {{justfile_directory()}}/Cargo.toml

verify: install-cargo-msrv
    cargo msrv verify --manifest-path {{justfile_directory()}}/Cargo.toml

install-cargo-msrv:
    cargo install cargo-msrv --version 0.16.0-beta.20
