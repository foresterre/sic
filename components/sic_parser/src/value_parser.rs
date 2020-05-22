use crate::errors::SicParserError;
use sic_image_engine::wrapper::filter_type::FilterTypeWrap;
use sic_image_engine::wrapper::image_path::ImageFromPath;
use std::convert::TryFrom;
use std::path::PathBuf;

/// The value parser module has a goal to parse image operation inputs.

#[derive(Clone, Debug)]
pub struct Describable<'a>(&'a str);

impl<'a> From<&'a &'a str> for Describable<'a> {
    fn from(v: &'a &'a str) -> Self {
        Describable(v)
    }
}

impl<'a> From<&'a str> for Describable<'a> {
    fn from(v: &'a str) -> Self {
        Describable(v)
    }
}

impl<'a> From<&'a String> for Describable<'a> {
    fn from(v: &'a String) -> Self {
        Describable(v.as_str())
    }
}

/// This iteration of the parse trait should help with unifying the
/// Pest parsing and cli ops arguments directly from &str.
///
/// From Pest we receive iterators, but we can use as_str, to request &str values.
/// We'll try to use this to map the received values as_str, and then we'll have
/// a similar structure as the image operation arguments from the cli (we receive these
/// eventually as Vec<&str>, thus iterable.
pub trait ParseInputsFromIter {
    type Error;

    fn parse<'a, T>(iterable: T) -> Result<Self, Self::Error>
    where
        T: IntoIterator,
        T::Item: Into<Describable<'a>> + std::fmt::Debug,
        Self: std::marker::Sized;
}

macro_rules! parse_next {
    ($iter:expr, $ty:ty, $err_msg:expr) => {
        $iter
            .next()
            .ok_or_else(|| SicParserError::ValueParsingError($err_msg.to_string()))
            .and_then(|v| {
                let v: Describable = v.into();
                v.0.parse::<$ty>()
                    .map_err(|_| SicParserError::ValueParsingError($err_msg.to_string()))
            })?;
    };
}

macro_rules! return_if_complete {
    ($iter:expr, $ok_value:expr, $err_msg:expr) => {
        if $iter.next().is_some() {
            Err(SicParserError::ValueParsingError($err_msg.to_string()))
        } else {
            Ok($ok_value)
        }
    };
}

macro_rules! define_parse_single_input {
    ($ty:ty, $err_msg:expr) => {
        impl ParseInputsFromIter for $ty {
            type Error = SicParserError;

            fn parse<'a, T>(iterable: T) -> Result<Self, Self::Error>
            where
                T: IntoIterator,
                T::Item: Into<Describable<'a>> + std::fmt::Debug,
                Self: std::marker::Sized,
            {
                let mut iter = iterable.into_iter();
                let value = parse_next!(iter, $ty, $err_msg);

                return_if_complete!(iter, value, $err_msg)
            }
        }
    };
}

define_parse_single_input!(f32, "Unable to map a value to f32. v2");
define_parse_single_input!(i32, "Unable to map a value to i32. v2");
define_parse_single_input!(u32, "Unable to map a value to u32. v2");
define_parse_single_input!(bool, "Unable to map a value to bool. v2");

// FIXME(foresterre): define macros for generic tuples and array (i.e. define_parse_multi!((u32, u32));
//                    we can combine the parse_single_input and parse_multi_input as well.

// for: crop
impl ParseInputsFromIter for (u32, u32, u32, u32) {
    type Error = SicParserError;

    fn parse<'a, T>(iterable: T) -> Result<Self, Self::Error>
    where
        T: IntoIterator,
        T::Item: Into<Describable<'a>> + std::fmt::Debug,
        Self: std::marker::Sized,
    {
        let mut iter = iterable.into_iter();
        const ERR_MSG: &str = "Unable to map a value to (u32, u32, u32, u32). v2";

        let res: (u32, u32, u32, u32) = (
            parse_next!(iter, u32, ERR_MSG),
            parse_next!(iter, u32, ERR_MSG),
            parse_next!(iter, u32, ERR_MSG),
            parse_next!(iter, u32, ERR_MSG),
        );

        return_if_complete!(iter, res, ERR_MSG)
    }
}

// for: filter3x3
impl ParseInputsFromIter for [f32; 9] {
    type Error = SicParserError;

