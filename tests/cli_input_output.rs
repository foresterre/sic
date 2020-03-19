use std::process::{Child, Command};

#[macro_use]
pub mod common;

use crate::common::*;

#[derive(Copy, Clone)]
enum RunWithIOArg {
    BothIO,
    NeitherIO,
    OnlyI,
    OnlyO,
}

impl RunWithIOArg {
    fn both(self, input: &str, output: &str) -> Command {
        let mut command = Command::new("cargo");
        command.args(&["run", "--", "-i", input, "-o", output]);
        command
    }

    fn neither(self, input: &str, output: &str) -> Command {
        let mut command = Command::new("cargo");
        command.args(&["run", "--", input, output]);
        command
    }

    fn only_i(self, input: &str, output: &str) -> Command {
        let mut command = Command::new("cargo");
        command.args(&["run", "--", "-i", input, output]);
        command
    }

    fn only_o(self, input: &str, output: &str) -> Command {
        let mut command = Command::new("cargo");
        command.args(&["run", "--", "-o", output, input]);
        command
    }

    fn start(self, input: &str, output: &str) -> std::io::Result<Child> {
        match self {
            RunWithIOArg::BothIO => self.both(input, output).spawn(),
            RunWithIOArg::OnlyI => self.only_i(input, output).spawn(),
            RunWithIOArg::OnlyO => self.only_o(input, output).spawn(),
            RunWithIOArg::NeitherIO => self.neither(input, output).spawn(),
        }
    }
}

#[test]
fn both_i_and_o_args() {
    let kind = RunWithIOArg::BothIO;
    let input = String::from(setup_input_path("palette_4x4.png").to_str().unwrap());
    let output = String::from(setup_output_path("io.jpg").to_str().unwrap());
    let result = kind.start(&input, &output).expect("process").wait();

    assert!(result.is_ok());
    assert!(result.unwrap().success());
}

#[test]
fn neither_i_and_o_args() {
    let kind = RunWithIOArg::NeitherIO;
    let input = String::from(setup_input_path("palette_4x4.png").to_str().unwrap());
    let output = String::from(setup_output_path("not_io.jpg").to_str().unwrap());
    let result = kind.start(&input, &output).expect("process").wait();

    assert!(result.is_ok());
    assert!(result.unwrap().success());
}

#[test]
fn only_i() {
    let kind = RunWithIOArg::OnlyI;
    let input = String::from(setup_input_path("palette_4x4.png").to_str().unwrap());
    let output = String::from(setup_output_path("iii.jpg").to_str().unwrap());
    let result = kind.start(&input, &output).expect("process").wait();

    assert!(result.is_ok());

    // expect a non zero exit status
    assert_not!(result.unwrap().success());
}

#[test]
fn only_o() {
    let kind = RunWithIOArg::OnlyO;
    let input = String::from(setup_input_path("palette_4x4.png").to_str().unwrap());
    let output = String::from(setup_output_path("ooo.jpg").to_str().unwrap());
    let result = kind.start(&input, &output).expect("process").wait();

    assert!(result.is_ok());

    // expect a non zero exit status
    assert_not!(result.unwrap().success());
}
