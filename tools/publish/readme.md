# cargo-publish-workspace

## Goals

* Publish workspace packages to crates.io with a single command.
* Support our (opinionated) package versioning scheme (i.e. all packages have the same major and minor version number,
 and the package version will be incremented regardless whether there are or aren't changes to the package since the last version).
* Contribute to enhanced release automation, so releasing a new version takes less time, which enables us to release more often,
 and with smaller increments.

## Command & options

```
cargo-publish-workspace
Topologically publish a complete workspace

All arguments provided after two dashes (--) will be passed on to 'cargo publish'. This means, if
cargo publish-workspace itself doesn't support a flag related to publishing a cargo crate (yet), you
can still use this method. For example, you may use a custom registry with the following command
`cargo publish-workspace <..options> -- --registry <registry>`. The '--registry <registry>'
arguments will be passed to cargo publish. Note: some arguments are also passed on by cargo publish-
workspace, in which case, if also provided after the two dashes may be passed on twice. For example,
this would be the case if we would run: `cargo publish-workspace <...options> --no-verify -- --no-
verify`.

USAGE:
    cargo publish-workspace [FLAGS] [OPTIONS] --new-version <new-version>

FLAGS:
        --dry-run
            Simulate running this program

    -h, --help
            Prints help information

        --no-verify
            Don't build the crate before publishing

    -V, --version
            Prints version information


OPTIONS:
        --manifest <manifest>
            The workspace manifest file, usually the root Cargo.toml [default: Cargo.toml]

        --new-version <new-version>
            The version to which all workspace crates will be updated

        --sleep <sleep>
            The amount of seconds to sleep between publishing of workspace packages

            Allows the index to update [default: 5]


Issues, requests or questions can be submitted at: 'https://github.com/foresterre/sic/issues',
please add the label 'X-cargo-release-workspace', thanks!
```

## Examples

Update packages in workspace to `0.14.21`, without verifying before publish. Sleep for 10 seconds between publishing of packages, so the crates.io registry has time to refresh.
```
cargo publish-workspace --sleep 10 --no-verify --new-version 0.14.21
```
