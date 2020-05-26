use crate::named_value::NamedValueError;
use sic_image_engine::errors::SicImageEngineError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SicParserError {
    #[error("expected named value with signature '{0}', but got no more inputs")]
    ExpectedNamedValue(String),

    #[error("expected value with of type '{0}', but got no more inputs")]
    ExpectedValue(String),

    #[error("unable to parse filter type: {0}")]
    FilterTypeError(SicImageEngineError),

    #[error("unable to parse named value: {0}")]
    NamedValueParsingError(NamedValueError),

    #[error("string value expected an inner value, but none was found")]
    NoInnerString,

    #[error("{0}")]
    OperationError(OperationParamError),

    #[error("unable to parse script: {0}")]
    PestGrammarError(String),

    #[error("parsing failed: operation doesn't exist")]
    UnknownOperationError,

    #[error("unable to parse value '{0}'")]
    ValueParsingError(String),
}

#[derive(Debug, Error)]
pub enum OperationParamError {
    #[error(
        "Unable to parse `set` environment command. Error: expected a single `set` inner element."
    )]
    SetEnvironment,

    #[error("Unable to parse `set` environment command. Error on element: {0}")]
    SetEnvironmentElement(String),

    #[error("Unable to parse operation argument(s): {0}")]
    PestArgError(String),

    #[error(
        "Unable to parse `del` environment command. Error: expected a single `del` inner element."
    )]
    UnsetEnvironment,

    #[error("Unable to parse `del` environment command. Error on element: {0}")]
    UnsetEnvironmentElement(String),
}
