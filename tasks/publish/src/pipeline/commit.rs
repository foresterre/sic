#![allow(unused)] // TODO!

use std::path::Path;
use std::process::Command;

pub(crate) fn create_git(is_dry_run: bool, dir: &Path) -> Box<dyn GitCommit> {
    if is_dry_run {
        Box::new(DryRunGit::with_working_dir(dir))
    } else {
        Box::new(RegularRunGit::from_working_dir(dir))
    }
}

pub(crate) trait GitCommit {
    fn commit_package(&mut self, pkg_name: &str, version: &str);
    fn run(&mut self) -> anyhow::Result<()>;
}

pub(crate) struct RegularRunGit {
    command: Command,
}

impl RegularRunGit {
    fn from_working_dir(dir: &Path) -> Self {
        let mut command = Command::new("git");
        command.current_dir(dir);

        RegularRunGit { command }
    }
}

impl GitCommit for RegularRunGit {
    fn commit_package(&mut self, pkg_name: &str, version: &str) {
        let message = commit_message(pkg_name, version);
        self.command
            .args(&["commit", "-m", &message, "--only", "--", "Cargo.*"]);
    }

    fn run(&mut self) -> anyhow::Result<()> {
        let child_process = self.command.spawn()?;
        let result = child_process.wait_with_output()?;

        anyhow::ensure!(
            result.status.success(),
            format!(
                "Git command failed with stdout: {} and stderr: {}",
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr)
            )
        );

        Ok(())
    }
}

pub(crate) struct DryRunGit {
    command: Command,
}

impl DryRunGit {
    fn with_working_dir(dir: &Path) -> Self {
        println!("git commit: using working dir: {}", dir.display());
        let mut command = Command::new("git");
        command.current_dir(dir);

        Self { command }
    }
}

impl GitCommit for DryRunGit {
    fn commit_package(&mut self, pkg_name: &str, version: &str) {
        println!(
            "git commit: changes will be committed with message '{}'",
            commit_message(pkg_name, version)
        );

        let message = commit_message(pkg_name, version);
        self.command.args(&[
            "commit",
            "--dry-run",
            "-m",
            &message,
            "--only",
            "--",
            "Cargo.*",
        ]);
    }

    fn run(&mut self) -> anyhow::Result<()> {
        println!("git commit: executing git command");

        let child_process = self.command.spawn()?;
        let result = child_process.wait_with_output()?;

        anyhow::ensure!(
            result.status.success(),
            format!(
                "Git command failed with stdout: {} and stderr: {}",
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr)
            )
        );

        Ok(())
    }
}

fn commit_message(pkg_name: &str, version: &str) -> String {
    format!("sic publish: {}@{}", pkg_name, version)
}
