use thiserror::Error;

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SicArgError {
    #[error("A long argument name (--name) should consist of at least two characters")]
    LongArgTokenLength,

    #[error("A long argument name (--name) should consist of alphanumeric characters, or valid character followed by single dash followed by a valid character")]
    LongArgAcceptedCharacters,

    #[error("The name of a long argument (--name) can't start with a dash or end in a dash")]
    LongArgFirstAndLastCharNotDashes,

    #[error("The name of a long argument (--name) can't consist of consecutive dashes")]
    LongArgNoConsecutiveDashes,

    #[error("Insufficient argument length: a short argument should consist of a single character")]
    ShortArgTokenLength,

    #[error("A short argument should consist of an alphabetic character")]
    ShortArgAcceptedCharacter,

    #[error("Unable to tokenize input")]
    TokenizeError,

    #[error("Token '{0}' was not recognized as a valid token")]
    UnrecognizedToken(String),
}
