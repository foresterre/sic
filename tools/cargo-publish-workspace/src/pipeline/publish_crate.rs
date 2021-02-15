use crate::arguments::PublishWorkspace;
use crate::pipeline::Action;
use anyhow::Context;
use guppy::graph::PackageMetadata;
use std::process::{Command, Stdio};

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
        let child_process = self.command.stderr(Stdio::piped()).spawn()?;
        let result = child_process.wait_with_output()?;

        if !result.status.success() {
            println!("encountered publish error for {}", self.pkg.name());
            if String::from_utf8_lossy(&result.stderr).contains("already uploaded") {
                println!("skipping {}: already published", self.pkg.name());
                return Ok(());
            } else {
                anyhow::bail!(
                    "publish {} failed in {}",
                    self.pkg.name(),
                    self.pkg.manifest_path().display()
                );
            }
        };

        Ok(())
    }
}
