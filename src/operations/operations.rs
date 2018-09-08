use image::{DynamicImage, FilterType};

use super::Operation;

pub trait ApplyOperation<O, T, E> {
    fn apply_operation(&self, operation: &O) -> Result<T, E>;
}

impl ApplyOperation<Operation, DynamicImage, String> for DynamicImage {
    fn apply_operation(&self, operation: &Operation) -> Result<DynamicImage, String> {
        match *operation {
            Operation::Blur(sigma) => Ok(self.blur(sigma)),
            Operation::Brighten(amount) => Ok(self.brighten(amount)),
            Operation::Contrast(c) => Ok(self.adjust_contrast(c)),
            // We need to ensure here that Filter3x3's `it` (&[f32]) has length 9.
            // Otherwise it will panic, see: https://docs.rs/image/0.19.0/src/image/dynimage.rs.html#349
            // This check happens already within the `parse` module.
            Operation::Filter3x3(ref it) => Ok(self.filter3x3(&it)),
            Operation::FlipHorizontal => Ok(self.fliph()),
            Operation::FlipVertical => Ok(self.flipv()),
            Operation::GrayScale => Ok(self.grayscale()),
            Operation::HueRotate(degree) => Ok(self.huerotate(degree)),
            Operation::Invert => {
                let ref mut img = self.clone();
                image::DynamicImage::invert(img);
                let res = img.clone();
                Ok(res)
            }
            Operation::Resize(new_x, new_y) => {
                Ok(self.resize_exact(new_x, new_y, FilterType::Gaussian))
            }
            Operation::Rotate90 => Ok(self.rotate90()),
            Operation::Rotate270 => Ok(self.rotate270()),
            Operation::Rotate180 => Ok(self.rotate180()),
            Operation::Unsharpen(sigma, threshold) => Ok(self.unsharpen(sigma, threshold)),
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
    use arrayvec::ArrayVec;
    use image::GenericImage;
    use operations::test_setup::*;

    #[test]
    fn test_blur() {
        let img: DynamicImage = setup_default_test_image();
        let operation = Operation::Blur(25.0);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        output_test_image_for_manual_inspection(&done.unwrap(), "target/test_blur.png")
    }

    #[test]
    fn test_brighten_pos() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::Brighten(25);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_brighten_pos_25.png")
    }

    #[test]
    fn test_brighten_zero() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();
        let operation = Operation::Brighten(0);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_eq!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_brighten_zero.png")
    }

