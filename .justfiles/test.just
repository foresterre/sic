test: all no-default-features

all:
    cargo test --all

no-default-features:
    cargo test --all --no-default-features
