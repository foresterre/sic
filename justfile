# determine the current Minimum Supported Rust Version for sic
msrv:
    cargo install cargo-msrv
    cargo msrv --output-format json -- cargo check --all-features

# format all workspace packages
fmt:
    cargo fmt --all

# run linter on all workspace packages
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# run tests in workspace
test:
    cargo test --all-features --all

deny:
	cargo deny --all-features check

# general check to run prior to committing source code
pre-commit:
    just fmt
    just lint
    just test
    just deny
