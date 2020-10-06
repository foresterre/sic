install:
	cargo install cross

build:
	make build-linux
	make build-mac
	make shasum

build-linux:
	@echo 'Building for Linux... 🐧'
	cross build --release --target=x86_64-unknown-linux-musl
	mkdir -p target/release-archives && tar -C target/x86_64-unknown-linux-musl/release -czf target/release-archives/sic-linux.tar.gz sic

# this only works on MacOS
build-mac:
	@echo 'Building for MacOS... 🍏'
	cross build --release --target=x86_64-apple-darwin
	mkdir -p target/release-archives && tar -C target/x86_64-apple-darwin/release -czf target/release-archives/sic-mac.tar.gz sic

shasum:
	shasum -a 256 target/release-archives/sic-*.tar.gz
