use std::fmt;
use std::fmt::Formatter;
use std::io::{Seek, Write};
use std::path::Path;

use sic_core::image;

use crate::errors::SicIoError;
use crate::format::gif::RepeatAnimation;
use crate::format::jpeg::JpegQuality;

pub mod gif;
pub mod jpeg;

pub trait IntoImageEncoder<W: Write + Seek> {
    /// Determine the encoding format based on the extension of a file path.
    fn from_extension(
        writer: W,
        path: &Path,
        settings: &EncoderSettings,
    ) -> Result<DynamicEncoder<W>, SicIoError>;

    /// Determine the encoding format based on a recognized given identifier.
    fn from_identifier(
        writer: W,
        identifier: impl AsRef<str>,
        settings: &EncoderSettings,
    ) -> Result<DynamicEncoder<W>, SicIoError>;
}

pub struct EncoderSettings {
    pub pnm_sample_encoding: image::codecs::pnm::SampleEncoding,
    pub jpeg_quality: JpegQuality,
    pub repeat_animation: RepeatAnimation,
}

impl Default for EncoderSettings {
    fn default() -> Self {
        Self {
            pnm_sample_encoding: image::codecs::pnm::SampleEncoding::Binary,
            jpeg_quality: JpegQuality::default(),
            repeat_animation: RepeatAnimation::default(),
        }
    }
}

#[allow(private_interfaces)]
pub enum DynamicEncoder<W: Write + Seek> {
    Avif(image::codecs::avif::AvifEncoder<W>),
    Bmp(BmpEncoder<W>),
    Exr(image::codecs::openexr::OpenExrEncoder<W>),
    Farbfeld(image::codecs::farbfeld::FarbfeldEncoder<W>),
    Gif(image::codecs::gif::GifEncoder<W>),
    Ico(image::codecs::ico::IcoEncoder<W>),
    Jpeg(JpegEncoder<W>),
    Pnm(image::codecs::pnm::PnmEncoder<W>),
    Png(image::codecs::png::PngEncoder<W>),
    Qoi(image::codecs::qoi::QoiEncoder<W>),
    Tga(image::codecs::tga::TgaEncoder<W>),
    Tiff(image::codecs::tiff::TiffEncoder<W>),
    Webp(image::codecs::webp::WebPEncoder<W>),
}

impl<W: Write + Seek> fmt::Debug for DynamicEncoder<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use DynamicEncoder::*;

        match self {
            Avif(_) => f.write_str("DynamicEncoder(Avif)"),
            Bmp(_) => f.write_str("DynamicEncoder(Bmp)"),
            Exr(_) => f.write_str("DynamicEncoder(Exr)"),
            Farbfeld(_) => f.write_str("DynamicEncoder(Farbfeld)"),
            Gif(_) => f.write_str("DynamicEncoder(Gif)"),
            Ico(_) => f.write_str("DynamicEncoder(Ico)"),
            Jpeg(_) => f.write_str("DynamicEncoder(Jpeg)"),
            Pnm(_) => f.write_str("DynamicEncoder(Pnm)"),
            Png(_) => f.write_str("DynamicEncoder(Png)"),
            Qoi(_) => f.write_str("DynamicEncoder(Qoi)"),
            Tga(_) => f.write_str("DynamicEncoder(Tga)"),
            Tiff(_) => f.write_str("DynamicEncoder(Tiff)"),
            Webp(_) => f.write_str("DynamicEncoder(Webp)"),
        }
    }
}

impl<W: Write + Seek> DynamicEncoder<W> {
    /// Create a BMP encoder.
    pub fn bmp(writer: W) -> Result<DynamicEncoder<W>, SicIoError> {
        Ok(Self::Bmp(BmpEncoder::new(writer)))
    }
}

impl<W: Write + Seek> IntoImageEncoder<W> for DynamicEncoder<W> {
    /// Determines the encoding format based on the extension of the given path.
    /// If the path has no extension, it will return an error.
    /// The extension if existing is matched against the identifiers, which currently
    /// are the extensions used.
    fn from_extension(
        writer: W,
        path: &Path,
        settings: &EncoderSettings,
    ) -> Result<DynamicEncoder<W>, SicIoError> {
        let extension = path.extension().and_then(|v| v.to_str());

        match extension {
            Some(ext) => {
                <DynamicEncoder<W> as IntoImageEncoder<W>>::from_identifier(writer, ext, settings)
            }
            None => Err(SicIoError::UnknownImageFormatFromFileExtension(
                path.to_path_buf(),
            )),
        }
    }

