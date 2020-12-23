use crate::arguments::PublishWorkspace;
use crate::pipeline::Action;
use anyhow::Context;
use guppy::graph::PackageMetadata;
use std::process::Command;

pub struct PublishCrate<'g> {
    command: Command,
    pkg: &'g PackageMetadata<'g>,
}

impl<'g> PublishCrate<'g> {
    pub fn try_new(pkg: &'g PackageMetadata<'g>, args: &PublishWorkspace) -> anyhow::Result<Self> {
        let mut command = Command::new("cargo");
        command.args(&["publish", "--allow-dirty"]);

        if args.dry_run {
            command.arg("--dry-run");
        }

        if args.no_verify {
            command.arg("--no-verify");
        }

        if !args.pass_on.is_empty() {
            command.args(&args.pass_on);
        }

        let from_directory = pkg.manifest_path().parent().with_context(|| {
            format!(
                "Expected parent folder for Cargo manifest at {}",
                pkg.manifest_path().display()
            )
        })?;

        command.current_dir(from_directory);

        Ok(Self { command, pkg })
    }
}

impl Action for PublishCrate<'_> {
    fn run(&mut self, _args: &PublishWorkspace) -> anyhow::Result<()> {
        let child_process = self.command.spawn()?;
        let result = child_process.wait_with_output()?;

        anyhow::ensure!(
            result.status.success(),
            format!(
                "Cargo publish command failed for '{}' with:\n\tstdout:\n\t{}\n\tstderr:\n\t{}",
                self.pkg.manifest_path().display(),
                String::from_utf8(result.stdout)?,
                String::from_utf8(result.stderr)?,
            )
        );

        Ok(())
    }
}
