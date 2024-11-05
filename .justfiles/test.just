test: all no-default-features

all:
    cargo test --all --manifest-path {{ join(justfile_directory(), 'Cargo.toml') }}

no-default-features:
    cargo test --all --no-default-features --manifest-path {{ join(justfile_directory(), 'Cargo.toml') }}
