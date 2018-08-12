use image::DynamicImage;
use pest::iterators::Pairs;
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
    Blur(u32),
    Brighten(i32),
    FlipHorizontal,
    FlipVertical,
    HueRotate(i32),
    Resize(u32, u32),
    Rotate90,
    Rotate180,
    Rotate270,
}

pub type Operations = Vec<Operation>;

pub fn parse_and_apply_script(image: DynamicImage, script: &str) -> Result<DynamicImage, String> {
    let parsed_script = SICParser::parse(PARSER_RULE, script);
    let rule_pairs: Pairs<Rule> = parsed_script
        .unwrap_or_else(|e| panic!("Unable to parse sic image operations script: {:?}", e));

    let operations: Result<Operations, String> = parse_image_operations(rule_pairs);

    match operations {
        Ok(ops) => apply_operations_on_image(image, &ops),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use operations::test_setup::*;

    #[test]
    fn test_multi_parse_and_apply_script() {
        let image = _setup();
        let script: &str = "flip_horizontal; resize 100 100; blur 3;";

        let result = parse_and_apply_script(image, script);

        assert!(result.is_ok());

        let _ = _manual_inspection(&result.unwrap(), "target/parse_util_apply_all.png");
    }
}
