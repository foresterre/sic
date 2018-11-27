use std::process::{Command, Output};

// For the ProcessWithConfig, ProcessMutWithConfig where we exit early,
// using std::process::exit(0), we can't test using something like:
//
// let arg = vec!["sic", "-H", "index"];
// let matches = get_app().get_matches_from(arg);
// // >> process::exit(0) is called, exits the whole test program; since its the first test
// // >> no tests are run, i.e. reports as 0 test complete
// let is_ok = run(matches);
// assert_eq!(Ok(()), is_ok);
//
// // >> Instead we will use Command for these for now;
// // >> This is suboptimal at best, so probably what we should do in the future
// // >> is that we return for each step some status, like Continue and Complete.
//
// note as of: v. 0.7.2

fn run_help_command(s: &str) -> Output {
    Command::new("cargo")
        .args(&["run", "--", "-H", s])
        .output()
        .expect("Running test failed")
}

#[test]
fn cli_user_manual_run_with_command_index() {
    let res = run_help_command("index");

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with("The following topics are available"));
}

#[test]
fn cli_user_manual_run_with_command_doesnt_exist() {
    let res = Command::new("cargo")
        .args(&["run", "--", "-H", "indexhahahah"])
        .output()
        .expect("Unable to run process");

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with("This topic is unavailable in the user manual"));
}

#[test]
fn cli_user_manual_run_with_command_doesnt_exist_emoji() {
    let res = run_help_command("🧐");

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with("This topic is unavailable in the user manual"));
}

/// Each image command help page currently starts with a h1 title with its the name.
/// To test, whether the correct page is printed, this helper function can be used.
/// Example:
///
/// ```ignore
/// assert!(std::str::from_utf8(&command.stdout).unwrap()
///     .starts_with(&cli_help_page_starts_with_text("blur")));
/// ```
fn cli_help_page_starts_with_text<'a>(page: &'a str) -> String {
    const UNDERLINE: &'static str = "=";
    let amount = page.len();
    let mut h1: Vec<&'a str> = Vec::with_capacity(2 + amount);

    // add title
    h1.push(page);

    // add newline
    h1.push("\n");

    // add underline
    for _ in 0..amount {
        h1.push(UNDERLINE);
    }

    h1.join("")
}

#[test]
fn cli_user_manual_run_with_command_blur() {
    let cmd = "brighten";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_brighten() {
    let cmd = "brighten";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_contrast() {
    let cmd = "contrast";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_filter3x3() {
    let cmd = "filter3x3";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_fliph() {
    let cmd = "fliph";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_flipv() {
    let cmd = "flipv";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_grayscale() {
    let cmd = "grayscale";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_huerotate() {
    let cmd = "huerotate";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_invert() {
    let cmd = "invert";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_resize() {
    let cmd = "resize";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_rotate90() {
    let cmd = "rotate90";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_rotate180() {
    let cmd = "rotate180";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_rotate270() {
    let cmd = "rotate270";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}

#[test]
fn cli_user_manual_run_with_command_unsharpen() {
    let cmd = "unsharpen";
    let res = run_help_command(cmd);

    assert!(res.status.success());
    assert!(std::str::from_utf8(&res.stdout)
        .unwrap()
        .starts_with(&cli_help_page_starts_with_text(cmd)));
}
