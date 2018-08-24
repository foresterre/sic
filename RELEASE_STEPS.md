Merge all PR's to the `master` branch, then:

### Verify

0. Verify that each PR has run `cargo fmt`.
1. Does `cargo test` succeed?
2. Does `cargo run -- resources/bwlines.png target/out.jpg --script "blur 10; flipv; resize 10 10;"` complete successfully and produce a 10x10 blurred and flipped vertically JPEG image?
3. Do all CI runs complete successfully?


### Update version number

4. Define update version number [as described here](https://doc.rust-lang.org/cargo/reference/publishing.html#publishing-a-new-version-of-an-existing-crate)
5. Update the version number in `Cargo.toml`
6. Update the version number in `src/main.rs` at the `Clap App .about()/1`


### Publish & Upload

7. Run `cargo package` and check the resulting `.crate`
8. Run `cargo publish` and verify the result


### Release, Tag & Binaries

9. Create a release with a version tag on [the github release page](https://github.com/foresterre/sic/releases)
    - tags are equal to crate version number e.g. `0.5.1`
    - release title is `sic-<version>` e.g. `sic-0.5.1`

10. Add Windows and Linux (Ubuntu compiled) binaries to the release
    - compile with `cargo build --release`