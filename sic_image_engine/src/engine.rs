/// This version of the operations module will use an AST like structure.
/// Instead of evaluating a program, we apply 'a language' on an image.
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

use sic_core::image::DynamicImage;
use sic_core::image::FilterType;
use sic_core::image::GenericImageView;

use crate::wrapper::filter_type::FilterTypeWrap;
use crate::ImgOp;

trait EnvironmentKey {
    fn key(&self) -> EnvironmentKind;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumDiscriminants)]
#[strum_discriminants(name(EnvironmentKind), derive(Display, Hash))]
pub enum EnvironmentItem {
    CustomSamplingFilter(FilterTypeWrap),
    PreserveAspectRatio,
}

impl EnvironmentItem {
    pub fn resize_sampling_filter(self) -> Option<FilterTypeWrap> {
        match self {
            EnvironmentItem::CustomSamplingFilter(k) => Some(k),
            _ => None,
        }
    }
}

impl EnvironmentKey for EnvironmentItem {
    fn key(&self) -> EnvironmentKind {
        match self {
            EnvironmentItem::CustomSamplingFilter(_) => EnvironmentKind::CustomSamplingFilter,
            EnvironmentItem::PreserveAspectRatio => EnvironmentKind::PreserveAspectRatio,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    store: HashMap<EnvironmentKind, EnvironmentItem>,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Environment {
    pub fn insert_or_update(&mut self, item: EnvironmentItem) {
        let key = item.key();

        *self.store.entry(key).or_insert(item) = item;
    }

    pub fn remove(&mut self, key: EnvironmentKind) -> Option<()> {
        self.store.remove(&key).map(|_| ())
    }

    pub fn get(&mut self, key: EnvironmentKind) -> Option<&EnvironmentItem> {
        self.store.get(&key)
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Operation(ImgOp),
    AddToEnv(EnvironmentItem),
    RemoveFromEnv(EnvironmentKind),
}

#[derive(Clone)]
pub struct ImageEngine {
    environment: Box<Environment>,
    image: Box<DynamicImage>,
}

impl ImageEngine {
    pub fn new(image: DynamicImage) -> Self {
        Self {
            environment: Box::from(Environment::default()),
            image: Box::from(image),
        }
    }

    pub fn ignite(&mut self, statements: &[Instruction]) -> Result<&DynamicImage, Box<dyn Error>> {
        for stmt in statements {
            match self.process_statement(stmt) {
                Ok(_) => continue,
                Err(err) => return Err(err),
            }
        }

        Ok(&self.image)
    }

    pub fn process_statement(&mut self, statement: &Instruction) -> Result<(), Box<dyn Error>> {
        match statement {
            Instruction::Operation(op) => self.process_operation(op),
            Instruction::AddToEnv(item) => self.process_register_env(*item),
            Instruction::RemoveFromEnv(key) => self.process_deregister_env(*key),
        }
    }

    pub fn process_operation(&mut self, operation: &ImgOp) -> Result<(), Box<dyn Error>> {
        match operation {
            ImgOp::Blur(sigma) => {
                *self.image = self.image.blur(*sigma);
                Ok(())
            }
            ImgOp::Brighten(amount) => {
                *self.image = self.image.brighten(*amount);
                Ok(())
            }
            ImgOp::Contrast(c) => {
                *self.image = self.image.adjust_contrast(*c);
                Ok(())
            }
            ImgOp::Crop((lx, ly, rx, ry)) => {
                // 1. verify that the top left anchor is smaller than the bottom right anchor
                // 2. verify that the selection is within the bounds of the image
                Verify::crop_selection_box_can_exist(*lx, *ly, *rx, *ry)
                    .and_then(|_| {
                        Verify::crop_selection_within_image_bounds(&self.image, *lx, *ly, *rx, *ry)
                    })
                    .map(|_| {
                        *self.image = self.image.crop(*lx, *ly, rx - lx, ry - ly);
                    })
            }
            // We need to ensure here that Filter3x3's `it` (&[f32]) has length 9.
            // Otherwise it will panic, see: https://docs.rs/image/0.19.0/src/image/dynimage.rs.html#349
            // This check already happens within the `parse` module.
            ImgOp::Filter3x3(ref it) => {
                *self.image = self.image.filter3x3(it);
                Ok(())
            }
            ImgOp::FlipHorizontal => {
                *self.image = self.image.fliph();
                Ok(())
            }
            ImgOp::FlipVertical => {
                *self.image = self.image.flipv();
                Ok(())
            }
            ImgOp::GrayScale => {
                *self.image = self.image.grayscale();
                Ok(())
            }
            ImgOp::HueRotate(degree) => {
                *self.image = self.image.huerotate(*degree);
                Ok(())
            }
            ImgOp::Invert => {
                self.image.invert();
                Ok(())
            }
            ImgOp::Resize((new_x, new_y)) => {
                const DEFAULT_RESIZE_FILTER: FilterType = FilterType::Gaussian;

                let filter = self
                    .environment
                    .get(EnvironmentKind::CustomSamplingFilter)
                    .and_then(|item| item.resize_sampling_filter())
                    .map(FilterType::from)
                    .unwrap_or(DEFAULT_RESIZE_FILTER);

                *self.image = if self
                    .environment
                    .get(EnvironmentKind::PreserveAspectRatio)
                    .is_some()
                {
                    self.image.resize(*new_x, *new_y, filter)
                } else {
                    self.image.resize_exact(*new_x, *new_y, filter)
                };

                Ok(())
            }
            ImgOp::Rotate90 => {
                *self.image = self.image.rotate90();
                Ok(())
            }
            ImgOp::Rotate180 => {
                *self.image = self.image.rotate180();
                Ok(())
            }
            ImgOp::Rotate270 => {
                *self.image = self.image.rotate270();
                Ok(())
            }
            ImgOp::Unsharpen((sigma, threshold)) => {
                *self.image = self.image.unsharpen(*sigma, *threshold);
                Ok(())
            }
        }
    }

    pub fn process_register_env(&mut self, item: EnvironmentItem) -> Result<(), Box<dyn Error>> {
        self.environment.insert_or_update(item);

        Ok(())
    }

    pub fn process_deregister_env(&mut self, key: EnvironmentKind) -> Result<(), Box<dyn Error>> {
        let success = self.environment.remove(key);

        if success.is_none() {
            eprintln!(
                "Warning: tried to de-register: {:?}, but wasn't registered.",
                key
            );
        }

        Ok(())
    }
}

struct Verify;

impl Verify {
    fn crop_selection_box_can_exist(
        lx: u32,
        ly: u32,
        rx: u32,
        ry: u32,
    ) -> Result<(), Box<dyn Error>> {
        if (rx <= lx) || (ry <= ly) {
            Err(format!(
                "Operation: crop -- Top selection coordinates are smaller than bottom selection coordinates. \
            Required top selection < bottom selection but given coordinates are: [top anchor: (x={}, y={}), bottom anchor: (x={}, y={})].",
                lx, ly, rx, ry
            ).into())
        } else {
            Ok(())
        }
    }

    fn crop_selection_within_image_bounds(
        image: &DynamicImage,
        lx: u32,
        ly: u32,
        rx: u32,
        ry: u32,
    ) -> Result<(), Box<dyn Error>> {
        let (dim_x, dim_y) = image.dimensions();

        match (lx <= dim_x, ly <= dim_y, rx <= dim_x, ry <= dim_y) {
            (true, true, true, true) => Ok(()),
            _ => {
                println!("error expected");
                Err(format!("Operation: crop -- Top or bottom selection coordinates out of bounds: selection is [top anchor: \
                (x={}, y={}), bottom anchor: (x={}, y={})] but max selection range is: (x={}, y={}).", lx, ly, rx, ry, dim_x, dim_y).into())
            }
        }
    }
}

#[cfg(test)]
mod environment_tests {
    use sic_core::image::FilterType;

    use super::*;

    #[test]
    fn environment_insert() {
        let mut env = Environment::default();
        assert!(!env
            .store
            .contains_key(&EnvironmentKind::CustomSamplingFilter));

        env.insert_or_update(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
            FilterType::Triangle,
        )));

        assert!(env
            .store
            .contains_key(&EnvironmentKind::CustomSamplingFilter));
    }

    #[test]
    fn environment_update() {
        let mut env = Environment::default();

        env.insert_or_update(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
            FilterType::Triangle,
        )));

        assert!(env
            .store
            .contains_key(&EnvironmentKind::CustomSamplingFilter));
        assert_eq!(
            EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(FilterType::Triangle)),
            *env.get(EnvironmentKind::CustomSamplingFilter).unwrap()
        );

