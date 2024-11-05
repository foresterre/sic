fmt: run

run:
    cargo fmt --all --manifest-path {{ join(justfile_directory(), 'Cargo.toml') }}

check:
    cargo fmt --all --manifest-path {{ join(justfile_directory(), 'Cargo.toml') }}  -- --check
