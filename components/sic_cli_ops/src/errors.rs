use sic_parser::errors::SicParserError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SicCliOpsError {
    #[error("Unable to parse: {0}")]
    ParserError(#[from] SicParserError),

    #[error("Failed to parse value of type {typ} ({err})")]
    UnableToParseValueOfType { err: SicParserError, typ: String },

    #[error(
        "Unification of multi valued argument(s) failed: arguments couldn't be \
         partitioned in correct chunk sizes. Length of chunk: {0}"
    )]
    UnableToCorrectlyPartitionMultiParamArguments(usize),

    #[error(
        "Unification of multi valued argument(s) failed: \
        When using an image operation cli argument which requires n values, \
        all values should be provided at once. For example, `--crop` takes 4 values \
        so, n=4. Now, `--crop 0 0 1 1` would be valid, but `--crop 0 0 --crop 1 1` would not."
    )]
    UnableToUnifyMultiValuedArguments,

    #[error("Values which take no arguments can't be unified")]
    UnableToUnifyBareValues,
}
