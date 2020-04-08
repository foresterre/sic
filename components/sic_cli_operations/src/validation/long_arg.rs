//! Validators which check whether the given characters make the argument a valid long argument token.
//! A long argument has the following shape: `--name`, where `name` satisfies the following properties:
//! * |name| >= 2
//! * name is alphanumeric or a dash
//! * name doesn't contain a consecutive dash
//! * name's first and last characters aren't dashes

use crate::errors::SicArgError;
use crate::validation::Validate;

pub(crate) const DASH: char = '-';

pub struct LongArgLengthValidator<'a>(pub(crate) &'a str);

impl Validate for LongArgLengthValidator<'_> {
    fn validate(&self) -> Result<(), SicArgError> {
        if self.0.len() >= 2 {
            Ok(())
        } else {
            Err(SicArgError::LongArgTokenLength)
        }
    }
}

#[cfg(test)]
mod length_validator {
    use super::*;

    #[test]
    fn happy() {
        let validator = LongArgLengthValidator("aa");
        assert!(validator.validate().is_ok())
    }

    #[parameterized(
        input = {
            "a",
            ""
        }
    )]
    fn sad(input: &str) {
        let validator = LongArgLengthValidator(input);
        assert_eq!(
            validator.validate().unwrap_err(),
            SicArgError::LongArgTokenLength
        )
    }
}

pub struct LongArgAcceptedCharsValidator<'a>(pub(crate) &'a str);

impl Validate for LongArgAcceptedCharsValidator<'_> {
    fn validate(&self) -> Result<(), SicArgError> {
        if self.0.chars().all(|c| c.is_alphanumeric() || c == DASH) {
            Ok(())
        } else {
            Err(SicArgError::LongArgAcceptedCharacters)
        }
    }
}

#[cfg(test)]
mod accepted_chars_validator {
    use super::*;

    parameterized::ide!();

    #[parameterized(
        input = {
            "a",
            "1",
            "-",
            "a-1z",
            "ß"
        }
    )]
    fn happy(input: &str) {
        let validator = LongArgAcceptedCharsValidator(input);
        assert!(validator.validate().is_ok())
    }

    #[parameterized(
        input = {
            ",",
            ".",
            "!",
            "~",
            "—", // em dash
            "―", // horizontal bar
            "‒", // figure dash
            "​", // zero width space
        }
    )]
    fn sad(input: &str) {
        let validator = LongArgAcceptedCharsValidator(input);
        assert_eq!(
            validator.validate().unwrap_err(),
            SicArgError::LongArgAcceptedCharacters
        )
    }
}

pub struct LongArgNoConsecutiveDashesValidator<'a>(pub(crate) &'a str);

impl Validate for LongArgNoConsecutiveDashesValidator<'_> {
    fn validate(&self) -> Result<(), SicArgError> {
        let windows: Vec<char> = self.0.chars().collect();
        let windows = windows.windows(2);

        for window in windows {
            let lhs = window.get(0);
            let rhs = window.get(1);

            if let Some(l) = lhs {
                if let Some(r) = rhs {
                    if *l == DASH && *r == DASH {
                        return Err(SicArgError::LongArgNoConsecutiveDashes);
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod no_consecutive_dashes_validator {
    use super::*;

    parameterized::ide!();

    #[parameterized(
        input = {
            "a-a",
            "-a-",
            "a-a-",
            "-a-a",
        }
    )]
    fn happy(input: &str) {
        let validator = LongArgNoConsecutiveDashesValidator(input);
        assert!(validator.validate().is_ok());
    }

    #[parameterized(
        input = {
            "a--a",
            "a--",
            "--a",
        }
    )]
    fn sad(input: &str) {
        let validator = LongArgNoConsecutiveDashesValidator(input);
        assert_eq!(
            validator.validate().unwrap_err(),
            SicArgError::LongArgNoConsecutiveDashes
        );
    }
}

pub struct LongArgFirstAndLastCharNotDashesValidator<'a>(pub(crate) &'a str);

impl Validate for LongArgFirstAndLastCharNotDashesValidator<'_> {
    fn validate(&self) -> Result<(), SicArgError> {
        let first = self.0.chars().next();
        let last = self.0.chars().next_back();

        match (first, last) {
            (Some(first), Some(last)) if first != DASH && last != DASH => Ok(()),
            _ => Err(SicArgError::LongArgFirstAndLastCharNotDashes),
        }
    }
}

#[cfg(test)]
mod first_and_last_char_not_dashes_validator {
    use super::*;

    #[test]
    fn happy() {
        let validator = LongArgFirstAndLastCharNotDashesValidator("a-----a");
        assert!(validator.validate().is_ok());
    }

    #[parameterized(
        input = {
            "-",
            "-a",
            "a-",
            "-a-"
        }
    )]
    fn sad(input: &str) {
        let validator = LongArgFirstAndLastCharNotDashesValidator(input);
        assert_eq!(
            validator.validate().unwrap_err(),
            SicArgError::LongArgFirstAndLastCharNotDashes
        );
    }
}
