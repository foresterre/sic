use std::cmp::max;
use std::collections::HashMap;
use std::hash::Hash;

use sic_core::image::imageops::{self, FilterType};
use sic_core::image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

use crate::errors::SicImageEngineError;
use crate::wrapper::filter_type::FilterTypeWrap;
use crate::ImgOp;

trait EnvironmentKey {
    fn key(&self) -> ItemName;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumDiscriminants)]
#[strum_discriminants(name(ItemName), derive(Display, Hash))]
pub enum EnvItem {
    CustomSamplingFilter(FilterTypeWrap),
    PreserveAspectRatio(bool),
}

impl EnvItem {
    pub fn resize_sampling_filter(self) -> Option<FilterTypeWrap> {
        match self {
            EnvItem::CustomSamplingFilter(k) => Some(k),
            _ => None,
        }
    }
}

impl EnvironmentKey for EnvItem {
    fn key(&self) -> ItemName {
        match self {
            EnvItem::CustomSamplingFilter(_) => ItemName::CustomSamplingFilter,
            EnvItem::PreserveAspectRatio(_) => ItemName::PreserveAspectRatio,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Env {
    store: HashMap<ItemName, EnvItem>,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Env {
    pub fn insert_or_update(&mut self, item: EnvItem) {
        let key = item.key();

        *self.store.entry(key).or_insert(item) = item;
    }

    pub fn remove(&mut self, key: ItemName) -> Option<()> {
        self.store.remove(&key).map(|_| ())
    }

    pub fn get(&mut self, key: ItemName) -> Option<&EnvItem> {
        self.store.get(&key)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instr {
    Operation(ImgOp),
    EnvAdd(EnvItem),
    EnvRemove(ItemName),
}

#[derive(Clone)]
pub struct ImageEngine {
    environment: Box<Env>,
    image: Box<DynamicImage>,
}

impl ImageEngine {
    pub fn new(image: DynamicImage) -> Self {
        Self {
            environment: Box::from(Env::default()),
            image: Box::from(image),
        }
    }

    pub fn ignite(&mut self, instructions: &[Instr]) -> Result<&DynamicImage, SicImageEngineError> {
        for instruction in instructions {
            match self.process_instruction(instruction) {
                Ok(_) => continue,
                Err(err) => return Err(err),
            }
        }

        Ok(&self.image)
    }

    fn process_instruction(&mut self, instruction: &Instr) -> Result<(), SicImageEngineError> {
        match instruction {
            Instr::Operation(op) => self.process_operation(op),
            Instr::EnvAdd(item) => {
                self.insert_env(*item);
                Ok(())
            }
            Instr::EnvRemove(key) => {
                self.remove_env(*key);
                Ok(())
            }
        }
    }

    fn process_operation(&mut self, operation: &ImgOp) -> Result<(), SicImageEngineError> {
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
                let selection = CropSelection::new(*lx, *ly, *rx, *ry);

                // 1. verify that the top left anchor is smaller than the bottom right anchor
                // 2. verify that the selection is within the bounds of the image
                selection
                    .dimensions_are_ok()
                    .and_then(|selection| selection.fits_within(&self.image))
                    .map(|_| {
                        *self.image = self.image.crop(*lx, *ly, rx - lx, ry - ly);
                    })
            }
            ImgOp::Diff(img) => {
                let other = img.open_image()?;
                *self.image = produce_image_diff(&self.image, &other);

                Ok(())
            }

            #[cfg(feature = "imageproc-ops")]
            ImgOp::DrawText(inner) => {
                let text = inner.text();
                let coords = inner.coords();
                let font_options = inner.font_options();

                let font_file = std::fs::read(&font_options.font_path)
                    .map_err(SicImageEngineError::FontFileLoadError)?;

                let font = rusttype::Font::try_from_bytes(&font_file)
                    .ok_or(SicImageEngineError::FontError)?;

                *self.image = DynamicImage::ImageRgba8(imageproc::drawing::draw_text(
                    &mut *self.image,
                    font_options.color,
                    coords.0,
                    coords.1,
                    font_options.scale,
                    &font,
                    text,
                ));

                Ok(())
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
            ImgOp::Overlay(overlay) => {
                let overlay_image = overlay.image_path().open_image()?;
                let pos = overlay.position();
                imageops::overlay(&mut *self.image, &overlay_image, pos.0, pos.1);

                Ok(())
            }
            ImgOp::Resize((new_x, new_y)) => {
                let filter = resize_filter_or_default(&mut self.environment);

                if let Some(reg) = self.environment.get(ItemName::PreserveAspectRatio) {
                    if let EnvItem::PreserveAspectRatio(preserve) = reg {
                        if *preserve {
                            *self.image = self.image.resize(*new_x, *new_y, filter);
                        } else {
                            *self.image = self.image.resize_exact(*new_x, *new_y, filter);
                        }
                    }
                } else {
                    // default if preserve-aspect-ratio option has not been set
                    *self.image = self.image.resize_exact(*new_x, *new_y, filter);
                }

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

    fn insert_env(&mut self, item: EnvItem) {
        self.environment.insert_or_update(item);
    }

    fn remove_env(&mut self, key: ItemName) {
        let success = self.environment.remove(key);

        if success.is_none() {
            eprintln!(
                "Warning: tried to de-register: {:?}, but wasn't registered.",
                key
            );
        }
    }
}

// same -> white pixel
const DIFF_PX_SAME: Rgba<u8> = Rgba([255, 255, 255, 255]);
// different -> coloured pixel
const DIFF_PX_DIFF: Rgba<u8> = Rgba([255, 0, 0, 255]);
// non overlapping -> transparent pixel
const DIFF_PX_NO_OVERLAP: Rgba<u8> = Rgba([0, 0, 0, 0]);

/// Takes the diff of two images.
///
/// If a pixel at `(x, y)` in the image `this` (`P`) compared to the pixel at `(x, y)` in the image `that` (`Q`):
/// * is the same: the output image will colour that pixel white.
/// * differs: the output image will colour that pixel red.
///
/// The output image (`R`) will have width `w=max(width(this), width(that))` and height
/// `h=max(height(this), height(that))`.
///
/// In case that two images when overlapped differ inversely in both width and height, so
/// `(P_width > Q_width ∧ P_height < Q_height) ⊕ (P_width < Q_width ∧ P_height > Q_height)` then
/// there will be pixels in `R`, for which for some pixels `p_{i, j} ∈ R | p_{i, j} ∉ P ∨ p_{i, j} ∉ Q`.
/// That is, the part of output image which isn't part of either of the two original input images.
/// These pixels will be 'coloured' black but with an alpha value of 0, so they will be transparent
/// as to show they were not part of the input images.
fn produce_image_diff(this: &DynamicImage, other: &DynamicImage) -> DynamicImage {
    let (lw, lh) = this.dimensions();
    let (rw, rh) = other.dimensions();

    let w = max(lw, rw);
    let h = max(lh, rh);

    let mut buffer = ImageBuffer::new(w, h);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        if this.in_bounds(x, y) && other.in_bounds(x, y) {
            if this.get_pixel(x, y) == other.get_pixel(x, y) {
                *pixel = DIFF_PX_SAME;
            } else {
                *pixel = DIFF_PX_DIFF;
            }
        } else if this.in_bounds(x, y) || other.in_bounds(x, y) {
            *pixel = DIFF_PX_DIFF;
        } else {
            *pixel = DIFF_PX_NO_OVERLAP;
        }
    }

    DynamicImage::ImageRgba8(buffer)
}

struct CropSelection {
    lx: u32,
    ly: u32,
    rx: u32,
    ry: u32,
}

impl CropSelection {
    pub(crate) fn new(lx: u32, ly: u32, rx: u32, ry: u32) -> Self {
        Self { lx, ly, rx, ry }
    }

    pub(crate) fn dimensions_are_ok(&self) -> Result<&Self, SicImageEngineError> {
        if self.are_dimensions_incorrect() {
            Err(SicImageEngineError::CropInvalidSelection(
                self.lx, self.ly, self.rx, self.ry,
            ))
        } else {
            Ok(self)
        }
    }

    pub(crate) fn fits_within(&self, outer: &DynamicImage) -> Result<&Self, SicImageEngineError> {
        let (dim_x, dim_y) = outer.dimensions();

        match (
            self.lx <= dim_x,
            self.ly <= dim_y,
            self.rx <= dim_x,
            self.ry <= dim_y,
        ) {
            (true, true, true, true) => Ok(self),
            _ => Err(SicImageEngineError::CropCoordinateOutOfBounds(
                dim_x, dim_y, self.lx, self.ly, self.rx, self.ry,
            )),
        }
    }

    fn are_dimensions_incorrect(&self) -> bool {
        (self.rx <= self.lx) || (self.ry <= self.ly)
    }
}

fn resize_filter_or_default(env: &mut Env) -> FilterType {
    env.get(ItemName::CustomSamplingFilter)
        .and_then(|item| item.resize_sampling_filter())
        .map(FilterType::from)
        .unwrap_or_else(|| FilterTypeWrap::default().into())
}

#[cfg(test)]
mod compatibility {

    // The raw_pixels() method was removed from the image crate in version 0.23
    // We replace it for our test cases with this straightforward trait, and trait impl for
    // DynamicImage.
    pub(crate) trait RawPixels {
        fn raw_pixels(&self) -> Vec<u8>;
    }

    impl RawPixels for sic_core::image::DynamicImage {
        fn raw_pixels(&self) -> Vec<u8> {
            match self {
                sic_core::image::DynamicImage::ImageLuma8(buffer) => buffer.to_vec(),
                sic_core::image::DynamicImage::ImageLumaA8(buffer) => buffer.to_vec(),
                sic_core::image::DynamicImage::ImageRgb8(buffer) => buffer.to_vec(),
                sic_core::image::DynamicImage::ImageRgba8(buffer) => buffer.to_vec(),
                sic_core::image::DynamicImage::ImageBgr8(buffer) => buffer.to_vec(),
                sic_core::image::DynamicImage::ImageBgra8(buffer) => buffer.to_vec(),
                _ => unimplemented!(),
            }
        }
    }
}

#[cfg(test)]
mod environment_tests {
    use super::*;

    #[test]
    fn environment_insert() {
        let mut env = Env::default();
        assert!(!env.store.contains_key(&ItemName::CustomSamplingFilter));

        env.insert_or_update(EnvItem::CustomSamplingFilter(FilterTypeWrap::new(
            FilterType::Triangle,
        )));

        assert!(env.store.contains_key(&ItemName::CustomSamplingFilter));
    }

    #[test]
    fn environment_update() {
        let mut env = Env::default();

        env.insert_or_update(EnvItem::CustomSamplingFilter(FilterTypeWrap::new(
            FilterType::Triangle,
        )));

        assert!(env.store.contains_key(&ItemName::CustomSamplingFilter));
        assert_eq!(
            EnvItem::CustomSamplingFilter(FilterTypeWrap::new(FilterType::Triangle)),
            *env.get(ItemName::CustomSamplingFilter).unwrap()
        );

        env.insert_or_update(EnvItem::CustomSamplingFilter(FilterTypeWrap::new(
            FilterType::Gaussian,
        )));

        assert!(env.store.contains_key(&ItemName::CustomSamplingFilter));
        assert_eq!(
            EnvItem::CustomSamplingFilter(FilterTypeWrap::new(FilterType::Gaussian)),
            *env.get(ItemName::CustomSamplingFilter).unwrap()
        );
    }

    #[test]
    fn environment_remove() {
        let mut env = Env::default();

        env.insert_or_update(EnvItem::CustomSamplingFilter(FilterTypeWrap::new(
            FilterType::Triangle,
        )));

        assert!(env.store.contains_key(&ItemName::CustomSamplingFilter));
        assert_eq!(
            EnvItem::CustomSamplingFilter(FilterTypeWrap::new(FilterType::Triangle)),
            *env.get(ItemName::CustomSamplingFilter).unwrap()
        );

        let removed = env.remove(ItemName::CustomSamplingFilter);

        assert!(removed.is_some());
        assert!(!env.store.contains_key(&ItemName::CustomSamplingFilter));
    }

    #[test]
    fn environment_remove_not_existing() {
        let mut env = Env::default();

        assert!(!env.store.contains_key(&ItemName::CustomSamplingFilter));

        let removed = env.remove(ItemName::CustomSamplingFilter);

        assert!(removed.is_none());
        assert!(!env.store.contains_key(&ItemName::CustomSamplingFilter));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::compatibility::*;
    use crate::wrapper::image_path::ImageFromPath;
    use sic_core::image::imageops::FilterType;
    use sic_core::image::GenericImageView;
    use sic_core::image::Rgba;
    use sic_testing::*;
    use std::path::PathBuf;

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
    fn diff_check_out_pixels() {
        const LEFT: &str = "2x3_wrabaa.png";
        let left = sic_testing::open_test_image(sic_testing::in_!(LEFT));
        const RIGHT: &str = "3x2_wbaaba.png";

        let mut engine = ImageEngine::new(left);
        let out = engine.ignite(&[Instr::Operation(ImgOp::Diff(ImageFromPath::new(
            PathBuf::from(in_!(RIGHT)),
        )))]);

        let out = out.unwrap();

        assert_eq!(out.width(), 3);
        assert_eq!(out.height(), 3);

        assert_eq!(out.get_pixel(0, 0), DIFF_PX_SAME);
        assert_eq!(out.get_pixel(1, 0), DIFF_PX_DIFF);
        assert_eq!(out.get_pixel(2, 0), DIFF_PX_DIFF);
        assert_eq!(out.get_pixel(0, 1), DIFF_PX_SAME);
        assert_eq!(out.get_pixel(1, 1), DIFF_PX_SAME);
        assert_eq!(out.get_pixel(2, 1), DIFF_PX_DIFF);
        assert_eq!(out.get_pixel(0, 2), DIFF_PX_DIFF);
        assert_eq!(out.get_pixel(1, 2), DIFF_PX_DIFF);
        assert_eq!(out.get_pixel(2, 2), DIFF_PX_NO_OVERLAP);

        output_test_image_for_manual_inspection(&out, out_!("test_diff_3x3.png"));
    }

    mod sizes {
        use super::*;

        ide!();

        #[pm(
            left = {
                "1x1_a.png",
                "2x2_baab.png",
                "2x3_yrgyyb.bmp",
                "2x2_baab.png",
            },
            right = {
                "1x1_b.png",
                "1x1_b.png",
                "3x2_ygyryb.bmp",
                "2x3_rrgrbb.bmp",
            },
            expected_width = {
                1,
                2,
                3,
                2,
            },
            expected_height = {
                1,
                2,
                3,
                3,
            },
        )]
        fn diff_has_expected_width_and_height(
            left: &str,
            right: &str,
            expected_width: u32,
            expected_height: u32,
        ) {
            let left_img = sic_testing::open_test_image(sic_testing::in_!(left));

            let mut engine = ImageEngine::new(left_img);
            let out = engine.ignite(&[Instr::Operation(ImgOp::Diff(ImageFromPath::new(
                PathBuf::from(in_!(right)),
            )))]);

            let out = out.unwrap();

            assert_eq!(out.width(), expected_width);
            assert_eq!(out.height(), expected_height);

            let name = format!("test_diff_{},{}.png", left, right);
            output_test_image_for_manual_inspection(&out, out_!(&name));
        }
    }

    #[test]
    fn resize_with_preserve_aspect_ratio() {
        // W 217 H 447
        let img: DynamicImage = setup_default_test_image();

        let mut engine = ImageEngine::new(img);
        let mut engine2 = engine.clone();
        let cmp_left = engine.ignite(&[
            Instr::EnvAdd(EnvItem::PreserveAspectRatio(true)),
            Instr::Operation(ImgOp::Resize((100, 100))),
        ]);

        assert!(cmp_left.is_ok());

        let cmp_right = engine2.ignite(&[Instr::Operation(ImgOp::Resize((100, 100)))]);

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
    fn resize_with_preserve_aspect_ratio_set_to_false() {
        // W 217 H 447
        let img: DynamicImage = setup_default_test_image();

        let mut engine = ImageEngine::new(img);
        let mut engine2 = engine.clone();
        let cmp_left = engine.ignite(&[
            Instr::EnvAdd(EnvItem::PreserveAspectRatio(false)),
            Instr::Operation(ImgOp::Resize((100, 100))),
        ]);

        assert!(cmp_left.is_ok());

        let cmp_right = engine2.ignite(&[Instr::Operation(ImgOp::Resize((100, 100)))]);

        assert!(cmp_left.is_ok());

        let left = cmp_left.unwrap();
        let right = cmp_right.unwrap();

        assert_eq!(left.raw_pixels(), right.raw_pixels());

        assert_eq!((100, 100), left.dimensions());

        output_test_image_for_manual_inspection(
            &left,
            out_!("test_resize_preserve_aspect_ratio_left_preserve_f.png"),
        );

        output_test_image_for_manual_inspection(
            &right,
            out_!("test_resize_preserve_aspect_ratio_right_default_f.png"),
        );
    }

    #[test]
    fn resize_with_sampling_filter_nearest() {
        let img: DynamicImage = setup_default_test_image();

        let mut engine = ImageEngine::new(img);
        let mut engine2 = engine.clone();
        let cmp_left = engine.ignite(&[
            Instr::EnvAdd(EnvItem::CustomSamplingFilter(FilterTypeWrap::new(
                FilterType::Nearest,
            ))),
            Instr::Operation(ImgOp::Resize((100, 100))),
        ]);

        assert!(cmp_left.is_ok());

        let cmp_right = engine2.ignite(&[Instr::Operation(ImgOp::Resize((100, 100)))]);

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

        let cmp_left = engine.ignite(&[
            Instr::EnvAdd(EnvItem::CustomSamplingFilter(FilterTypeWrap::new(
                FilterType::Nearest,
            ))),
            Instr::EnvRemove(ItemName::CustomSamplingFilter),
            Instr::Operation(ImgOp::Resize((100, 100))),
        ]);

        assert!(cmp_left.is_ok());

        let cmp_right = engine2.ignite(&[Instr::Operation(ImgOp::Resize((100, 100)))]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

        assert!(done.is_ok());

        output_test_image_for_manual_inspection(&done.unwrap(), out_!("test_blur.png"));
    }

    #[test]
    fn test_brighten_pos() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::Brighten(25);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_crop_err_ly_larger_than_ry() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        // not rx >= lx
        let operation = ImgOp::Crop((0, 1, 0, 0));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&[Instr::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_crop_err_out_of_image_bounds_top_lx() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        let operation = ImgOp::Crop((3, 0, 1, 1));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&[Instr::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_crop_err_out_of_image_bounds_top_ly() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        let operation = ImgOp::Crop((0, 3, 1, 1));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&[Instr::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_crop_err_out_of_image_bounds_top_rx() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        let operation = ImgOp::Crop((0, 0, 3, 1));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&[Instr::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_crop_err_out_of_image_bounds_top_ry() {
        let img: DynamicImage = sic_testing::open_test_image(in_!("blackwhite_2x2.bmp"));

        let operation = ImgOp::Crop((0, 0, 1, 3));

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&[Instr::Operation(operation)]);

        assert!(done.is_err());
    }

    #[test]
    fn test_filter3x3() {
        let img: DynamicImage = setup_default_test_image();
        let cmp: DynamicImage = setup_default_test_image();

        let operation = ImgOp::Filter3x3([1.0, 0.5, 0.0, 1.0, 0.5, 0.0, 1.0, 0.5, 0.0]);

        let mut operator = ImageEngine::new(img);
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

        assert!(done.is_ok());

        let result_img = done.unwrap();

        assert_ne!(cmp.raw_pixels(), result_img.raw_pixels());

        output_test_image_for_manual_inspection(&result_img, out_!("test_invert.png"));
    }

    mod overlay {
        use super::*;
        use crate::wrapper::overlay::OverlayInputs;

        #[test]
        fn overlay_with_self_at_origin() {
            let img = setup_default_test_image();
            let overlay = sic_testing::in_!("unsplash_763569_cropped.jpg");

            let mut engine = ImageEngine::new(img.clone());
            let res = engine.ignite(&[Instr::Operation(ImgOp::Overlay(OverlayInputs::new(
                ImageFromPath::new(overlay.into()),
                (0, 0),
            )))]);

            let res_image = res.unwrap();
            assert_eq!(img.raw_pixels(), res_image.raw_pixels());

            output_test_image_for_manual_inspection(
                &res_image,
                out_!("test_overlay_self_origin.png"),
            );
        }

        #[test]
        fn overlay_with_self_outside_bounds() {
            let img = setup_default_test_image();
            let bounds = img.dimensions();

            let overlay = sic_testing::in_!("unsplash_763569_cropped.jpg");

            let mut engine = ImageEngine::new(img.clone());
            let res = engine.ignite(&[Instr::Operation(ImgOp::Overlay(OverlayInputs::new(
                ImageFromPath::new(overlay.into()),
                (bounds.0, bounds.1),
            )))]);

            let res_image = res.unwrap();
            assert_eq!(img.raw_pixels(), res_image.raw_pixels());

            output_test_image_for_manual_inspection(
                &res_image,
                out_!("test_overlay_self_bounds.png"),
            );
        }

        #[test]
        fn overlay_with_self_se_quarter() {
            let img = setup_default_test_image();
            let bounds = img.dimensions();

            let overlay = sic_testing::in_!("unsplash_763569_cropped.jpg");

            let mut engine = ImageEngine::new(img.clone());
            let res = engine.ignite(&[
                Instr::Operation(ImgOp::Invert),
                Instr::Operation(ImgOp::Overlay(OverlayInputs::new(
                    ImageFromPath::new(overlay.into()),
                    (bounds.0 / 2, bounds.1 / 2),
                ))),
            ]);

            let res_image = res.unwrap();
            assert_ne!(img.raw_pixels(), res_image.raw_pixels());

            output_test_image_for_manual_inspection(
                &res_image,
                out_!("test_overlay_self_se_quarter.png"),
            );
        }
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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);

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
        let done = operator.ignite(&[Instr::Operation(operation)]);
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
        let done = operator.ignite(&[Instr::Operation(operation)]);
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
            Instr::Operation(ImgOp::Resize((80, 100))),
            Instr::Operation(ImgOp::Blur(5.0)),
            Instr::Operation(ImgOp::FlipHorizontal),
            Instr::Operation(ImgOp::FlipVertical),
            Instr::Operation(ImgOp::Rotate90),
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

    #[cfg(feature = "imageproc-ops")]
    mod imageproc_ops_tests {
        use super::*;
        use crate::wrapper::draw_text_inner::DrawTextInner;
        use crate::wrapper::font_options::{FontOptions, FontScale};

        #[test]
        fn draw_text() {
            let img: DynamicImage =
                DynamicImage::ImageRgb8(sic_core::image::RgbImage::new(200, 200));

            let font_file = Into::<PathBuf>::into(env!("CARGO_MANIFEST_DIR"))
                .join("../../resources/font/Lato-Regular.ttf");

            let operation = ImgOp::DrawText(DrawTextInner::new(
                "HELLO WORLD".to_string(),
                (0, 0),
                FontOptions::new(
                    font_file,
                    Rgba([255, 255, 0, 255]),
                    FontScale::Uniform(16.0),
                ),
            ));

            let mut operator = ImageEngine::new(img);
            let done = operator.ignite(&[Instr::Operation(operation)]);
            assert!(done.is_ok());

            let result_img = done.unwrap();

            output_test_image_for_manual_inspection(
                &result_img,
                out_!("test_imageproc_ops_draw__text.png"),
            );
        }
    }
}
