const HELP_OPERATION_BLUR: &str = include_str!("../../docs/cli_help_script_blur.txt");

pub struct UserManual;

impl UserManual {
    pub fn help_text(which: &str) -> Result<&str, String> {
        match which {
            "blur" => Ok(HELP_OPERATION_BLUR),
            _ => Err(
                "Help text not available. Potentially, the operation does not exist.".to_string(),
            ),
        }
    }
}
