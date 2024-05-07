use sic_core::image;
use std::io::{Seek, Write};

/// Wrapper for [`BmpEncoder`], which takes the writer by value, instead of by mutable reference.
/// All other encoders in `image` take the writer by value. Our [`DynamicEncoder`] wraps all formats
/// and also requires the writer to be given by value. This wrapper creates a new [`BmpEncoder`]
/// when writing the image, so it doesn't have to hold on to a mutable reference to its internal
/// writer.
///
/// [`BmpEncoder`]: image::codecs::bmp::BmpEncoder
pub struct BmpEncoder<W> {
    writer: W,
}

impl<W: Write + Seek> BmpEncoder<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<W: Write + Seek> image::ImageEncoder for BmpEncoder<W> {
    fn write_image(
        mut self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: image::ExtendedColorType,
    ) -> image::ImageResult<()> {
        image::codecs::bmp::BmpEncoder::new(&mut self.writer)
            .write_image(buf, width, height, color_type)
    }
}
