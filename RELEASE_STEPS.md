RELEASE PR: #<PR NUMBER>

---

<!-- Merge all PR's to the `master` branch, then: -->

<!-- ### Release
Optional:

A description about what is included in this update, a thank you or something
else which is noteworthy :).
-->

### Notable changes from `0.10` to `0.11`:
- ... (issue: #X, PR: #Y)
- ... (issue: #X, PR: #Y)
- ... (issue: #X, PR: #Y)

---

### Open issues

Blocking:
- [ ] ... (issue: #X, PR: #Y)
- [ ] ... (issue: #X, PR: #Y)
- [ ] ... (issue: #X, PR: #Y)

---

### Release checklist

**Initialize release**

- [ ] Create release issue. Already a step complete! :wink:
- [ ] Create release branch

**Update version number**

- [ ] Define update version number [as described here](https://doc.rust-lang.org/cargo/reference/publishing.html#publishing-a-new-version-of-an-existing-crate).
- [ ] Update the version number in all `Cargo.toml` files (i.e. all workspace crates).
    - Specifically, for the pre-1.0 versions the versioning strategy is currently as follows:
        - The minor version number is the same for all workspace crates, and incremented
          for each release. 
        - The patch version is increased on a as-needed basis, separately per crate. 
- The Clap App version number in `src/main.rs` will be equal to the version number in the root `Cargo.toml`.


**Update the dependency licenses to be included in the binary as per their licenses**

- [ ] Run `cargo run --example update_dep_licenses` to update the licenses of dependencies, which will be included in a binary build.

**Create Release PR**

- [ ] Create release PR 

**Verify**

- [ ] Verify formatting with `cargo +nightly fmt`.
- [ ] Does `cargo test` succeed?
- [ ] Verify image output of images produced with `cargo test --features output-test-images`.
- [ ] Do all CI runs complete successfully?
- [ ] Are the versions indeed updated for all packages?
- [ ] Is the application version updated?
- [ ] Are the dependency licenses updated?

Completed!

**Merge Release PR**

If no blocking issues are detected and this checklist is complete,
we can move on to the next step.

**Publish & Upload**

- [ ] Run `cargo package` and check the resulting `.crate`
- [ ] Run `cargo publish` and verify the result

**Release, Tag & Binaries**

<!-- TODO add changelist in the repo and modify per PR -->
- [ ] Write release notes

- [ ] Create a release with a version tag on [the github release page](https://github.com/foresterre/sic/releases)
    - tags are equal to crate version number and prefixed with `v`, e.g. `v0.5.1`
    - release title is `sic-<version>` e.g. `sic-0.5.1`

- [ ] Add Windows and Linux binaries to the release
    - compile with `cargo build --release`


---

[Release steps](https://github.com/foresterre/sic/blob/master/RELEASE_STEPS.md)


