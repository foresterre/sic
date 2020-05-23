#![cfg(feature = "imageproc-ops")]

#[macro_use]
pub mod common;

#[macro_use]
extern crate parameterized;

#[cfg(test)]
mod tests {
    use crate::common::*;

    ide!();

    #[parameterized(
        ops = {
            r#"draw-text "example" rgba(0,0,0,255) size(24) font("%font%");"#,
        },
        output_file = {
            "imageproc_ops_draw_text"
        },
    )]
    fn check_imageproc_ops_with_script(ops: &str, output_file: &str) {
        let font_file = &[
            env!("CARGO_MANIFEST_DIR"),
            "/resources/font/Lato-Regular.ttf",
        ]
        .concat();

        let ops = ops.replace("%font%", font_file);

        let mut process = command_with_features(
            "unsplash_763569_cropped.jpg",
            format!("{}.png", output_file).as_str(),
            Some("--apply-operations"),
            &ops,
            &["imageproc-ops"],
            false,
        );
        let result = process.wait();
        assert!(result.is_ok());
        assert!(result.unwrap().success());
    }
}
