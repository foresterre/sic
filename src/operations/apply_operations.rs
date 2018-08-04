use std::path::Path;
use image::{DynamicImage, FilterType};

use super::Operation;


pub trait ApplyOperation {
  fn apply_on(&self, img: &DynamicImage) -> Result<DynamicImage, String>;
}

impl ApplyOperation for Operation {
  fn apply_on(&self, img: &DynamicImage) ->  Result<DynamicImage, String> {
      match *self {
        Operation::Blur(sigma) => Ok(img.blur(sigma as f32)),
        Operation::FlipHorizontal => Ok(img.fliph()),
        Operation::FlipVertical => Ok(img.flipv()),
        Operation::Resize(new_x, new_y) => Ok(img.resize_exact(new_x, new_y, FilterType::Gaussian)),
        _ => unimplemented!(),
      }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IMAGE_PATH: &str = "resources/unsplash_763569_cropped.jpg";

    fn setup() -> DynamicImage {
      image::open(&Path::new(TEST_IMAGE_PATH)).unwrap()
    }

    fn manual_inspection(img: &DynamicImage, path: &str) {
      if !cfg!(feature = "dont-run-on-ci") {
        let _ = img.save(path);
      }
    }

    #[test]
    fn test_blur() {
      let img: DynamicImage = setup();
      let operation = Operation::Blur(25);

      let done = operation.apply_on(&img);
      assert!(done.is_ok());

      manual_inspection(&done.unwrap(), "target/test_blur.png")
    }

      #[test]
    fn test_fliph() {
      let img: DynamicImage = setup();
      let operation = Operation::FlipHorizontal;

      let done = operation.apply_on(&img);
      assert!(done.is_ok());

      manual_inspection(&done.unwrap(), "target/test_fliph.png")
    }

    #[test]
    fn test_flipv() {
      let img: DynamicImage = setup();
      let operation = Operation::FlipVertical;

      let done = operation.apply_on(&img);
      assert!(done.is_ok());

      manual_inspection(&done.unwrap(), "target/test_flipv.png")
    }


    #[test]
    fn test_resize_up_gaussian() {
      // 217x447px => 400x500
      let img: DynamicImage = setup();
      let operation = Operation::Resize(400, 500);

      let done = operation.apply_on(&img);
      assert!(done.is_ok());

      manual_inspection(&done.unwrap(), "target/test_scale_400x500.png")
    }

    #[test]
    fn test_resize_down_gaussian() {
      // 217x447px => 100x200
      let img: DynamicImage = setup();
      let operation = Operation::Resize(100, 200);

      let done = operation.apply_on(&img);
      assert!(done.is_ok());

      manual_inspection(&done.unwrap(), "target/test_scale_100x200.png")
    }


}



