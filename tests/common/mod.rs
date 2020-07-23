use std::collections::VecDeque;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

pub struct SicTestCommandBuilder {
    command: Command,
    features: VecDeque<&'static str>,
}

impl SicTestCommandBuilder {
    pub fn new() -> Self {
        SicTestCommandBuilder {
            command: Command::new("sic"),
            features: VecDeque::new(),
        }
    }

    pub fn with_feature(mut self, feature: &'static str) -> Self {
        self.features.push_back(feature);
        self
    }

    pub fn finalize_cargo_options(mut self) -> Self {
        if !self.features.is_empty() {
            self.features.push_front("--features");
            self.command.args(&self.features);
        }

        self.command.arg("--");
        self
    }

    pub fn input(mut self, path: &str) -> Self {
        self.command.args(&["--input", path]);
        self
    }

    pub fn input_from_resources(mut self, path: &str) -> Self {
        let path = &Self::with_resources_path(path);

        self.command.args(&["--input", path]);
        self
    }

    pub fn glob_input(mut self, pattern: &str) -> Self {
        self.command.args(&["--glob-input", pattern]);
        self
    }

    pub fn glob_input_from_resources(mut self, path: &str) -> Self {
        let path = &Self::with_resources_path(path);

        self.command.args(&["--glob-input", path]);
        self
    }

    pub fn output(mut self, path: &str) -> Self {
        self.command.args(&["--output", path]);
        self
    }

    pub fn output_in_target(mut self, path: &str) -> Self {
        let path = &Self::with_target_path(path);

        self.command.args(&["--output", path]);
        self
    }

    pub fn glob_output(mut self, path: &str) -> Self {
        self.command.args(&["--glob-output", path]);
        self
    }

    pub fn glob_output_in_target(mut self, path: &str) -> Self {
        let path = &Self::with_target_path(path);

        self.command.args(&["--glob-output", path]);
        self
    }

    pub fn with_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.command.args(args);
        self
    }

    pub fn spawn_child(mut self) -> Child {
        self.command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Unable to spawn child process for SicCommandBuilder instance")
    }

    fn with_resources_path(path: &str) -> String {
        setup_input_path(path)
            .into_os_string()
            .into_string()
            .expect("Unable to create test case with resources path")
    }

    fn with_target_path(path: &str) -> String {
        setup_output_path(path)
            .into_os_string()
            .into_string()
            .expect("Unable to create test case with target path")
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