        env.insert_or_update(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
            FilterType::Gaussian,
        )));

        assert!(env
            .store
            .contains_key(&EnvironmentKind::CustomSamplingFilter));
        assert_eq!(
            EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(FilterType::Gaussian)),
            *env.get(EnvironmentKind::CustomSamplingFilter).unwrap()
        );
    }

    #[test]
    fn environment_remove() {
        let mut env = Environment::default();

        env.insert_or_update(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
            FilterType::Triangle,
        )));

        assert!(env
            .store
            .contains_key(&EnvironmentKind::CustomSamplingFilter));
        assert_eq!(
            EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(FilterType::Triangle)),
            *env.get(EnvironmentKind::CustomSamplingFilter).unwrap()
        );

        let removed = env.remove(EnvironmentKind::CustomSamplingFilter);

        assert!(removed.is_some());
        assert!(!env
            .store
            .contains_key(&EnvironmentKind::CustomSamplingFilter));
    }

    #[test]
    fn environment_remove_not_existing() {
        let mut env = Environment::default();

        assert!(!env
            .store
            .contains_key(&EnvironmentKind::CustomSamplingFilter));

        let removed = env.remove(EnvironmentKind::CustomSamplingFilter);

        assert!(removed.is_none());
        assert!(!env
            .store
            .contains_key(&EnvironmentKind::CustomSamplingFilter));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use sic_core::image::DynamicImage;
    use sic_core::image::FilterType;
    use sic_core::image::GenericImageView;
    use sic_core::image::Rgba;

    use sic_testing::{in_, out_};

    // output images during tests to verify the results visually
    fn output_test_image_for_manual_inspection(img: &DynamicImage, path: &str) {
        if cfg!(feature = "output-test-images") {
            let _ = img.save(path);
        }
    }

    fn setup_default_test_image() -> DynamicImage {
        const DEFAULT_TEST_IMAGE_PATH: &str = "unsplash_763569_cropped.jpg";
        sic_testing::open_test_image(sic_testing::in_!(DEFAULT_TEST_IMAGE_PATH))
    }

    #[test]
    fn resize_with_preserve_aspect_ratio() {
        // W 217 H 447
        let img: DynamicImage = setup_default_test_image();

        let mut engine = ImageEngine::new(img);
        let mut engine2 = engine.clone();
        let cmp_left = engine.ignite(&vec![
            Instruction::AddToEnv(EnvironmentItem::PreserveAspectRatio),
            Instruction::Operation(ImgOp::Resize((100, 100))),
        ]);

        assert!(cmp_left.is_ok());

        let cmp_right = engine2.ignite(&vec![Instruction::Operation(ImgOp::Resize((100, 100)))]);

        assert!(cmp_left.is_ok());

        let left = cmp_left.unwrap();
        let right = cmp_right.unwrap();

        assert_ne!(left.raw_pixels(), right.raw_pixels());

        // 447 > 217 ->  (u32) |_ 217px/447px*100px _| = 48px
        assert_eq!((48, 100), left.dimensions());

        output_test_image_for_manual_inspection(
            &left,
            out_!("test_resize_preserve_aspect_ratio_left_preserve.png"),
        );

        output_test_image_for_manual_inspection(
            &right,
            out_!("test_resize_preserve_aspect_ratio_right_default.png"),
        );
    }

    #[test]
    fn resize_with_sampling_filter_nearest() {
        let img: DynamicImage = setup_default_test_image();

        let mut engine = ImageEngine::new(img);
        let mut engine2 = engine.clone();
        let cmp_left = engine.ignite(&vec![
            Instruction::AddToEnv(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                FilterType::Nearest,
            ))),
            Instruction::Operation(ImgOp::Resize((100, 100))),
        ]);

        assert!(cmp_left.is_ok());

        let cmp_right = engine2.ignite(&vec![Instruction::Operation(ImgOp::Resize((100, 100)))]);

        assert!(cmp_left.is_ok());

        let left = cmp_left.unwrap();
        let right = cmp_right.unwrap();

        assert_ne!(left.raw_pixels(), right.raw_pixels());

        output_test_image_for_manual_inspection(
            &left,
            out_!("test_resize_sampling_filter_left_nearest.png"),
        );

        output_test_image_for_manual_inspection(
            &right,
            out_!("test_resize_sampling_filter_right_default_gaussian.png"),
        );
    }

    #[test]
    fn register_unregister_sampling_filter() {
        let img: DynamicImage = setup_default_test_image();

        let mut engine = ImageEngine::new(img);
        let mut engine2 = engine.clone();

        let cmp_left = engine.ignite(&vec![
            Instruction::AddToEnv(EnvironmentItem::CustomSamplingFilter(FilterTypeWrap::new(
                FilterType::Nearest,
            ))),
            Instruction::RemoveFromEnv(EnvironmentKind::CustomSamplingFilter),
            Instruction::Operation(ImgOp::Resize((100, 100))),
        ]);

        assert!(cmp_left.is_ok());

        let cmp_right = engine2.ignite(&vec![Instruction::Operation(ImgOp::Resize((100, 100)))]);

        assert!(cmp_left.is_ok());

        let left = cmp_left.unwrap();
        let right = cmp_right.unwrap();

        assert_eq!(left.raw_pixels(), right.raw_pixels());

        output_test_image_for_manual_inspection(
            &left,
            out_!("test_register_unregister_sampling_filter_left.png"),
        );

        output_test_image_for_manual_inspection(
            &right,
            out_!("test_register_unregister_sampling_filter_right.png"),
        );
    }

    #[test]
    fn test_blur() {
        let img: DynamicImage = setup_default_test_image();
        let operation = ImgOp::Blur(10.0);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        output_test_image_for_manual_inspection(&done.unwrap(), out_!("test_blur.png"));
    }

    #[test]
    fn test_brighten_pos() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::Brighten(25);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_brighten_pos_25.png"));
    }

    #[test]
    fn test_brighten_zero() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();
        let operation = ImgOp::Brighten(0);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_eq!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_brighten_zero.png"));
    }

    #[test]
    fn test_brighten_neg() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::Brighten(-25);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_brighten_neg_25.png"));
    }

    #[test]
    fn test_contrast_pos() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::Contrast(150.9);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_contrast_pos_15_9.png"));
    }

    #[test]
    fn test_contrast_neg() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::Contrast(-150.9);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_contrast_pos_15_9.png"));
    }

    #[test]
    fn test_crop_ok_no_change() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));
        let cmp: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        let operation = ImgOp::Crop((0, 0, 2, 2));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_eq!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_crop_no_change.bmp"));
    }

    #[test]
    fn test_crop_ok_to_one_pixel() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));
        let cmp: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        let operation = ImgOp::Crop((0, 0, 1, 1));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        let result_dim = result_img.dimensions();
        assert_eq!(1, result_dim.0);
        assert_eq!(1, result_dim.1);

        assert_eq!(Rgba([0, 0, 0, 255]), result_img.get_pixel(0, 0));

        output_test_image_for_manual_inspection(
            &result_img,
            out_!("test_crop_ok_to_one_pixel.bmp"),
        );
    }

    #[test]
    fn test_crop_ok_to_half_horizontal() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));
        let cmp: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        let operation = ImgOp::Crop((0, 0, 2, 1));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        let result_dim = result_img.dimensions();
        assert_eq!(2, result_dim.0);
        assert_eq!(1, result_dim.1);

        assert_eq!(Rgba([0, 0, 0, 255]), result_img.get_pixel(0, 0));
        assert_eq!(Rgba([255, 255, 255, 255]), result_img.get_pixel(1, 0));

        output_test_image_for_manual_inspection(
            &result_img,
            out_!("test_crop_ok_to_half_horizontal.bmp"),
        );
    }

    #[test]
    fn test_crop_err_lx_larger_than_rx() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        // not rx >= lx
        let operation = ImgOp::Crop((1, 0, 0, 0));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_crop_err_ly_larger_than_ry() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        // not rx >= lx
        let operation = ImgOp::Crop((0, 1, 0, 0));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_crop_err_out_of_image_bounds_top_lx() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        let operation = ImgOp::Crop((3, 0, 1, 1));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_crop_err_out_of_image_bounds_top_ly() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        let operation = ImgOp::Crop((0, 3, 1, 1));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_crop_err_out_of_image_bounds_top_rx() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        let operation = ImgOp::Crop((0, 0, 3, 1));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_crop_err_out_of_image_bounds_top_ry() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        let operation = ImgOp::Crop((0, 0, 1, 3));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_filter3x3() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::Filter3x3([1.0, 0.5, 0.0, 1.0, 0.5, 0.0, 1.0, 0.5, 0.0]);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_filter3x3.png"))
    }

    #[test]
    fn test_flip_h() {
        let img: DynamicImage = setup_default_test_image();
        let operation = ImgOp::FlipHorizontal;

        let (xa, ya) = img.dimensions();
        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, xb);
        assert_eq!(ya, yb);

        output_test_image_for_manual_inspection(&img_result, out_!("test_fliph.png"));
    }

    #[test]
    fn test_flip_v() {
        let img: DynamicImage = setup_default_test_image();
        let operation = ImgOp::FlipVertical;

        let (xa, ya) = img.dimensions();
        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, xb);
        assert_eq!(ya, yb);

        output_test_image_for_manual_inspection(&img_result, out_!("test_flipv.png"));
    }

    #[test]
    fn test_gray_scale() {
        use sic_core::image::Pixel;

        let img: DynamicImage = sic_testing::open_test_image(in_!("rainbow_8x6.bmp"));
        let operation = ImgOp::GrayScale;

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

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

        output_test_image_for_manual_inspection(&img_result, out_!("test_gray_scale.png"));
    }

    #[test]
    fn test_hue_rotate_neg() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::HueRotate(-100);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_hue_rot_neg_100.png"));
    }

    #[test]
    fn test_hue_rotate_pos() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::HueRotate(100);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_hue_rot_pos_100.png"));
    }

    #[test]
    fn test_hue_rotate_zero() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::HueRotate(0);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_eq!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_hue_rot_0.png"));
    }

    #[test]
    fn test_hue_rotate_360() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::HueRotate(360);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        // https://docs.rs/image/0.19.0/image/enum.DynamicImage.html#method.huerotate
        // huerotate(0) should be huerotate(360), but this doesn't seem the case
        assert_eq!(cmp.huerotate(360).raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_hue_rot_pos_360.png"));
    }

    #[test]
    fn test_hue_rotate_over_rotate_pos() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::HueRotate(460);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.huerotate(100).raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_hue_rot_pos_460.png"));
    }

    #[test]
    fn test_invert() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::Invert;

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_invert.png"));
    }

    #[test]
    fn test_resize_down_gaussian() {
        // 217x447px => 100x200
        let img: DynamicImage = setup_default_test_image();
        let operation = ImgOp::Resize((100, 200));

        let (xa, ya) = img.dimensions();

        assert_eq!(xa, 217);
        assert_eq!(ya, 447);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xb, 100);
        assert_eq!(yb, 200);

        output_test_image_for_manual_inspection(&img_result, out_!("test_scale_100x200.png"));
    }

    #[test]
    fn test_resize_up_gaussian() {
        // 217x447px => 300x500
        let img: DynamicImage = setup_default_test_image();
        let operation = ImgOp::Resize((250, 500));

        let (xa, ya) = img.dimensions();

        assert_eq!(xa, 217);
        assert_eq!(ya, 447);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xb, 250);
        assert_eq!(yb, 500);

        output_test_image_for_manual_inspection(&img_result, out_!("test_scale_250x500.png"));
    }

    #[test]
    fn test_rotate90() {
        let img: DynamicImage = setup_default_test_image();
        let operation = ImgOp::Rotate90;

        let (xa, ya) = img.dimensions();
        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, yb);
        assert_eq!(xb, ya);

        output_test_image_for_manual_inspection(&img_result, out_!("test_rotate90.png"));
    }

    #[test]
    fn test_rotate180() {
        let img: DynamicImage = setup_default_test_image();
        let operation = ImgOp::Rotate180;

        let (xa, ya) = img.dimensions();
        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, xb);
        assert_eq!(ya, yb);

        output_test_image_for_manual_inspection(&img_result, out_!("test_rotate180.png"));
    }

    #[test]
    fn test_rotate270() {
        let img: DynamicImage = setup_default_test_image();
        let operation = ImgOp::Rotate270;

        let (xa, ya) = img.dimensions();
        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);

        assert!(done.is_ok());

        let img_result = done.unwrap();
        let (xb, yb) = img_result.dimensions();

        assert_eq!(xa, yb);
        assert_eq!(xb, ya);

        output_test_image_for_manual_inspection(&img_result, out_!("test_rotate270.png"));
    }

    #[test]
    fn test_unsharpen_pos() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::Unsharpen((20.1, 20));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_unsharpen_20_1_20.png"));
    }

    #[test]
    fn test_unsharpen_neg() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::Unsharpen((-20.1, -20));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&vec![Instruction::Operation(operation)]);
        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(
            &result_img,
            out_!("test_unsharpen_neg20_1_neg20.png"),
        );
    }

    #[test]
    fn test_multi() {
        // 217x447px original
        let img: DynamicImage = setup_default_test_image();
        let operations = vec![
            Instruction::Operation(ImgOp::Resize((80, 100))),
            Instruction::Operation(ImgOp::Blur(5.0)),
            Instruction::Operation(ImgOp::FlipHorizontal),
            Instruction::Operation(ImgOp::FlipVertical),
            Instruction::Operation(ImgOp::Rotate90),
        ];
        let (xa, ya) = img.dimensions();

        assert_eq!(ya, 447);
        assert_eq!(xa, 217);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&operations);

        assert!(done.is_ok());

        let done_image = done.unwrap();
        let (xb, yb) = done_image.dimensions();

        // dim original => 80x100 => 100x80
        assert_eq!(xb, 100);
        assert_eq!(yb, 80);

        output_test_image_for_manual_inspection(&done_image, out_!("test_multi.png"));
    }
}