    /// Determines an image output format based on a given `&str` identifier.
    /// Identifiers are based on common output file extensions.
    fn from_identifier(
        writer: W,
        identifier: impl AsRef<str>,
        settings: &EncoderSettings,
    ) -> Result<DynamicEncoder<W>, SicIoError> {
        use DynamicEncoder::*;

        let id = identifier.as_ref();

        Ok(match id.to_ascii_lowercase().as_str() {
            "avif" => Avif(image::codecs::avif::AvifEncoder::new(writer)),
            "bmp" => Bmp(BmpEncoder::new(writer)),
            "exr" => Exr(image::codecs::openexr::OpenExrEncoder::new(writer)),
            "ff" | "farbfeld" => Farbfeld(image::codecs::farbfeld::FarbfeldEncoder::new(writer)),
            "gif" => {
                let mut encoder = image::codecs::gif::GifEncoder::new(writer);
                encoder
                    .set_repeat(settings.repeat_animation.into())
                    .map_err(SicIoError::ImageError)?;
                Gif(encoder)
            }
            "ico" => Ico(image::codecs::ico::IcoEncoder::new(writer)),
            "jpeg" | "jpg" => Jpeg(JpegEncoder::new(writer, settings.jpeg_quality)),
            "pam" => Pnm(image::codecs::pnm::PnmEncoder::new(writer)
                .with_subtype(image::codecs::pnm::PnmSubtype::ArbitraryMap)),
            "pbm" => Pnm(image::codecs::pnm::PnmEncoder::new(writer).with_subtype(
                image::codecs::pnm::PnmSubtype::Bitmap(settings.pnm_sample_encoding),
            )),
            "pgm" => Pnm(image::codecs::pnm::PnmEncoder::new(writer).with_subtype(
                image::codecs::pnm::PnmSubtype::Graymap(settings.pnm_sample_encoding),
            )),
            "png" => Png(image::codecs::png::PngEncoder::new(writer)),
            "ppm" => Pnm(image::codecs::pnm::PnmEncoder::new(writer).with_subtype(
                image::codecs::pnm::PnmSubtype::Pixmap(settings.pnm_sample_encoding),
            )),
            "qoi" => Qoi(image::codecs::qoi::QoiEncoder::new(writer)),
            "tga" => Tga(image::codecs::tga::TgaEncoder::new(writer)),
            "tiff" | "tif" => Tiff(image::codecs::tiff::TiffEncoder::new(writer)),
            "webp" => Webp(image::codecs::webp::WebPEncoder::new_lossless(writer)),
            _ => return Err(SicIoError::UnknownImageFormat(id.to_string())),
        })
    }
}

impl<W: Write + Seek> image::ImageEncoder for DynamicEncoder<W> {
    fn write_image(
        self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: image::ExtendedColorType,
    ) -> image::ImageResult<()> {
        match self {
            Self::Avif(enc) => enc.write_image(buf, width, height, color_type),
            Self::Bmp(enc) => enc.write_image(buf, width, height, color_type),
            Self::Exr(enc) => enc.write_image(buf, width, height, color_type),
            Self::Farbfeld(enc) => enc.write_image(buf, width, height, color_type),
            Self::Gif(mut enc) => {
                // The `ColorTypePreprocessor` will, if enabled, convert the image to `RgbaImage`
                // if necessary.
                // This is unfortunate though, we're making a copy for sauce.
                let image_buffer = image::RgbaImage::from_raw(width, height, buf.to_vec())
                    .ok_or_else(|| {
                        image::ImageError::Encoding(image::error::EncodingError::new(
                            image::error::ImageFormatHint::Exact(image::ImageFormat::Gif),
                            "sic: Unable to construct frame from raw buffer".to_string(),
                        ))
                    })?;

                enc.encode_frame(image::Frame::new(image_buffer))
            }
            Self::Ico(enc) => enc.write_image(buf, width, height, color_type),
            Self::Jpeg(enc) => enc.write_image(buf, width, height, color_type),
            Self::Pnm(enc) => enc.write_image(buf, width, height, color_type),
            Self::Png(enc) => enc.write_image(buf, width, height, color_type),
            Self::Qoi(enc) => enc.write_image(buf, width, height, color_type),
            Self::Tga(enc) => enc.write_image(buf, width, height, color_type),
            Self::Tiff(enc) => enc.write_image(buf, width, height, color_type),
            Self::Webp(enc) => enc.write_image(buf, width, height, color_type),
        }
    }
}

