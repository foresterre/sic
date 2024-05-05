use crate::errors::SicIoError;
use crate::preprocessor::Preprocess;
use sic_core::{image, SicImage};

pub struct PickFramePreprocessor {
    format: image::ImageFormat,
}

impl Preprocess for PickFramePreprocessor {
    type Err = SicIoError;

    fn preprocess(self, image: SicImage) -> Result<SicImage, Self::Err> {
        // encode_animated_image currently does this, but that's unfortunate, because:
        //
        // cargo run -- --glob-input "resources/*.png" --glob-output "globtest/modified_format_and_ext/" --output-format jpeg
        //     Finished dev [unoptimized + debuginfo] target(s) in 0.06s
        //      Running `target/debug/sic --glob-input 'resources/*.png' --glob-output globtest/modified_format_and_ext/ --output-format jpeg`
        // WARN: Unable to encode animated image buffer with format 'Jpeg': encoding first frame only
        // Error: With input: ./resources/apng_sample.png
        //
        // Caused by:
        //     0: Unable to write image
        //     1: The encoder or decoder for Jpeg does not support the color type `Rgba8`
        //
        // This dodges the ColorType preprocessor because that one currently only does SicImage::Static
        // TODO: consider, fixing it by adding this extra pre processor (but now order of preprocessor becomes important), or via DynamicEncoder, or
        //   maybe generic over SicImage<Frame>
        todo!()
    }
}
