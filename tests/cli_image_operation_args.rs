#[macro_use]
pub mod common;

// fixme: add additional integration tests

#[cfg(test)]
mod blur {
    use crate::common::*;

    //    FIXME(image-rs/image#983): blur panics on option unwrap within image::imageops::sample::vertical_sample.
    //    #[test]
    //    fn blur_0() {
    //        let mut process = command(DEFAULT_IN, "img_op_arg.png", "--blur 0");
    //        let result = process.wait();
    //        assert!(result.is_ok());
    //        assert!(result.unwrap().success());
    //    }

    #[test]
    fn blur_1() {
        let mut process = command(DEFAULT_IN, "img_op_arg.png", "--blur 1");
        let result = process.wait();
        assert!(result.is_ok());
        assert!(result.unwrap().success());
    }

    #[test]
    fn blur_1_dot_1() {
        let mut process = command(DEFAULT_IN, "img_op_arg.png", "--blur 1.1");
        let result = process.wait();
        assert!(result.is_ok());
        assert!(result.unwrap().success());
    }

    #[test]
    fn blur_neg_1_dot_1() {
        let mut process = command(DEFAULT_IN, "img_op_arg.png", "--blur -1.1");
        let result = process.wait();
        assert!(result.is_ok());
        assert!(result.unwrap().success());
    }
}

#[cfg(test)]
mod crop {
    use crate::common::*;

    #[test]
    fn crop_simple() {
        let mut process = command(DEFAULT_IN, "img_op_arg.png", "--crop 0 0 1 1");
        let result = process.wait();
        assert!(result.is_ok());
        assert!(result.unwrap().success());
    }

    #[test]
    fn dont_allow_separated_values() {
        let mut process = command(DEFAULT_IN, "img_op_arg.png", "--crop 0 0 --crop 1 1");
        let result = process.wait();
        assert!(result.is_ok());
        assert_not!(result.unwrap().success());
    }

    #[test]
    fn incorrect_amount_of_values() {
        let mut process = command(DEFAULT_IN, "img_op_arg.png", "--crop 0 0 1");
        let result = process.wait();
        assert!(result.is_ok());
        assert_not!(result.unwrap().success());
    }

    #[test]
    fn crop_multiple_ok() {
        let mut process = command(
            DEFAULT_IN,
            "img_op_arg.png",
            "--crop 2 2 3 3 --crop 0 0 1 1",
        );
        let result = process.wait();
        assert!(result.is_ok());
        assert!(result.unwrap().success());
    }

    // The following will succeed, even though it shouldn't,
    // however since this part (parsing of the directly given cli arguments) is handled by Clap,
    // we can't detect it (without parsing the argv ourselves).
    //
    // Clap gives us for a cli argument, i.e. --crop only the amount of values and the indices of the
    // values (as far as I am aware). Since we don't known how much times --crop was provided, we
    // don't have the information we need to solve this issue.
    //
    // Perhaps in the future, we will check argv ourselves, or find another solution.
    #[test]
    fn crop_multiple_one_empty() {
        let mut process = command(
            DEFAULT_IN,
            "img_op_arg.png",
            "--crop --crop 2 2 3 3 --crop 0 0 1 1",
        );
        let result = process.wait();
        assert!(result.is_ok());

        // Here we would like to assert_not! instead
        assert!(result.unwrap().success());
    }

    // This one however will fail, as we tell Clap we require 4 values for this cli argument.
    #[test]
    fn crop_empty() {
        let mut process = command(DEFAULT_IN, "img_op_arg.png", "--crop");
        let result = process.wait();
        assert!(result.is_ok());

        assert_not!(result.unwrap().success());
    }

    #[test]
    fn crop_too_few() {
        let mut process = command(DEFAULT_IN, "img_op_arg.png", "--crop 2 2 2");
        let result = process.wait();
        assert!(result.is_ok());

        assert_not!(result.unwrap().success());
    }

    #[test]
    fn crop_too_many() {
        let mut process = command(DEFAULT_IN, "img_op_arg.png", "--crop 2 2 2 2 2");
        let result = process.wait();
        assert!(result.is_ok());

        assert_not!(result.unwrap().success());
    }

}