impl<W: Write + Seek> DynamicEncoder<W> {
    pub fn format(&self) -> image::ImageFormat {
        match self {
            Self::Avif(_) => image::ImageFormat::Avif,
            Self::Bmp(_) => image::ImageFormat::Bmp,
            Self::Exr(_) => image::ImageFormat::Jpeg,
            Self::Farbfeld(_) => image::ImageFormat::Farbfeld,
            Self::Gif(_) => image::ImageFormat::Gif,
            Self::Ico(_) => image::ImageFormat::Ico,
            Self::Jpeg(_) => image::ImageFormat::Jpeg,
            Self::Pnm(_) => image::ImageFormat::Pnm,
            Self::Png(_) => image::ImageFormat::Pnm,
            Self::Qoi(_) => image::ImageFormat::Qoi,
            Self::Tga(_) => image::ImageFormat::Tga,
            Self::Tiff(_) => image::ImageFormat::Tiff,
            Self::Webp(_) => image::ImageFormat::WebP,
        }
    }
}

/// Wrapper for [`BmpEncoder`], which takes the writer by value, instead of by mutable reference.
/// All other encoders in `image` take the writer by value. Our [`DynamicEncoder`] wraps all formats
/// and also requires the writer to be given by value. This wrapper creates a new [`BmpEncoder`]
/// when writing the image, so it doesn't have to hold on to a mutable reference to its internal
/// writer.
///
/// [`BmpEncoder`]: image::codecs::bmp::BmpEncoder
struct BmpEncoder<W> {
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

/// Box wrapper for [`JpegEncoder`], which is at least 4187 bytes large, exploding the size of the
/// [`DynamicEncoder`].
struct JpegEncoder<W> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;
    use std::io::SeekFrom;

    #[derive(Debug)]
    struct DummyMem;

    impl Seek for DummyMem {
        fn seek(&mut self, _pos: SeekFrom) -> std::io::Result<u64> {
            Ok(0)
        }
    }

    impl Write for DummyMem {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[parameterized(
        ext = {
            "avif",
            "bmp",
            "exr",
            "farbfeld",
            "gif",
            "ico",
            "jpg",
            "jpeg",
            "png",
            "pbm",
            "pgm",
            "ppm",
            "pam",
            "qoi",
            "tga",
            "tiff",
            "tif"
        },
        expected = {
            image::ImageFormat::Avif,
            image::ImageFormat::Bmp,
            image::ImageFormat::OpenExr,
            image::ImageFormat::Farbfeld,
            image::ImageFormat::Gif,
            image::ImageFormat::Ico,
            image::ImageFormat::Jpeg,
            image::ImageFormat::Jpeg,
            image::ImageFormat::Png,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
            image::ImageFormat::Qoi,
            image::ImageFormat::Tga,
            image::ImageFormat::Tiff,
            image::ImageFormat::Tiff,
        }
    )]
    fn test_with_extensions(ext: &str, expected: image::ImageFormat) {
        let path = format!("image.{}", ext);
        let path = Path::new(&path);

        let settings = EncoderSettings::default();
        let mut mem = DummyMem;
        let dynamic_encoder = DynamicEncoder::from_extension(&mut mem, path, &settings);

        assert_eq!(dynamic_encoder.unwrap().format(), expected);
    }

