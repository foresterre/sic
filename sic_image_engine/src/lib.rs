#[cfg(test)]
#[macro_use]
mod test_includes;

pub mod engine;
pub mod wrapper;

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Blur(f32),
    Brighten(i32),
    Contrast(f32),
    Crop(u32, u32, u32, u32),
    Filter3x3([f32; 9]),
    FlipHorizontal,
    FlipVertical,
    GrayScale,
    HueRotate(i32),
    Invert,
    Resize(u32, u32),
    Rotate90,
    Rotate180,
    Rotate270,
    Unsharpen(f32, i32),
}

pub enum OpArg {
    Empty,
    FloatingPoint(f32),
    Integer(i32),
    UnsignedIntegerTuple2(u32, u32),
    UnsignedIntegerTuple4(u32, u32, u32, u32),
    FloatingPointArray9([f32; 9]),
    FloatingPointIntegerTuple2(f32, i32),
}

pub fn operation_by_name(name: &str, value: OpArg) -> Result<Operation, String> {
    match (name, value) {
        ("blur", OpArg::FloatingPoint(v)) => Ok(Operation::Blur(v)),
        ("brighten", OpArg::Integer(v)) => Ok(Operation::Brighten(v)),
        ("contrast", OpArg::FloatingPoint(v)) => Ok(Operation::Contrast(v)),
        ("crop", OpArg::UnsignedIntegerTuple4(u0, u1, u2, u3)) => {
            Ok(Operation::Crop(u0, u1, u2, u3))
        }
        ("filter3x3", OpArg::FloatingPointArray9(v)) => Ok(Operation::Filter3x3(v)),
        ("fliph", OpArg::Empty) => Ok(Operation::FlipHorizontal),
        ("flipv", OpArg::Empty) => Ok(Operation::FlipVertical),
        ("grayscale", OpArg::Empty) => Ok(Operation::GrayScale),
        ("huerotate", OpArg::Integer(v)) => Ok(Operation::HueRotate(v)),
        ("invert", OpArg::Empty) => Ok(Operation::Invert),
        ("resize", OpArg::UnsignedIntegerTuple2(u0, u1)) => Ok(Operation::Resize(u0, u1)),
        ("rotate90", OpArg::Empty) => Ok(Operation::Rotate90),
        ("rotate180", OpArg::Empty) => Ok(Operation::Rotate180),
        ("rotate270", OpArg::Empty) => Ok(Operation::Rotate270),
        ("unsharpen", OpArg::FloatingPointIntegerTuple2(f, i)) => Ok(Operation::Unsharpen(f, i)),
        _ => Err("No suitable operation was found.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // blur
    // ----------

    #[test]
    fn blur_ok() {
        let actual = operation_by_name("blur", OpArg::FloatingPoint(1.5));

        assert_eq!(actual, Ok(Operation::Blur(1.5)));
    }

    #[test]
    fn blur_name_err() {
        let actual = operation_by_name("blur'", OpArg::FloatingPoint(1.5));

        assert_ne!(actual, Ok(Operation::Blur(1.5)));
    }

    #[test]
    fn blur_arg_err() {
        let actual = operation_by_name("blur", OpArg::Empty);

        assert_ne!(actual, Ok(Operation::Blur(1.5)));
    }

    // brighten
    // ----------

    #[test]
    fn brighten_ok() {
        let actual = operation_by_name("brighten", OpArg::Integer(-25));

        assert_eq!(actual, Ok(Operation::Brighten(-25)));
    }

    // contrast
    // ----------

    #[test]
    fn contrast_ok() {
        let actual = operation_by_name("contrast", OpArg::FloatingPoint(1.5));

        assert_eq!(actual, Ok(Operation::Contrast(1.5)));
    }

    // crop
    // ----------

    #[test]
    fn crop_ok() {
        let actual = operation_by_name("crop", OpArg::UnsignedIntegerTuple4(0, 1, 2, 3));

        assert_eq!(actual, Ok(Operation::Crop(0, 1, 2, 3)));
    }

    // filter3x3
    // ----------

    #[test]
    fn filter3x3_ok() {
        let array: [f32; 9] = [1.0; 9];

        let actual = operation_by_name("filter3x3", OpArg::FloatingPointArray9(array));

        assert_eq!(actual, Ok(Operation::Filter3x3(array)));
    }

    // fliph
    // ----------

    #[test]
    fn fliph_ok() {
        let actual = operation_by_name("fliph", OpArg::Empty);

        assert_eq!(actual, Ok(Operation::FlipHorizontal));
    }

    // flipv
    // ----------

    #[test]
    fn flipv_ok() {
        let actual = operation_by_name("flipv", OpArg::Empty);

        assert_eq!(actual, Ok(Operation::FlipVertical));
    }

    // grayscale
    // ----------

    #[test]
    fn grayscale_ok() {
        let actual = operation_by_name("grayscale", OpArg::Empty);

        assert_eq!(actual, Ok(Operation::GrayScale));
    }

    // huerotate
    // ----------

    #[test]
    fn huerotate_ok() {
        let actual = operation_by_name("huerotate", OpArg::Integer(-399));

        assert_eq!(actual, Ok(Operation::HueRotate(-399)));
    }

    // invert
    // ----------

    #[test]
    fn invert_ok() {
        let actual = operation_by_name("invert", OpArg::Empty);

        assert_eq!(actual, Ok(Operation::Invert));
    }

    // resize
    // ----------

    #[test]
    fn resize_ok() {
        let actual = operation_by_name("resize", OpArg::UnsignedIntegerTuple2(80, 40));

        assert_eq!(actual, Ok(Operation::Resize(80, 40)));
    }

    // rotate90
    // ----------

    #[test]
    fn rotate90_ok() {
        let actual = operation_by_name("rotate90", OpArg::Empty);

        assert_eq!(actual, Ok(Operation::Rotate90));
    }

    // rotate180
    // ----------

    #[test]
    fn rotate180_ok() {
        let actual = operation_by_name("rotate180", OpArg::Empty);

        assert_eq!(actual, Ok(Operation::Rotate180));
    }

    // rotate270
    // ----------

    #[test]
    fn rotate270_ok() {
        let actual = operation_by_name("rotate270", OpArg::Empty);

        assert_eq!(actual, Ok(Operation::Rotate270));
    }

    // unsharpen
    // ----------

    #[test]
    fn unsharpen_ok() {
        let actual = operation_by_name("unsharpen", OpArg::FloatingPointIntegerTuple2(1.5, 3));

        assert_eq!(actual, Ok(Operation::Unsharpen(1.5, 3)));
    }
}
