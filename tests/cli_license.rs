use std::process::{Command, Output};

fn run_license_command() -> Output {
    Command::new("cargo")
        .args(&["run", "--", "--license"])
        .output()
        .expect("Running test failed")
}

#[test]
fn cli_license_full() {
    let res = run_license_command();

    let parts = vec![
        "Simple Image Converter license: \n\n",
        include_str!("../LICENSE"),
        "\n\n\n",
    ];

    assert!(res.status.success());
    assert_eq!(&parts.join(""), std::str::from_utf8(&res.stdout).unwrap());
}
