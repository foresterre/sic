pub mod app;
pub mod user_manual;

pub fn get_tool_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
