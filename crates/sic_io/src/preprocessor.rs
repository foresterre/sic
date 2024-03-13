use sic_core::SicImage;

pub mod color_type;

pub trait Preprocess {
    type Err;

    fn preprocess(self, image: SicImage) -> Result<SicImage, Self::Err>;
}
