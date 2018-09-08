use arrayvec::ArrayVec;
use image::DynamicImage;
use pest::Parser;

use operations::operations::apply_operations_on_image;
use operations::parse::parse_image_operations;

#[cfg(test)]
mod test_setup;

mod operations;
mod parse;

// ensure grammar refreshes on compile
const _GRAMMAR: &str = include_str!("grammar.pest");
const PARSER_RULE: Rule = Rule::main;

#[derive(Parser)]
#[grammar = "operations/grammar.pest"]
struct SICParser;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Blur(f32),
    Brighten(i32),
    Contrast(f32),
    Filter3x3(ArrayVec<[f32; 9]>),
    FlipHorizontal,
    FlipVertical,
    GrayScale,
    HueRotate(i32),
    Invert,
    Resize(u32, u32),
    Rotate90,
    Rotate180,
    Rotate270,
    Unsharpen(f32, i32),
}

pub type Operations = Vec<Operation>;

pub fn parse_and_apply_script(image: DynamicImage, script: &str) -> Result<DynamicImage, String> {
    let parsed_script = SICParser::parse(PARSER_RULE, script);

    parsed_script
        .map_err(|err| format!("Unable to parse sic image operations script: {:?}", err))
        .and_then(|pairs| parse_image_operations(pairs))
        .and_then(|operations| apply_operations_on_image(image, &operations))
}

#[cfg(test)]
mod tests {
    use super::*;
    use operations::test_setup::*;

    #[test]
    fn test_too_many_args() {
        let input = "blur 15 28;";
        let parsed = SICParser::parse(Rule::main, input);

        // Manually constructing a pest::Error::ParsingError is a hell, because of Position, which,
        // except for the from_start() method is private.
        assert!(parsed.is_err());
    }

    #[test]
    fn test_multi_parse_and_apply_script() {
        let image = setup_default_test_image();
        let script: &str = "fliph; resize 100 100; blur 3;";

        let result = parse_and_apply_script(image, script);

        assert!(result.is_ok());

        let _ = output_test_image_for_manual_inspection(&result.unwrap(), "target/parse_util_apply_all.png");
    }
}
