use crate::encode_settings::jpeg::JpegQuality;
use sic_core::image;
use std::io::{Seek, Write};

/// Box wrapper for [`JpegEncoder`], which is at least 4187 bytes large, exploding the size of the
/// [`DynamicEncoder`].
pub struct JpegEncoder<W> {
    writer: Box<image::codecs::jpeg::JpegEncoder<W>>,
}

impl<W: Write> JpegEncoder<W> {
    pub fn new(writer: W, quality: JpegQuality) -> Self {
        Self {
            writer: Box::new(image::codecs::jpeg::JpegEncoder::new_with_quality(
                writer,
                quality.as_u8(),
            )),
        }
    }
}

impl<W: Write + Seek> image::ImageEncoder for JpegEncoder<W> {
    fn write_image(
        self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: image::ExtendedColorType,
    ) -> image::ImageResult<()> {
        self.writer.write_image(buf, width, height, color_type)
    }
}
