//! pest parser combining sic_parser and sic_cli_ops
//!
//! Makes the pest parser more universal, and less specific (ie the operation name isn't part of the
//! grammar any more). This also helps with supporting plugins (dev) in a single workflow.
//!
//! Planned to become version 0.x for sic_parser and after stability testing 1.0
//!
//! TODO: setup compatibility testing against the current binaries for both `cli_ops` and `parser`.

#[macro_use]
extern crate pest_derive;

pub mod errors;

use crate::errors::SicDuoError;
use pest::Parser;

// In-crate variant of sic_image_engine::Instr. sic_image_engine::Instr should implement `From<Op>`:
// `impl From<sic_duo::Op> for Instr { ... }`.
// This way, the parser crate will be independent from the image engine crate.
pub enum Op {}

#[derive(Parser)]
#[grammar = "grammar2.pest"]
pub struct SICParser;

const PARSER_MODULE_ENTRY: Rule = Rule::module;

// TODO: introduce sic_imgland::Instr here?
pub fn parse_script(script: &str) -> Result<Vec<Op>, SicDuoError> {
    let parsed_script = SICParser::parse(PARSER_MODULE_ENTRY, script);

    parsed_script
        .map_err(|_err| SicDuoError::UnimplementedError("PEGParse (1)".to_string()))
        .and_then(parse_image_operations)
}

pub fn parse_image_operations(
    _pairs: pest::iterators::Pairs<'_, Rule>,
) -> Result<Vec<Op>, SicDuoError> {
    Err(SicDuoError::UnimplementedError(
        "PairsToAST (2)".to_string(),
    ))
}