    fn parse<'a, T>(iterable: T) -> Result<Self, Self::Error>
    where
        T: IntoIterator,
        T::Item: Into<Describable<'a>> + std::fmt::Debug,
        Self: std::marker::Sized,
    {
        let mut iter = iterable.into_iter();
        const ERR_MSG: &str = "Unable to map a value to [f32; 9]. v2";

        let res: [f32; 9] = [
            //
            parse_next!(iter, f32, ERR_MSG),
            parse_next!(iter, f32, ERR_MSG),
            parse_next!(iter, f32, ERR_MSG),
            //
            parse_next!(iter, f32, ERR_MSG),
            parse_next!(iter, f32, ERR_MSG),
            parse_next!(iter, f32, ERR_MSG),
            //
            parse_next!(iter, f32, ERR_MSG),
            parse_next!(iter, f32, ERR_MSG),
            parse_next!(iter, f32, ERR_MSG),
        ];

        return_if_complete!(iter, res, ERR_MSG)
    }
}

// for: resize
impl ParseInputsFromIter for (u32, u32) {
    type Error = SicParserError;

    fn parse<'a, T>(iterable: T) -> Result<Self, Self::Error>
    where
        T: IntoIterator,
        T::Item: Into<Describable<'a>> + std::fmt::Debug,
        Self: std::marker::Sized,
    {
        let mut iter = iterable.into_iter();
        const ERR_MSG: &str = "Unable to map a value to (u32, u32). v2";

        let res: (u32, u32) = (
            parse_next!(iter, u32, ERR_MSG),
            parse_next!(iter, u32, ERR_MSG),
        );

        return_if_complete!(iter, res, ERR_MSG)
    }
}

// for: unsharpen
impl ParseInputsFromIter for (f32, i32) {
    type Error = SicParserError;

    fn parse<'a, T>(iterable: T) -> Result<Self, Self::Error>
    where
        T: IntoIterator,
        T::Item: Into<Describable<'a>> + std::fmt::Debug,
        Self: std::marker::Sized,
    {
        let mut iter = iterable.into_iter();
        const ERR_MSG: &str = "Unable to map a value to (f32, i32). v2";

        let res: (f32, i32) = (
            parse_next!(iter, f32, ERR_MSG),
            parse_next!(iter, i32, ERR_MSG),
        );

        return_if_complete!(iter, res, ERR_MSG)
    }
}

impl ParseInputsFromIter for String {
    type Error = SicParserError;

    fn parse<'a, T>(iterable: T) -> Result<Self, Self::Error>
    where
        T: IntoIterator,
        T::Item: Into<Describable<'a>> + std::fmt::Debug,
        Self: std::marker::Sized,
    {
        let mut iter = iterable.into_iter();
        const ERR_MSG: &str = "Unable to map a value to (f32, i32). v2";

        let res: Describable<'a> = iter
            .next()
            .ok_or_else(|| SicParserError::ValueParsingError(ERR_MSG.to_string()))?
            .into();

        return_if_complete!(iter, String::from(res.0), ERR_MSG)
    }
}

impl ParseInputsFromIter for ImageFromPath {
    type Error = SicParserError;

    fn parse<'a, T>(iterable: T) -> Result<Self, Self::Error>
    where
        T: IntoIterator,
        T::Item: Into<Describable<'a>> + std::fmt::Debug,
        Self: std::marker::Sized,
    {
        let mut iter = iterable.into_iter();

        let err_msg_no_such_element = || "A path was expected but none was found.".to_string();
        let err_msg_invalid_path =
            || "Unable to construct a valid path for the current platform.".to_string();

        let path = iter
            .next()
            .map(Into::<Describable>::into)
            .ok_or_else(|| SicParserError::ValueParsingError(err_msg_no_such_element()))
            .and_then(|v: Describable| {
                let len = v.0.len();

                // TODO: this is unnecessary: this is necessary when using the outer rule 'string_unicode'
                //  but we should use the inner rule 'string_inner', which spans just the text
                let unquoted = &v.0[1..len - 1];

                PathBuf::try_from(unquoted)
                    .map_err(|_| SicParserError::ValueParsingError(err_msg_invalid_path()))
            })?;

        let err_msg_too_many_elements =
            || "Too many arguments found: a single path was expected".to_string();

        return_if_complete!(iter, ImageFromPath::new(path), err_msg_too_many_elements())
    }
}
impl ParseInputsFromIter for FilterTypeWrap {
    type Error = SicParserError;

    fn parse<'a, T>(iterable: T) -> Result<Self, Self::Error>
    where
        T: IntoIterator,
        T::Item: Into<Describable<'a>> + std::fmt::Debug,
        Self: std::marker::Sized,
    {
        let mut iter = iterable.into_iter();

        let err_msg_no_such_element =
            || "A filter type was expected but none was found.".to_string();

        let filter_type = iter
            .next()
            .map(Into::<Describable>::into)
            .ok_or_else(|| SicParserError::ValueParsingError(err_msg_no_such_element()))
            .and_then(|v: Describable| {
                FilterTypeWrap::try_from_str(v.0).map_err(SicParserError::FilterTypeError)
            })?;

        let err_msg_too_many_elements =
            || "Too many arguments found: a single filter type was expected".to_string();

        return_if_complete!(iter, filter_type, err_msg_too_many_elements())
    }
}

