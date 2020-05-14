extern crate sic;

const ABOUT: &str = include_str!("../resources/help-pages/about.txt");
const HELP_OPERATIONS_AVAILABLE: &str =
    include_str!("../resources/help-pages/image_operations.txt");
const VERSION: &str = env!("CARGO_PKG_VERSION");

use sic::cli::app::create_app;

fn main() {
    let mut cli = create_app(VERSION, ABOUT, HELP_OPERATIONS_AVAILABLE);

    let program_name = option_env!("SIC_COMPLETIONS_APP_NAME").unwrap_or("sic");

    let shell = match option_env!("SIC_COMPLETIONS_FOR_SHELL").unwrap_or("zsh") {
        "bash" => clap::Shell::Bash,
        "elvish" => clap::Shell::Elvish,
        "fish" => clap::Shell::Fish,
        "powershell" => clap::Shell::PowerShell,
        "zsh" => clap::Shell::Zsh,
        _ => clap::Shell::Zsh,
    };

    let or_out_dir = || {
        std::env::args_os().nth(1).unwrap_or_else(|| {
            eprintln!(
                "No argument found for the output directory, attempting to use the current \
            directory instead..."
            );

            std::env::current_dir()
                .expect("A 'current directory' was not defined")
                .into_os_string()
        })
    };

    let out = option_env!("SIC_COMPLETIONS_OUT_DIR")
        .map(From::from)
        .unwrap_or_else(or_out_dir);

    cli.gen_completions(program_name, shell, out)
}
