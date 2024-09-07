use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

#[derive(Clone, Debug)]
pub struct SicTestCommandBuilder {
    commands: Vec<OsString>,
    features: Vec<&'static str>,
}

impl SicTestCommandBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SicTestCommandBuilder {
            commands: Vec::with_capacity(128),
            features: Vec::new(),
        }
    }

    pub fn with_feature(mut self, feature: &'static str) -> Self {
        self.features.push(feature);
        self
    }

    pub fn input<S: Into<OsString>>(mut self, path: S) -> Self {
        self.commands.push("--input".into());
        self.commands.push(path.into());
        self
    }

    pub fn input_from_resources<S: AsRef<OsStr>>(mut self, path: S) -> Self {
        let path = Self::with_resources_path(path.as_ref());

        self.commands.push("--input".into());
        self.commands.push(path);
        self
    }

    pub fn glob_input<S: Into<OsString>>(mut self, pattern: S) -> Self {
        self.commands.push("--glob-input".into());
        self.commands.push(pattern.into());
        self
    }

    pub fn glob_input_from_resources<S: AsRef<OsStr>>(mut self, path: S) -> Self {
        let path = Self::with_resources_path(path.as_ref());

        self.commands.push("--glob-input".into());
        self.commands.push(path);
        self
    }

    pub fn output<S: Into<OsString>>(mut self, path: S) -> Self {
        self.commands.push("--output".into());
        self.commands.push(path.into());
        self
    }

    pub fn output_in_target<S: AsRef<OsStr>>(mut self, path: S) -> Self {
        let path = Self::with_target_path(path.as_ref());

        self.commands.push("--output".into());
        self.commands.push(path);
        self
    }

    pub fn glob_output<S: Into<OsString>>(mut self, path: S) -> Self {
        self.commands.push("--glob-output".into());
        self.commands.push(path.into());
        self
    }

    pub fn glob_output_in_target<S: AsRef<OsStr>>(mut self, path: S) -> Self {
        let path = Self::with_target_path(path.as_ref());

        self.commands.push("--glob-output".into());
        self.commands.push(path);
        self
    }

    pub fn with_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.commands.extend(args.into_iter().map(|s| s.into()));
        self
    }

    pub fn spawn_child(self) -> Child {
        let mut command = Command::new("cargo");
        command.arg("run");

        if !self.features.is_empty() {
            command.arg("--features");
            command.args(&self.features);
        }

        command.arg("--");

        command.args(self.commands);

        command
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .inspect_err(|_| {
                eprintln!(
                    "Spawn child error for SicTestCommandBuilder:\nCommand: {:?}",
                    command
                );
            })
            .expect("Unable to spawn child process for SicTestCommandBuilder instance")
    }

    fn with_resources_path(path: &OsStr) -> OsString {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join(path)
            .into_os_string()
    }

    fn with_target_path(path: &OsStr) -> OsString {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(path)
            .into_os_string()
    }
}

pub fn setup_input_path(test_image_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join(test_image_path)
}

pub fn setup_output_path(test_output_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(test_output_path)
}

pub const DEFAULT_IN: &str = "rainbow_8x6.bmp";

#[allow(unused)]
macro_rules! assert_not {
    ($e:expr) => {
        assert!(!$e)
    };
}
