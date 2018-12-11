<!-- Merge all PR's to the `master` branch, then: -->

<!-- ### Release
Optional:

A description about what is included in this update, a thank you or something
else which is noteworthy :).
-->

### Changes from `current version` to `new version`:
- ... (issue: #X, PR: #Y)
- ... (issue: #X, PR: #Y)
- ... (issue: #X, PR: #Y)

---

### Problems encountered & their resolutions

Blocking:
- [ ] ... (issue: #X, PR: #Y)
- [ ] ... (issue: #X, PR: #Y)
- [ ] ... (issue: #X, PR: #Y)

---

### Release checklist

**Create Release PR**

- [ ] Included this checklist. Already a step complete! :wink:

**Verify**

- [ ] Verify formatting with `cargo +nightly fmt`.
- [ ] Does `cargo test` succeed?
- [ ] Verify image output of images produced with `cargo test --features output-test-images`.
- [ ] Do all CI runs complete successfully?


**Update version number**

- [ ] Define update version number [as described here](https://doc.rust-lang.org/cargo/reference/publishing.html#publishing-a-new-version-of-an-existing-crate).
- [ ] Update the version number in `Cargo.toml`.
- The Clap App version number in `src/main.rs` will be equal to the version number in `Cargo.toml`.


**Update the dependency licenses to be included in the binary as per their licenses**

- [ ] Run `cargo run --example update_dep_licenses` to update the licenses of dependencies, which will be included in a binary build.

**Verify again**

Same as above.

**Merge Release PR**

If no blocking issues are detected and this checklist is complete,
we can move on to the next step.

**Publish & Upload**

- [ ] Run `cargo package` and check the resulting `.crate`
- [ ] Run `cargo publish` and verify the result

**Release, Tag & Binaries**

- [ ] Create a release with a version tag on [the github release page](https://github.com/foresterre/sic/releases)
    - tags are equal to crate version number e.g. `0.5.1`
    - release title is `sic-<version>` e.g. `sic-0.5.1`


- [ ] Add Windows and Linux (Ubuntu compiled) binaries to the release
    - compile with `cargo build --release`

---

[Release steps](https://github.com/foresterre/sic/blob/master/RELEASE_STEPS.md)

---

PR: #<PR NUMBER>
