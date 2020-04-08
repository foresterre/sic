use crate::errors;
use crate::errors::SicArgError;

pub mod long_arg;
pub mod short_arg;

pub trait Validate {
    fn validate(&self) -> Result<(), errors::SicArgError>;
}

pub fn validate_argument_name<'a>(validators: &'a [&'a dyn Validate]) -> Result<(), SicArgError> {
    for validator in validators {
        if let e @ Err(_) = validator.validate() {
            return e;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::short_arg::{ShortArgAcceptedCharValidator, ShortArgLengthValidator};

    #[test]
    fn check_with_ok() {
        let key = "a";
        let v1: &dyn Validate = &ShortArgLengthValidator(key);
        let v2: &dyn Validate = &ShortArgAcceptedCharValidator(key);

        let validation = validate_argument_name(&[v1, v2]);

        assert!(validation.is_ok())
    }

    #[test]
    fn check_with_err() {
        let key = "!";
        let v1: &dyn Validate = &ShortArgLengthValidator(key);
        let v2: &dyn Validate = &ShortArgAcceptedCharValidator(key);

        let validation = validate_argument_name(&[v1, v2]);

        assert!(validation.is_err())
    }
}
