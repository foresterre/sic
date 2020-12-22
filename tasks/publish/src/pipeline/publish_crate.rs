use crate::arguments::PublishWorkspace;
use anyhow::Context;
use guppy::graph::PackageMetadata;
use std::process::Command;

pub(crate) fn create_publisher<'g>(
    pkg: PackageMetadata<'g>,
    args: &PublishWorkspace,
) -> anyhow::Result<Box<dyn PublishPackage + 'g>> {
    let publish = PublishImpl::try_new(pkg, args)?;

    Ok(Box::new(publish))
}

pub(crate) trait PublishPackage {
    fn publish(&mut self) -> anyhow::Result<()>;
}

struct PublishImpl<'g> {
    command: Command,
    pkg: PackageMetadata<'g>,
}

impl<'g> PublishImpl<'g> {
    pub(crate) fn try_new(
        pkg: PackageMetadata<'g>,
        args: &PublishWorkspace,
    ) -> anyhow::Result<Self> {
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

impl PublishPackage for PublishImpl<'_> {
    fn publish(&mut self) -> anyhow::Result<()> {
        let child_process = self.command.spawn()?;
        let result = child_process.wait_with_output()?;

        anyhow::ensure!(
            result.status.success(),
            format!(
                "Cargo publish command failed for '{}' with stdout: {} and stderr: {}",
                self.pkg.manifest_path().display(),
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr)
            )
        );

        Ok(())
    }
}
