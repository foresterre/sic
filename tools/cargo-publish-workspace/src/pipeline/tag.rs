use crate::arguments::PublishWorkspace;
use crate::pipeline::Action;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct Tag {
    command: Command,
}

impl Action for Tag {
    fn run(&mut self, args: &PublishWorkspace) -> anyhow::Result<()> {
        if args.dry_run || args.no_git_tag {
            return Ok(());
        }

        self.command.stdout(Stdio::inherit());
        self.command.stderr(Stdio::inherit());
        let mut child_process = self.command.spawn()?;
        let result = child_process.wait()?;
        println!("tag: git tag exited with exit code: {}", result);

        Ok(())
    }
}

impl Tag {
    pub fn from_working_dir(args: &PublishWorkspace, dir: &Path) -> Self {
        let mut command = Command::new("git");

        println!("tag: tagging in {}", dir.display());
        command.current_dir(dir);

        Self::create_cmd(&mut command, args.version());

        Self { command }
    }

    fn create_cmd(command: &mut Command, version: &str) {
        let tag = Self::tag_name(version);
        println!("tag: creating tag '{}'", &tag);
        let arguments = ["tag", &tag];

        command.args(&arguments);
    }

    fn tag_name(version: &str) -> String {
        format!("v{}", version)
    }
}
