deny: workspace

workspace: install-cargo-deny
    cargo deny --log-level error --workspace --manifest-path  {{ join(justfile_directory(), 'Cargo.toml') }} check

install-cargo-deny:
    cargo install cargo-deny
