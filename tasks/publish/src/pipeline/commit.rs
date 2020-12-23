use crate::arguments::PublishWorkspace;
use crate::pipeline::Action;
use std::path::Path;
use std::process::Command;

pub struct Commit {
    command: Command,
}

impl Action for Commit {
    fn run(&mut self) -> anyhow::Result<()> {
        let child_process = self.command.spawn()?;
        let result = child_process.wait_with_output()?;

        anyhow::ensure!(
            result.status.success(),
            format!(
                "Git command failed with:\n\tstdout:\n\t{}\n\tstderr:\t\n{}",
                String::from_utf8(result.stdout)?,
                String::from_utf8(result.stderr)?
            )
        );

        Ok(())
    }
}

impl Commit {
    pub fn from_working_dir(args: &PublishWorkspace, package_name: &str, dir: &Path) -> Self {
        let mut command = Command::new("git");
        command.current_dir(dir);

        Self::create_cmd(&mut command, package_name, &args.version(), args.dry_run);

        Commit { command }
    }

    fn create_cmd(command: &mut Command, package_name: &str, version: &str, dry_run: bool) {
        let message = Self::commit_message(package_name, version);
        let mut arguments = vec!["commit"];

        if dry_run {
            arguments.push("--dry-run");
        }

        arguments.extend(&["-m", &message, "--only", "--", "Cargo.*"]);

        command.args(&arguments);
    }

    fn commit_message(pkg_name: &str, version: &str) -> String {
        format!("update {}@{}", pkg_name, version)
    }
}
