use std::process::{Child, Command};

trait RunWithArgs {
    fn spawn_process(self) -> std::io::Result<Child>;
}

impl<'a> RunWithArgs for &'a str {
    fn spawn_process(self) -> std::io::Result<Child> {
        let mut command = Command::new("cargo");

        let cmd: String = ["run -- ", self].join("");
        let args = cmd.split_ascii_whitespace().collect::<Vec<&str>>();

        command.args(&args);
        command.spawn()
    }
}

// TODO play with input/output syntax
#[test]
fn batch_with_glob() {
    let cmd = "--glob-input resources/*.png --glob-output target/*.jpg".spawn_process();
    let cmd = cmd.expect("process failed to spawn").wait();

    assert!(cmd.is_ok());
    assert!(cmd.unwrap().success());
}
