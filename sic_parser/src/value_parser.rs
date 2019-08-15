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
            .ok_or_else(|| $err_msg.to_string())
            .and_then(|v| {
                let v: Describable = v.into();
                v.0.parse::<$ty>().map_err(|_| $err_msg.to_string())
            })?;
    };
}

macro_rules! return_if_complete {
    ($iter:expr, $ok_value:expr, $err_msg:expr) => {
        if let Some(_) = $iter.next() {
            Err($err_msg.to_string())
        } else {
            Ok($ok_value)
        }
    };
}

macro_rules! define_parse_single_input {
    ($ty:ty, $err_msg:expr) => {
        impl ParseInputsFromIter for $ty {
            type Error = String;

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
    type Error = String;

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
    type Error = String;

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
    type Error = String;

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
    type Error = String;

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
    type Error = String;

    fn parse<'a, T>(iterable: T) -> Result<Self, Self::Error>
    where
        T: IntoIterator,
        T::Item: Into<Describable<'a>> + std::fmt::Debug,
        Self: std::marker::Sized,
    {
        let mut iter = iterable.into_iter();
        const ERR_MSG: &str = "Unable to map a value to (f32, i32). v2";

        let res: Describable<'a> = iter.next().ok_or_else(|| ERR_MSG.to_string())?.into();

        return_if_complete!(iter, String::from(res.0), ERR_MSG)
    }
}

#[cfg(test)]
mod tests_parse_from_iter {
    use super::*;

    #[test]
    fn a_f32() {
        let some: f32 = ParseInputsFromIter::parse(&["-1.03"]).unwrap();
        assert_eq!(some, -1.03f32)
    }

    mod tuple_u32_u32_u32_u32 {
        use super::*;

        #[test]
        fn a_tuple_of_u32_u32_u32_u32() {
            let some: (u32, u32, u32, u32) =
                ParseInputsFromIter::parse(&["03579", "0", "1", "1"]).unwrap();
            assert_eq!(some, (3579u32, 0u32, 1u32, 1u32))
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_u32_u32_u32_u32_fail_on_neg() {
            let _some: (u32, u32, u32, u32) =
                ParseInputsFromIter::parse(&["03579", "-0", "1", "1"]).unwrap();
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_u32_u32_u32_u32_fail_on_length_too_short() {
            let _some: (u32, u32, u32, u32) =
                ParseInputsFromIter::parse(&["03579", "-0", "1"]).unwrap();
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_u32_u32_u32_u32_fail_on_length_too_long() {
            let _some: (u32, u32, u32, u32) =
                ParseInputsFromIter::parse(&["03579", "0", "1", "1", "1"]).unwrap();
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_u32_u32_u32_u32_fail_not_u32() {
            let _some: (u32, u32, u32, u32) =
                ParseInputsFromIter::parse(&["03579", "0", "1", "o"]).unwrap();
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

        #[test]
        #[should_panic]
        fn array_of_f32_oof_too_short() {
            let _some: [f32; 9] =
                ParseInputsFromIter::parse(&["1", "2", "3", "4", "5.5", "-6.0", "7", "8"]).unwrap();
        }

        #[test]
        #[should_panic]
        fn array_of_f32_oof_too_long() {
            let _some: [f32; 9] = ParseInputsFromIter::parse(&[
                "1", "2", "3", "4", "5.5", "-6.0", "7", "8", "-9.9999", "1",
            ])
            .unwrap();
        }

        #[test]
        #[should_panic]
        fn array_of_f32_oof_not_a_f32() {
            let _some: [f32; 9] = ParseInputsFromIter::parse(&[
                "1", "2", "3", "4", "5.5", "-6.0", "7", "8", "lalala",
            ])
            .unwrap();
        }
    }

    mod tuple_u32_u32 {
        use super::*;

        #[test]
        fn a_tuple_of_u32_u32() {
            let some: (u32, u32) = ParseInputsFromIter::parse(&["03579", "0"]).unwrap();
            assert_eq!(some, (3579u32, 0u32))
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_u32_u32_fail_on_neg() {
            let _some: (u32, u32) = ParseInputsFromIter::parse(&["03579", "-0"]).unwrap();
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_u32_u32_fail_on_empty() {
            let empty: &[&str; 0] = &[];
            let _some: (u32, u32) = ParseInputsFromIter::parse(empty).unwrap();
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_u32_u32_fail_on_too_short() {
            let _some: (u32, u32) = ParseInputsFromIter::parse(&["03579"]).unwrap();
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_u32_u32_fail_on_too_long() {
            let _some: (u32, u32) = ParseInputsFromIter::parse(&["03579", "1", "1"]).unwrap();
        }
    }

    mod tuple_f32_i32 {
        use super::*;

        #[test]
        fn a_tuple_of_f32_i32() {
            let some: (f32, i32) = ParseInputsFromIter::parse(&["-03579.1", "-0"]).unwrap();
            assert_eq!(some, (-3579.1f32, 0i32))
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_f32_i32_fail_on_not_f32() {
            let _some: (f32, i32) = ParseInputsFromIter::parse(&["f", "-1"]).unwrap();
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_f32_i32_fail_on_not_i32() {
            let _some: (f32, i32) = ParseInputsFromIter::parse(&["-1.0", "f"]).unwrap();
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_f32_i32_fail_on_empty() {
            let empty: &[&str; 0] = &[];
            let _some: (f32, i32) = ParseInputsFromIter::parse(empty).unwrap();
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_f32_i32_fail_on_too_short() {
            let _some: (f32, i32) = ParseInputsFromIter::parse(&["03579"]).unwrap();
        }

        #[test]
        #[should_panic]
        fn a_tuple_of_f32_i32_fail_on_too_long() {
            let _some: (f32, i32) = ParseInputsFromIter::parse(&["03579", "1", "1"]).unwrap();
        }
    }

}
