#[macro_use]
extern crate pest_derive;

use pest::Parser;
use sic_image_engine::engine::Program;

use crate::rule_parser::parse_image_operations;

pub mod rule_parser;
pub mod value_parser;

const PARSER_RULE: Rule = Rule::main;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SICParser;

pub fn parse_script(script: &str) -> Result<Program, String> {
    let parsed_script = SICParser::parse(PARSER_RULE, script);

    parsed_script
        .map_err(|err| format!("Unable to parse sic image operations script: {:?}", err))
        .and_then(parse_image_operations)
}

#[cfg(test)]
mod tests {
    use sic_image_engine::engine::Statement;
    use sic_image_engine::Operation;

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

        assert!(parsed.is_ok());

        assert_eq!(
            parsed.unwrap(),
            vec![
                Statement::Operation(Operation::Blur(15.0)),
                Statement::Operation(Operation::FlipVertical)
            ]
        );
    }

    #[test]
    fn test_parsed_fail() {
        let input = "blur 15.7.; flipv";
        let parsed = parse_script(input);

        assert!(parsed.is_err());
    }
}
