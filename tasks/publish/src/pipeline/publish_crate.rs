use anyhow::Context;
use guppy::graph::PackageMetadata;
use std::process::Command;

pub(crate) fn create_publisher<'g>(
    is_dry_run: bool,
    pkg: PackageMetadata<'g>,
) -> anyhow::Result<Box<dyn PublishPackage + 'g>> {
    Ok(if is_dry_run {
        Box::new(PublishDryRun::try_new(pkg)?)
    } else {
        Box::new(PublishOnCratesIO::try_new(pkg)?)
    })
}

pub(crate) trait PublishPackage {
    fn publish(&mut self) -> anyhow::Result<()>;
}

pub(crate) struct PublishOnCratesIO<'g> {
    publisher: PublishImpl<'g>,
}

impl<'g> PublishOnCratesIO<'g> {
    pub(crate) fn try_new(pkg: PackageMetadata<'g>) -> anyhow::Result<Self> {
        Ok(Self {
            publisher: PublishImpl::try_new(pkg, false)?,
        })
    }
}

impl PublishPackage for PublishOnCratesIO<'_> {
    fn publish(&mut self) -> anyhow::Result<()> {
        self.publisher.publish()
    }
}

pub(crate) struct PublishDryRun<'g> {
    publisher: PublishImpl<'g>,
}

impl<'g> PublishDryRun<'g> {
    pub(crate) fn try_new(pkg: PackageMetadata<'g>) -> anyhow::Result<Self> {
        Ok(Self {
            publisher: PublishImpl::try_new(pkg, true)?,
        })
    }
}

impl PublishPackage for PublishDryRun<'_> {
    fn publish(&mut self) -> anyhow::Result<()> {
        self.publisher.publish()
    }
}

struct PublishImpl<'g> {
    command: Command,
    pkg: PackageMetadata<'g>,
}

impl<'g> PublishImpl<'g> {
    pub(crate) fn try_new(pkg: PackageMetadata<'g>, dry_run: bool) -> anyhow::Result<Self> {
        let mut command = Command::new("cargo");
        command.args(&["publish", "--allow-dirty", "--no-verify"]);

        if dry_run {
            command.arg("--dry-run");
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
