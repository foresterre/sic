//! sic_cli_operations is a lexer and parser for cli arguments, specifically those related to image operations.
//! This parser will complement Clap which is used for ordinary cli arguments.
//! Specifically, it should only parse (image) operations and skip non image operations.
//!
//! Supported:
//! - long args
//! - short args
//!
//! Unsupported:
//! - positional arguments
//! - Invoked program, so the first raw cli argument should be removed from the iterator.
//! - Flag repetitions (e.g. -vvv instead of -v -v -v)
//! - Values given to long args and short args which start with `--` or `-`.

#![deny(
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    nonstandard_style,
    trivial_casts,
    trivial_numeric_casts,
    unused
)]

// Example of input given by args():
//
// If we run sic with `sic --run-external "oxipng -i 3 {input} --some-flag"`, std::env::args() gives us the
// following elements (nth element > arg):
// 0 > sic
// 1 > --run-external
// 2 > oxipng -i 3 {input} --some-flag

#[cfg(test)]
#[macro_use]
extern crate parameterized;

use crate::errors::SicArgError;
use crate::validation::long_arg::{
    LongArgAcceptedCharsValidator, LongArgFirstAndLastCharNotDashesValidator,
    LongArgLengthValidator, LongArgNoConsecutiveDashesValidator,
};
use crate::validation::short_arg::{ShortArgAcceptedCharValidator, ShortArgLengthValidator};
use crate::validation::{validate_argument_name, Validate};

pub mod errors;
pub(crate) mod validation;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    /// White space separated elements
    chunks: &'a [&'a str],

    /// The current chunk
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(cli_args: &'a [&'a str]) -> Self {
        Self {
            chunks: cli_args,
            pos: 0,
        }
    }

    pub fn tokens(&mut self) -> Result<Vec<Token<'a>>, errors::SicArgError> {
        self.collect::<Result<Vec<_>, errors::SicArgError>>()
    }

    fn consume_token(&mut self) -> Result<Token<'a>, errors::SicArgError> {
        if let Some(w) = self.chunks.get(self.pos) {
            let token_result = match w {
                w if w.starts_with("--") => {
                    let name = &w[2..];

                    let v0: &dyn Validate = &LongArgLengthValidator(name);
                    let v1: &dyn Validate = &LongArgAcceptedCharsValidator(name);
                    let v2: &dyn Validate = &LongArgNoConsecutiveDashesValidator(name);
                    let v3: &dyn Validate = &LongArgFirstAndLastCharNotDashesValidator(name);
                    validate_argument_name(&[v0, v1, v2, v3])?;

                    Ok(Token {
                        typ: TokenType::LongArg,
                        slice: name,
                    })
                }
                w if w.starts_with('-') => {
                    let name = &w[1..];

                    let v0: &dyn Validate = &ShortArgLengthValidator(name);
                    let v1: &dyn Validate = &ShortArgAcceptedCharValidator(name);
                    validate_argument_name(&[v0, v1])?;

                    Ok(Token {
                        typ: TokenType::ShortArg,
                        slice: name,
                    })
                }
                w if w.chars().all(|c| !c.is_ascii_control()) => Ok(Token {
                    typ: TokenType::ArgValue,
                    slice: w,
                }),
                err => Err(SicArgError::UnrecognizedToken((*err).to_string())),
            };

            self.pos += 1;

            token_result
        } else {
            Err(SicArgError::TokenizeError)
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, errors::SicArgError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.chunks.len() {
            Some(self.consume_token())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token<'a> {
    typ: TokenType,
    slice: &'a str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenType {
    /// Argument starting with `--`, e.g. `--example`.
    LongArg,

    /// Argument starting with `-`, e.g. `-a`.
    ShortArg,

    /// "" or '' delimited string, or a single word
    ArgValue,
}

#[cfg(test)]
mod tokenizer_tests {
    use crate::{Token, TokenType, Tokenizer};

    mod pm {
        use super::*;

        parameterized::ide!();

        #[parameterized(
            input = {
                "-o",
                "--out",
                "v",
                "x y",
                "x, y"
            },
            expected = {
                Token {
                    typ: TokenType::ShortArg,
                    slice: "o"
                },
                Token {
                    typ: TokenType::LongArg,
                    slice: "out"
                },
                Token {
                    typ: TokenType::ArgValue,
                    slice: "v"
                },
                Token {
                    typ: TokenType::ArgValue,
                    slice: "x y"
                },
                Token {
                    typ: TokenType::ArgValue,
                    slice: "x, y"
                },
            }
        )]
        fn tokens(input: &str, expected: Token) {
            let args = &[input];
            let mut tokenizer = Tokenizer::new(args);
            assert_eq!(tokenizer.tokens().unwrap(), vec![expected]);
        }
    }

    #[test]
    fn token_types() {
        let mut tokenizer = Tokenizer::new(&[
            "-o", "a", "--two", "b", "--three", "--four", "val x", "--five", "val y",
        ]);
        let tokens = tokenizer.tokens();

        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    typ: TokenType::ShortArg,
                    slice: "o"
                },
                Token {
                    typ: TokenType::ArgValue,
                    slice: "a"
                },
                Token {
                    typ: TokenType::LongArg,
                    slice: "two"
                },
                Token {
                    typ: TokenType::ArgValue,
                    slice: "b"
                },
                Token {
                    typ: TokenType::LongArg,
                    slice: "three"
                },
                Token {
                    typ: TokenType::LongArg,
                    slice: "four"
                },
                Token {
                    typ: TokenType::ArgValue,
                    slice: "val x"
                },
                Token {
                    typ: TokenType::LongArg,
                    slice: "five"
                },
                Token {
                    typ: TokenType::ArgValue,
                    slice: "val y"
                }
            ]
        )
    }

    #[test]
    fn token_types_with_long_arg_as_arg_value() {
        let mut tokenizer = Tokenizer::new(&[
            "-i",
            "input.png",
            "--output",
            "out.png",
            "--run-external",
            "oxipng -i 3 {input} --some-flag",
        ]);
        let tokens = tokenizer.tokens();

        assert_eq!(
            tokens.unwrap(),
            vec![
                Token {
                    typ: TokenType::ShortArg,
                    slice: "i"
                },
                Token {
                    typ: TokenType::ArgValue,
                    slice: "input.png"
                },
                Token {
                    typ: TokenType::LongArg,
                    slice: "output"
                },
                Token {
                    typ: TokenType::ArgValue,
                    slice: "out.png"
                },
                Token {
                    typ: TokenType::LongArg,
                    slice: "run-external"
                },
                Token {
                    typ: TokenType::ArgValue,
                    slice: "oxipng -i 3 {input} --some-flag"
                },
            ]
        )
    }
}

/// Known list of operations to parse
#[derive(Copy, Clone, Debug)]
pub enum _OpC {
    Blur,
}
