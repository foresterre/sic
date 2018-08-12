use image::{DynamicImage, FilterType};

use super::Operation;

pub trait ApplyOperation<O, T, E> {
    fn apply_operation(&self, operation: &O) -> Result<T, E>;
}

impl ApplyOperation<Operation, DynamicImage, String> for DynamicImage {
    fn apply_operation(&self, operation: &Operation) -> Result<DynamicImage, String> {
        match *operation {
            Operation::Blur(sigma) => Ok(self.blur(sigma as f32)),
            Operation::Brighten(amount) => Ok(self.brighten(amount)),
            Operation::FlipHorizontal => Ok(self.fliph()),
            Operation::FlipVertical => Ok(self.flipv()),
            Operation::HueRotate(degree) => Ok(self.huerotate(degree)),
            Operation::Resize(new_x, new_y) => {
                Ok(self.resize_exact(new_x, new_y, FilterType::Gaussian))
            }
            Operation::Rotate90 => Ok(self.rotate90()),
            Operation::Rotate270 => Ok(self.rotate270()),
            Operation::Rotate180 => Ok(self.rotate180()),
        }
    }
}

pub fn apply_operations_on_image(
    image: DynamicImage,
    operations: &[Operation],
) -> Result<DynamicImage, String> {
    // this should be possible clean and nice and functional, but right now, I can't come up with it.

    let mut mut_img: DynamicImage = image;

    for op in operations.iter() {
        let op_status = mut_img.apply_operation(op);

        if op_status.is_err() {
            return op_status.map_err(|err| err.to_string());
        } else {
            mut_img = op_status.unwrap();
        }
    }

    Ok(mut_img)
}

#[cfg(test)]
mod tests {
    use super::*;
    use operations::test_setup::*;
    use image::GenericImage;

    #[test]
    fn test_blur() {
        let img: DynamicImage = _setup();
        let operation = Operation::Blur(25);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        _manual_inspection(&done.unwrap(), "target/test_blur.png")
    }

    #[test]
    fn test_brighten_pos() {
        let img: DynamicImage = _setup();
        let cmp: DynamicImage = _setup();

        let operation = Operation::Brighten(25);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        _manual_inspection(&result_img, "target/test_brighten_pos_25.png")
    }

    #[test]
    fn test_brighten_zero() {
        let img: DynamicImage = _setup();
        let cmp: DynamicImage = _setup();
        let operation = Operation::Brighten(0);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_eq!(cmp.raw_pixels(), result_img.raw_pixels());

        _manual_inspection(&result_img, "target/test_brighten_zero.png")
    }

    #[test]
    fn test_brighten_neg() {
        let img: DynamicImage = _setup();
        let cmp: DynamicImage = _setup();

        let operation = Operation::Brighten(-25);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        _manual_inspection(&result_img, "target/test_brighten_neg_25.png")
    }

    #[test]
    fn test_flip_h() {
        let img: DynamicImage = _setup();
        let operation = Operation::FlipHorizontal;

        let (xa, ya) = img.dimensions();
        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, xb);
        assert_eq!(ya, yb);

        _manual_inspection(&img_result, "target/test_fliph.png")
    }

    #[test]
    fn test_flip_v() {
        let img: DynamicImage = _setup();
        let operation = Operation::FlipVertical;

        let (xa, ya) = img.dimensions();
        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, xb);
        assert_eq!(ya, yb);

        _manual_inspection(&img_result, "target/test_flipv.png")
    }

    #[test]
    fn test_hue_rotate_neg() {
        let img: DynamicImage = _setup();
        let cmp: DynamicImage = _setup();

        let operation = Operation::HueRotate(-100);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        _manual_inspection(&result_img, "target/test_hue_rot_neg_100.png")
    }

    #[test]
    fn test_hue_rotate_pos() {
        let img: DynamicImage = _setup();
        let cmp: DynamicImage = _setup();

        let operation = Operation::HueRotate(100);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        _manual_inspection(&result_img, "target/test_hue_rot_pos_100.png")
    }

    #[test]
    fn test_hue_rotate_zero() {
        let img: DynamicImage = _setup();
        let cmp: DynamicImage = _setup();

        let operation = Operation::HueRotate(0);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_eq!(cmp.raw_pixels(), result_img.raw_pixels());

        _manual_inspection(&result_img, "target/test_brighten_0.png")
    }

    #[test]
    fn test_hue_rotate_360() {
        let img: DynamicImage = _setup();
        let cmp: DynamicImage = _setup();

        let operation = Operation::HueRotate(360);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        // https://docs.rs/image/0.19.0/image/enum.DynamicImage.html#method.huerotate
        // huerotate(0) should be huerotate(360), but this doesn't seem the case
        assert_eq!(cmp.huerotate(360).raw_pixels(), result_img.raw_pixels());

        _manual_inspection(&result_img, "target/test_brighten_360.png")
    }

    #[test]
    fn test_hue_rotate_over_rotate_pos() {
        let img: DynamicImage = _setup();
        let cmp: DynamicImage = _setup();

        let operation = Operation::HueRotate(460);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.huerotate(100).raw_pixels(), result_img.raw_pixels());

        _manual_inspection(&result_img, "target/test_hue_rot_pos_460.png")
    }

    #[test]
    fn test_resize_up_gaussian() {
        // 217x447px => 400x500
        let img: DynamicImage = _setup();
        let operation = Operation::Resize(400, 500);

        let (xa, ya) = img.dimensions();

        assert_eq!(ya, 447);
        assert_eq!(xa, 217);

        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xb, 400);
        assert_eq!(yb, 500);

        _manual_inspection(&img_result, "target/test_scale_400x500.png")
    }

    #[test]
    fn test_resize_down_gaussian() {
        // 217x447px => 100x200
        let img: DynamicImage = _setup();
        let operation = Operation::Resize(100, 200);

        let (xa, ya) = img.dimensions();

        assert_eq!(ya, 447);
        assert_eq!(xa, 217);

        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xb, 100);
        assert_eq!(yb, 200);

        _manual_inspection(&img_result, "target/test_scale_100x200.png")
    }

    #[test]
    fn test_rotate90() {
        let img: DynamicImage = _setup();
        let operation = Operation::Rotate90;

        let (xa, ya) = img.dimensions();
        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, yb);
        assert_eq!(xb, ya);

        _manual_inspection(&img_result, "target/test_rotate90.png")
    }

    #[test]
    fn test_rotate180() {
        let img: DynamicImage = _setup();
        let operation = Operation::Rotate180;

        let (xa, ya) = img.dimensions();
        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, xb);
        assert_eq!(ya, yb);

        _manual_inspection(&img_result, "target/test_rotate180.png")
    }

    #[test]
    fn test_rotate270() {
        let img: DynamicImage = _setup();
        let operation = Operation::Rotate270;

        let (xa, ya) = img.dimensions();
        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, yb);
        assert_eq!(xb, ya);

        _manual_inspection(&img_result, "target/test_rotate270.png")
    }

    #[test]
    fn test_multi() {
        // 217x447px original
        let img: DynamicImage = _setup();
        let operations = vec![
            Operation::Resize(80, 100),
            Operation::Blur(5),
            Operation::FlipHorizontal,
            Operation::FlipVertical,
            Operation::Rotate90,
        ];

        let (xa, ya) = img.dimensions();

        assert_eq!(ya, 447);
        assert_eq!(xa, 217);

        let done = apply_operations_on_image(img, &operations);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        // dim original => 80x100 => 100x80
        assert_eq!(xb, 100);
        assert_eq!(yb, 80);

        _manual_inspection(&img_result, "target/test_multi.png")
    }

}