#[cfg(test)]
mod tests_parse_from_iter {
    use super::*;
    use sic_testing::*;

    #[test]
    fn a_f32() {
        let some: f32 = ParseInputsFromIter::parse(&["-1.03"]).unwrap();
        sic_testing::approx_eq_f32!(some, -1.03f32)
    }

    mod tuple_u32_u32_u32_u32 {
        use super::*;

        #[test]
        fn should_succeed_with() {
            let some: (u32, u32, u32, u32) =
                ParseInputsFromIter::parse(&["03579", "0", "1", "1"]).unwrap();
            assert_eq!(some, (3579u32, 0u32, 1u32, 1u32));
        }

        #[pm(input = {
            &["-4", "3", "2", "1"],         // &[a, _b, _c, _d]: a not u32 (neg)
            &["4", "-3", "2", "1"],         // &[_a, b, _c, _d]: b not u32 (neg)
            &["4", "3", "-2", "1"],         // &[_a, _b, c, _d]: c not u32 (neg)
            &["4", "3", "2", "-1"],         // &[_a, _b, _c, d]: d not u32 (neg)
            &["4", "3", "2", "o"],          // &[..., x]: x not f32 (not a number)
            &["4", "3", "2"],               // len() == 4 expected
            &["4", "3", "2", "1", "0"],     // len() == 4 expected
            &[],                            // empty
        })]
        fn expected_failures(input: &[&str]) {
            let result: Result<(u32, u32, u32, u32), SicParserError> =
                ParseInputsFromIter::parse(input);
            assert!(result.is_err());
        }
    }

    mod array_f32x9 {
        use super::*;

        #[test]
        fn array_of_f32() {
            let some: [f32; 9] = ParseInputsFromIter::parse(&[
                "1", "2", "3", "4", "5.5", "-6.0", "7", "8", "-9.9999",
            ])
            .unwrap();
            let expected: [f32; 9] = [
                1f32, 2f32, 3f32, 4f32, 5.5f32, -6.0f32, 7f32, 8f32, -9.9999f32,
            ];
            assert_eq!(some, expected);
        }

        #[pm(input = {
            &["1", "2", "3", "4", "5.5", "-6.0", "7", "8", "lalala"],               // &[..., x]: x not f32 (not a number)
            &["1", "2", "3", "4", "5.5", "-6.0", "7", "8"],                         // len() == 9 expected
            &["1", "2", "3", "4", "5.5", "-6.0", "7", "8", "-9.9999", "1"],         // len() == 9 expected
            &[],                                                                    // empty
        })]
        fn expected_failures(input: &[&str]) {
            let result: Result<[f32; 9], SicParserError> = ParseInputsFromIter::parse(input);
            assert!(result.is_err())
        }
    }

    mod tuple_u32_u32 {
        use super::*;

        #[test]
        fn a_tuple_of_u32_u32() {
            let some: (u32, u32) = ParseInputsFromIter::parse(&["03579", "0"]).unwrap();
            assert_eq!(some, (3579u32, 0u32))
        }

        #[pm(input = {
            &["4", "-0"],   // [_x, y]: y not u32 (neg 0)
            &["4", "-4"],   // [_x, y]: y not u32 (neg number)
            &["4", "f"],    // [_x, y]: y not u32 (not a number)
            &["03579"],     // len() == 2 expected
            &[],            // empty
        })]
        fn expected_failures(input: &[&str]) {
            let result: Result<(u32, u32), SicParserError> = ParseInputsFromIter::parse(input);
            assert!(result.is_err());
        }
    }

    mod tuple_f32_i32 {
        use super::*;

        #[test]
        fn a_tuple_of_f32_i32() {
            let some: (f32, i32) = ParseInputsFromIter::parse(&["-03579.1", "-0"]).unwrap();
            assert_eq!(some, (-3579.1f32, 0i32))
        }

        #[pm(input = {
            &["-f", "-1"],      // [x, _y]: x not f32
            &["-1.0", "f"],     // [_x, y]: y not i32
            &["4"],             // len() == 2 expected
            &["4", "4", "4"],   // len() == 2 expected
            &[],                // empty
        })]
        fn expected_failures(input: &[&str]) {
            let result: Result<(f32, i32), SicParserError> = ParseInputsFromIter::parse(input);
            assert!(result.is_err());
        }
    }
}
