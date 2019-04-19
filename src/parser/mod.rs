use combostew::operations::engine::Program;
use pest::Parser;

use crate::parser::parse::parse_image_operations;

mod parse;

const PARSER_RULE: Rule = Rule::main;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct SICParser;

pub fn parse_script(script: &str) -> Result<Program, String> {
    let parsed_script = SICParser::parse(PARSER_RULE, script);

    parsed_script
        .map_err(|err| format!("Unable to parse sic image operations script: {:?}", err))
        .and_then(parse_image_operations)
}

#[cfg(test)]
mod tests {
    use combostew::operations::engine::Statement;
    use combostew::operations::Operation;

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
