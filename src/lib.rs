pub mod app;
pub mod help;
pub mod sic_processor;

pub fn get_tool_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
