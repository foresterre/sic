extern crate clap;
extern crate sic_lib;

fn main() {
    let mut cli = sic_lib::app::cli::cli();

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
            std::env::current_dir()
                .expect("Unable to receive current directory.")
                .into_os_string()
        })
    };

    let out = option_env!("SIC_COMPLETIONS_OUT_DIR")
        .map(From::from)
        .unwrap_or_else(or_out_dir);

    cli.gen_completions(program_name, shell, out)
}
