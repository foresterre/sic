Merge all PR's to the `master` branch, then:

### Verify

0. Verify that each PR has run `cargo +nightly fmt`.
1. Does `cargo test` succeed?
2. Does `cargo run -- resources/bwlines.png target/out.jpg --script "blur 10; flipv; resize 10 10;"` complete
    successfully and produce a 10x10 blurred and flipped vertically JPEG image?
3. Do all CI runs complete successfully?


### Update version number

4. Define update version number [as described here](https://doc.rust-lang.org/cargo/reference/publishing.html#publishing-a-new-version-of-an-existing-crate)
5. Update the version number in `Cargo.toml`
6. Update the version number in `src/main.rs` at the `Clap App .about()/1`

### Update the dependency licenses to be included in the binary as per their licenses.

7. Run `cargo run --example update_dep_licenses` to update the licenses of dependencies, which will be included in
    a binary build.

### Publish & Upload

8. Run `cargo package` and check the resulting `.crate`
9. Run `cargo publish` and verify the result


### Release, Tag & Binaries

10. Create a release with a version tag on [the github release page](https://github.com/foresterre/sic/releases)
    - tags are equal to crate version number e.g. `0.5.1`
    - release title is `sic-<version>` e.g. `sic-0.5.1`


11. Add Windows and Linux (Ubuntu compiled) binaries to the release
    - compile with `cargo build --release`

