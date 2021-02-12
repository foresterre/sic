test:
    cargo test --all

before-push:
    cargo fmt --all
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all

publish new_version:
    cargo run -p publish-cargo-workspace -- publish-workspace --new-version {{new_version}} --sleep 10
    cargo run -p pack-release