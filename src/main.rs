use sic_lib::{get_app, run};

fn main() -> Result<(), String> {
    let matches = get_app().get_matches();

    run(matches)
}
