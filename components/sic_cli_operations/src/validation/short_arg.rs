//! Validators which check whether the given characters make the argument a valid short argument token.
//! A short argument has the following shape: `-x`, where `x` satisfies the following properties:
//! * |x| = 1
//! * x is alphabetic

use crate::errors::SicArgError;
use crate::validation::Validate;

pub struct ShortArgLengthValidator<'a>(pub(crate) &'a str);

impl Validate for ShortArgLengthValidator<'_> {
    fn validate(&self) -> Result<(), SicArgError> {
        if self.0.len() == 1 {
            Ok(())
        } else {
            Err(SicArgError::ShortArgTokenLength)
        }
    }
}

#[cfg(test)]
mod length_validator {
    use super::*;

    #[test]
    fn happy() {
        let v = ShortArgLengthValidator("a");
        let validation = v.validate();
        assert!(validation.is_ok())
    }

    #[test]
    fn sad() {
        let v = ShortArgLengthValidator("aa");
        let validation = v.validate();
        assert_eq!(validation.unwrap_err(), SicArgError::ShortArgTokenLength)
    }
}

pub struct ShortArgAcceptedCharValidator<'a>(pub(crate) &'a str);

impl Validate for ShortArgAcceptedCharValidator<'_> {
    fn validate(&self) -> Result<(), SicArgError> {
        if let Some(c) = self.0.chars().next() {
            if c.is_alphabetic() {
                return Ok(());
            }
        }

        Err(SicArgError::ShortArgAcceptedCharacter)
    }
}

#[cfg(test)]
mod accepted_char {
    use super::*;

    parameterized::ide!();

    #[parameterized(
        input = {
            "a",
            "z",
            "ß",
        }
    )]
    fn happy(input: &str) {
        let v = ShortArgAcceptedCharValidator(input);
        let validation = v.validate();
        assert!(validation.is_ok())
    }

    #[parameterized(
        input = {
            "!",
            "1",
            "",
        }
    )]
    fn sad(input: &str) {
        let v = ShortArgAcceptedCharValidator(input);
        let validation = v.validate();
        assert_eq!(
            validation.unwrap_err(),
            SicArgError::ShortArgAcceptedCharacter
        )
    }
}