    #[parameterized(
        identifier = {
            "avif",
            "bmp",
            "exr",
            "farbfeld",
            "gif",
            "ico",
            "jpg",
            "jpeg",
            "png",
            "pbm",
            "pgm",
            "ppm",
            "pam",
            "qoi",
            "tga",
            "tiff",
            "tif"
        },
        expected = {
            image::ImageFormat::Avif,
            image::ImageFormat::Bmp,
            image::ImageFormat::OpenExr,
            image::ImageFormat::Farbfeld,
            image::ImageFormat::Gif,
            image::ImageFormat::Ico,
            image::ImageFormat::Jpeg,
            image::ImageFormat::Jpeg,
            image::ImageFormat::Png,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
            image::ImageFormat::Qoi,
            image::ImageFormat::Tga,
            image::ImageFormat::Tiff,
            image::ImageFormat::Tiff,
        }
    )]
    fn test_with_identifier(identifier: &str, expected: image::ImageFormat) {
        let settings = EncoderSettings::default();
        let mut mem = DummyMem;
        let dynamic_encoder = DynamicEncoder::from_identifier(&mut mem, identifier, &settings);

        assert_eq!(dynamic_encoder.unwrap().format(), expected);
    }

    #[parameterized(
        identifier = {
            "AVIF",
            "BMP",
            "EXR",
            "FARBFELD",
            "GIF",
            "ICO",
            "JPG",
            "JPEG",
            "PNG",
            "PBM",
            "PGM",
            "PPM",
            "PAM",
            "QOI",
            "TGA",
            "TIFF",
            "TIF"
        },
        expected = {
            image::ImageFormat::Avif,
            image::ImageFormat::Bmp,
            image::ImageFormat::OpenExr,
            image::ImageFormat::Farbfeld,
            image::ImageFormat::Gif,
            image::ImageFormat::Ico,
            image::ImageFormat::Jpeg,
            image::ImageFormat::Jpeg,
            image::ImageFormat::Png,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
            image::ImageFormat::Pnm,
            image::ImageFormat::Qoi,
            image::ImageFormat::Tga,
            image::ImageFormat::Tiff,
            image::ImageFormat::Tiff,
        }
    )]
    fn test_with_identifier_uppercase(identifier: &str, expected: image::ImageFormat) {
        let settings = EncoderSettings::default();
        let mut mem = DummyMem;
        let dynamic_encoder = DynamicEncoder::from_identifier(&mut mem, identifier, &settings);

        assert_eq!(dynamic_encoder.unwrap().format(), expected);
    }

    #[test]
    fn extension_unknown_extension() {
        let path = Path::new("w_ext.h");
        let settings = EncoderSettings::default();
        let mut mem = DummyMem;
        let dynamic_encoder = DynamicEncoder::from_extension(&mut mem, path, &settings);

        assert_eq!(
            dynamic_encoder.unwrap_err(),
            SicIoError::UnknownImageFormatFromFileExtension(Path::new("w_ext.h").to_path_buf())
        );
    }

    #[test]
    fn extension_no_extension() {
        let path = Path::new("png");
        let settings = EncoderSettings::default();
        let mut mem = DummyMem;
        let dynamic_encoder = DynamicEncoder::from_extension(&mut mem, path, &settings);

        assert_eq!(
            dynamic_encoder.unwrap_err(),
            SicIoError::UnknownImageFormatFromFileExtension(Path::new("png").to_path_buf())
        );
    }

    #[test]
    fn extension_invalid_extension() {
        let path = Path::new(".png");
        let settings = EncoderSettings::default();
        let mut mem = DummyMem;
        let dynamic_encoder = DynamicEncoder::from_extension(&mut mem, path, &settings);

        assert_eq!(
            dynamic_encoder.unwrap_err(),
            SicIoError::UnknownImageFormatFromFileExtension(Path::new(".png").to_path_buf())
        );
    }

    #[test]
    fn identifier_unknown_identifier() {
        let path = Path::new("");
        let settings = EncoderSettings::default();
        let mut mem = DummyMem;
        let dynamic_encoder = DynamicEncoder::from_extension(&mut mem, path, &settings);

        assert_eq!(
            dynamic_encoder.unwrap_err(),
            SicIoError::UnknownImageFormatFromFileExtension(Path::new("").to_path_buf())
        );
    }

    // non default: pnm ascii + "pbm"
    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn identifier_custom_pnm_sample_encoding_ascii_pbm() {
        let mut settings = EncoderSettings::default();
        settings.pnm_sample_encoding = image::codecs::pnm::SampleEncoding::Ascii;

        let mut mem = DummyMem;
        let dynamic_encoder = DynamicEncoder::from_identifier(&mut mem, "pbm", &settings).unwrap();

        assert_eq!(dynamic_encoder.format(), image::ImageFormat::Pnm);
    }

    // non default: pnm ascii + "pgm"
    #[parameterized(
        identifier = {
            "pbm",
            "pgm",
            "ppm",
        }
    )]
    #[allow(clippy::field_reassign_with_default)]
    fn identifier_custom_pnm_sample_encoding_ascii_pgm(identifier: &str) {
        let mut settings = EncoderSettings::default();
        settings.pnm_sample_encoding = image::codecs::pnm::SampleEncoding::Ascii;

        let mut mem = DummyMem;
        let dynamic_encoder =
            DynamicEncoder::from_identifier(&mut mem, identifier, &settings).unwrap();

        assert_eq!(dynamic_encoder.format(), image::ImageFormat::Pnm);
    }
}
