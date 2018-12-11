use arrayvec::ArrayVec;
use pest::Parser;

use crate::operations::parse::parse_image_operations;

#[cfg(test)]
mod mod_test_includes;

mod parse;
pub(crate) mod transformations;

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
    Crop(u32, u32, u32, u32),
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

pub fn parse_script(script: &str) -> Result<Vec<Operation>, String> {
    let parsed_script = SICParser::parse(PARSER_RULE, script);

    parsed_script
        .map_err(|err| format!("Unable to parse sic image operations script: {:?}", err))
        .and_then(parse_image_operations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_many_args() {
        let input = "blur 15 28;";
        let parsed = SICParser::parse(Rule::main, input);

        // Manually constructing a pest::Error::ParsingError is a hell, because of Position, which,
        // except for the from_start() method is private.
        assert!(parsed.is_err());
    }

    #[test]
    fn test_parsed() {
        let input = "blur 15; flipv";
        let parsed = parse_script(input);

        assert_eq!(
            parsed,
            Ok(vec![Operation::Blur(15.0), Operation::FlipVertical])
        );
    }

    #[test]
    fn test_parsed_fail() {
        let input = "blur 15.7.; flipv";
        let parsed = parse_script(input);

        assert!(parsed.is_err());
    }

}