    #[test]
    fn test_brighten_neg() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::Brighten(-25);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_brighten_neg_25.png")
    }

    #[test]
    fn test_contrast_pos() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::Contrast(150.9);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_contrast_pos_15_9.png")
    }

    #[test]
    fn test_contrast_neg() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::Contrast(-150.9);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_contrast_pos_15_9.png")
    }

    #[test]
    fn test_filter3x3() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::Filter3x3(ArrayVec::from([
            1.0, 0.5, 0.0, 1.0, 0.5, 0.0, 1.0, 0.5, 0.0,
        ]));

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_filter3x3.png")
    }

    #[test]
    fn test_flip_h() {
        let img: DynamicImage = setup_default_test_image();
        let operation = Operation::FlipHorizontal;

        let (xa, ya) = img.dimensions();
        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, xb);
        assert_eq!(ya, yb);

        output_test_image_for_manual_inspection(&img_result, "target/test_fliph.png")
    }

    #[test]
    fn test_flip_v() {
        let img: DynamicImage = setup_default_test_image();
        let operation = Operation::FlipVertical;

        let (xa, ya) = img.dimensions();
        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, xb);
        assert_eq!(ya, yb);

        output_test_image_for_manual_inspection(&img_result, "target/test_flipv.png")
    }

    #[test]
    fn test_gray_scale() {
        use image::Pixel;

        let img: DynamicImage = setup_test_image("resources/rainbow_8x6.bmp");
        let operation = Operation::GrayScale;

        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();

        // The color type isn't actually changed to luma, so instead of checking color type,
        // here pixels are checked to have equal (r, g, b) components.
        for i in 0..8 {
            for j in 0..6 {
                let pixel = img_result.get_pixel(i, j);
                let channels_result = pixel.channels();
                let r_component = channels_result[0];
                let g_component = channels_result[1];
                let b_component = channels_result[2];

                assert_eq!(r_component, g_component);
                assert_eq!(g_component, b_component);
            }
        }

        output_test_image_for_manual_inspection(&img_result, "target/test_gray_scale.png")
    }

    #[test]
    fn test_hue_rotate_neg() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::HueRotate(-100);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_hue_rot_neg_100.png")
    }

    #[test]
    fn test_hue_rotate_pos() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::HueRotate(100);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_hue_rot_pos_100.png")
    }

    #[test]
    fn test_hue_rotate_zero() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::HueRotate(0);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_eq!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_hue_rot_0.png")
    }

    #[test]
    fn test_hue_rotate_360() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::HueRotate(360);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        // https://docs.rs/image/0.19.0/image/enum.DynamicImage.html#method.huerotate
        // huerotate(0) should be huerotate(360), but this doesn't seem the case
        assert_eq!(cmp.huerotate(360).raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_hue_rot_pos_360.png")
    }

    #[test]
    fn test_hue_rotate_over_rotate_pos() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::HueRotate(460);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.huerotate(100).raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_hue_rot_pos_460.png")
    }

    #[test]
    fn test_invert() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::Invert;

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_invert.png")
    }

    #[test]
    fn test_resize_down_gaussian() {
        // 217x447px => 100x200
        let img: DynamicImage = setup_default_test_image();
        let operation = Operation::Resize(100, 200);

        let (xa, ya) = img.dimensions();

        assert_eq!(xa, 217);
        assert_eq!(ya, 447);

        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xb, 100);
        assert_eq!(yb, 200);

        output_test_image_for_manual_inspection(&img_result, "target/test_scale_100x200.png")
    }

    #[test]
    fn test_resize_up_gaussian() {
        // 217x447px => 300x500
        let img: DynamicImage = setup_default_test_image();
        let operation = Operation::Resize(300, 500);

        let (xa, ya) = img.dimensions();

        assert_eq!(xa, 217);
        assert_eq!(ya, 447);

        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xb, 300);
        assert_eq!(yb, 500);

        output_test_image_for_manual_inspection(&img_result, "target/test_scale_400x500.png")
    }

    #[test]
    fn test_rotate90() {
        let img: DynamicImage = setup_default_test_image();
        let operation = Operation::Rotate90;

        let (xa, ya) = img.dimensions();
        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, yb);
        assert_eq!(xb, ya);

        output_test_image_for_manual_inspection(&img_result, "target/test_rotate90.png")
    }

    #[test]
    fn test_rotate180() {
        let img: DynamicImage = setup_default_test_image();
        let operation = Operation::Rotate180;

        let (xa, ya) = img.dimensions();
        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, xb);
        assert_eq!(ya, yb);

        output_test_image_for_manual_inspection(&img_result, "target/test_rotate180.png")
    }

    #[test]
    fn test_rotate270() {
        let img: DynamicImage = setup_default_test_image();
        let operation = Operation::Rotate270;

        let (xa, ya) = img.dimensions();
        let done = img.apply_operation(&operation);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, yb);
        assert_eq!(xb, ya);

        output_test_image_for_manual_inspection(&img_result, "target/test_rotate270.png")
    }

    #[test]
    fn test_unsharpen_pos() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::Unsharpen(20.1, 20);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_unsharpen_20_1_20.png")
    }

    #[test]
    fn test_unsharpen_neg() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = Operation::Unsharpen(-20.1, -20);

        let done = img.apply_operation(&operation);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, "target/test_unsharpen_neg20_1_neg20.png")
    }

    #[test]
    fn test_multi() {
        // 217x447px original
        let img: DynamicImage = setup_default_test_image();
        let operations = vec![
            Operation::Resize(80, 100),
            Operation::Blur(5.0),
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

        output_test_image_for_manual_inspection(&img_result, "target/test_multi.png")
    }

}
