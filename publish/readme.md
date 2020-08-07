_work-in-progress_

**Goals:**

* Publish workspace packages to crates.io with a single command.
* Use our opinionated package versioning scheme (i.e. all packages have the same major and minor version number,
 and the package version will be incremented regardless whether there are or aren't changes to the package since the last version).
* Contribute to enhanced release automation, so releasing a new version takes less time, which enables us to release more often,
 and with smaller increments.

**Example:**

`cargo run --package publish --bin publish --  --new-version 0.13.0`
or
`publish --new-version 0.13.0`