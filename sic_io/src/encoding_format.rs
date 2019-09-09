use std::path::Path;

use sic_core::image;
use std::error::Error;

use crate::export::ExportMethod;

const DEFAULT_PIPED_OUTPUT_FORMAT: image::ImageOutputFormat = image::ImageOutputFormat::BMP;

pub trait EncodingFormatByMethod {
    /// Determine the encoding format based on the method of exporting.
    fn by_method<P: AsRef<Path>>(
        &self,
        method: &ExportMethod<P>,
    ) -> Result<image::ImageOutputFormat, Box<dyn Error>>;
}

pub trait EncodingFormatByExtension {
    /// Determine the encoding format based on the extension of a file path.
    fn by_extension<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<image::ImageOutputFormat, Box<dyn Error>>;
}

pub trait EncodingFormatByIdentifier {
    /// Determine the encoding format based on the method of exporting.
    /// Determine the encoding format based on a recognized given identifier.
    fn by_identifier(&self, identifier: &str) -> Result<image::ImageOutputFormat, Box<dyn Error>>;
}

pub trait EncodingFormatJPEGQuality {
    /// Returns a validated jpeg quality value.
    /// If no such value exists, it will return an error instead.
    fn jpeg_quality(&self) -> Result<JPEGQuality, Box<dyn Error>>;
}

pub trait EncodingFormatPNMSampleEncoding {
    /// Returns a pnm sample encoding type.
    /// If no such value exists, it will return an error instead.
    fn pnm_encoding_type(&self) -> Result<image::pnm::SampleEncoding, Box<dyn Error>>;
}

/// This struct ensures no invalid JPEG qualities can be stored.
/// Using this struct instead of `u8` directly should ensure no panics occur because of invalid
/// quality values.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct JPEGQuality {
    quality: u8,
}

impl Default for JPEGQuality {
    /// The default JPEG quality is `80`.
    fn default() -> Self {
        Self { quality: 80 }
    }
}

impl JPEGQuality {
    /// Returns an Ok result if the quality requested is between 1 and 100 (inclusive).
    pub fn try_from(quality: u8) -> Result<Self, Box<dyn Error>> {
        if (1u8..=100u8).contains(&quality) {
            Ok(JPEGQuality { quality })
        } else {
            let message = "JPEG Quality should range between 1 and 100 (inclusive).";
            Err(From::from(message.to_string()))
        }
    }

    /// Return the valid quality value.
    pub fn as_u8(self) -> u8 {
        self.quality
    }
}

impl EncodingFormatByMethod for DetermineEncodingFormat {
    /// Determine the encoding format based on the method of exporting.
    /// For stdout, the default piped output format will be used.
    ///     If another format is wanted the `by_identifier` function should be used instead.
    /// For file, the format will be determined based on the output path extension.
    fn by_method<P: AsRef<Path>>(
        &self,
        method: &ExportMethod<P>,
    ) -> Result<image::ImageOutputFormat, Box<dyn Error>> {
        match method {
            ExportMethod::StdoutBytes => Ok(DEFAULT_PIPED_OUTPUT_FORMAT),
            ExportMethod::File(path) => self.by_extension(path),
        }
    }
}

impl EncodingFormatByExtension for DetermineEncodingFormat {
    /// Determines the encoding format based on the extension of the given path.
    /// If the path has no extension, it will return an error.
    /// The extension if existing is matched against the identifiers, which currently
    /// are the extensions used.
    fn by_extension<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<image::ImageOutputFormat, Box<dyn Error>> {
        let extension = path.as_ref().extension().and_then(|v| v.to_str());

        match extension {
            Some(some) => self.by_identifier(some),
            None => Err(From::from({
                let message = "Unable to determine output format from extension.";
                message.to_string()
            })),
        }
    }
}

impl EncodingFormatByIdentifier for DetermineEncodingFormat {
    /// Determines an image output format based on a given `&str` identifier.
    /// Identifiers are based on common output file extensions.
    fn by_identifier(&self, identifier: &str) -> Result<image::ImageOutputFormat, Box<dyn Error>> {
        match identifier {
            "bmp" => Ok(image::ImageOutputFormat::BMP),
            "gif" => Ok(image::ImageOutputFormat::GIF),
            "ico" => Ok(image::ImageOutputFormat::ICO),
            "jpeg" | "jpg" => Ok(image::ImageOutputFormat::JPEG(self.jpeg_quality()?.as_u8())),
            "png" => Ok(image::ImageOutputFormat::PNG),
            "pbm" => Ok(image::ImageOutputFormat::PNM(
                image::pnm::PNMSubtype::Bitmap(self.pnm_encoding_type()?),
            )),
            "pgm" => Ok(image::ImageOutputFormat::PNM(
                image::pnm::PNMSubtype::Graymap(self.pnm_encoding_type()?),
            )),
            "ppm" => Ok(image::ImageOutputFormat::PNM(
                image::pnm::PNMSubtype::Pixmap(self.pnm_encoding_type()?),
            )),
            "pam" => Ok(image::ImageOutputFormat::PNM(
                image::pnm::PNMSubtype::ArbitraryMap,
            )),
            _ => Err(From::from(format!(
                "No supported image output format was found, input: {}.",
                identifier
            ))),
        }
    }
}

pub struct DetermineEncodingFormat {
    pub pnm_sample_encoding: Option<image::pnm::SampleEncoding>,
    pub jpeg_quality: Option<JPEGQuality>,
}

impl EncodingFormatPNMSampleEncoding for DetermineEncodingFormat {
    fn pnm_encoding_type(&self) -> Result<image::pnm::SampleEncoding, Box<dyn Error>> {
        self.pnm_sample_encoding.ok_or_else(|| {
            let message = "Using PNM requires the sample encoding to be set.";
            From::from(message.to_string())
        })
    }
}

impl EncodingFormatJPEGQuality for DetermineEncodingFormat {
    fn jpeg_quality(&self) -> Result<JPEGQuality, Box<dyn Error>> {
        self.jpeg_quality.ok_or_else(|| {
            let message = "Using JPEG requires the JPEG quality to be set.";

            From::from(message.to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_FORMATS: &[&str] = &[
        "bmp", "gif", "ico", "jpg", "jpeg", "png", "pbm", "pgm", "ppm", "pam",
    ];
    const EXPECTED_VALUES: &[image::ImageOutputFormat] = &[
        image::ImageOutputFormat::BMP,
        image::ImageOutputFormat::GIF,
        image::ImageOutputFormat::ICO,
        image::ImageOutputFormat::JPEG(80),
        image::ImageOutputFormat::JPEG(80),
        image::ImageOutputFormat::PNG,
        image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Bitmap(
            image::pnm::SampleEncoding::Binary,
        )),
        image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Graymap(
            image::pnm::SampleEncoding::Binary,
        )),
        image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Pixmap(
            image::pnm::SampleEncoding::Binary,
        )),
        image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::ArbitraryMap),
    ];

    fn setup_default_format_determiner() -> DetermineEncodingFormat {
        DetermineEncodingFormat {
            pnm_sample_encoding: Some(image::pnm::SampleEncoding::Binary),
            jpeg_quality: Some(JPEGQuality::try_from(80).unwrap()),
        }
    }

    //
    fn test_with_method_path(ext: &str, expected: &image::ImageOutputFormat) {
        let path = format!("w_path.{}", ext);
        let method = ExportMethod::File(path);

        let format_determiner = setup_default_format_determiner();
        let result = format_determiner.by_method(&method);

        assert_eq!(result.unwrap(), *expected);
    }

    #[test]
    fn method_path_with_defaults() {
        let zipped = INPUT_FORMATS.iter().zip(EXPECTED_VALUES.iter());

        for (ext, exp) in zipped {
            test_with_method_path(ext, exp);
        }
    }

    //
    fn test_with_extensions(ext: &str, expected: &image::ImageOutputFormat) {
        let path = format!("w_ext.{}", ext);

        let format_determiner = setup_default_format_determiner();
        let result = format_determiner.by_extension(path.as_str());

        assert_eq!(result.unwrap(), *expected);
    }

    #[test]
    fn extension_with_defaults() {
        let zipped = INPUT_FORMATS.iter().zip(EXPECTED_VALUES.iter());

        for (ext, exp) in zipped {
            test_with_extensions(ext, exp);
        }
    }

    //
    #[test]
    #[should_panic]
    fn extension_unknown_extension() {
        let path = format!("w_ext.h");

        let format_determiner = setup_default_format_determiner();
        let result = format_determiner.by_extension(path.as_str());

        result.unwrap();
    }

    //
    #[test]
    #[should_panic]
    fn extension_no_extension() {
        let path = format!("png");

        let format_determiner = setup_default_format_determiner();
        let result = format_determiner.by_extension(path.as_str());

        result.unwrap();
    }

    //
    #[test]
    #[should_panic]
    fn extension_invalid_extension() {
        let path = format!(".png");

        let format_determiner = setup_default_format_determiner();
        let result = format_determiner.by_extension(path.as_str());

        result.unwrap();
    }

    //
    fn test_with_identifier(identifier: &str, expected: &image::ImageOutputFormat) {
        let format_determiner = setup_default_format_determiner();
        let result = format_determiner.by_identifier(identifier);

        assert_eq!(result.unwrap(), *expected);
    }

    #[test]
    fn identifier_with_defaults() {
        let zipped = INPUT_FORMATS.iter().zip(EXPECTED_VALUES.iter());

        for (id, exp) in zipped {
            test_with_identifier(id, exp);
        }
    }

    //
    #[test]
    #[should_panic]
    fn identifier_unknown_identifier() {
        let format_determiner = setup_default_format_determiner();
        let result = format_determiner.by_identifier("");

        result.unwrap();
    }

    // non default: pnm ascii + "pbm"
    #[test]
    fn identifier_custom_pnm_sample_encoding_ascii_pbm() {
        let format_determiner = DetermineEncodingFormat {
            pnm_sample_encoding: Some(image::pnm::SampleEncoding::Ascii),
            jpeg_quality: None,
        };

        let result = format_determiner.by_identifier("pbm").unwrap();
        let expected = image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Bitmap(
            image::pnm::SampleEncoding::Ascii,
        ));

        assert_eq!(result, expected);
    }

    // non default: pnm ascii + "pgm"
    #[test]
    fn identifier_custom_pnm_sample_encoding_ascii_pgm() {
        let format_determiner = DetermineEncodingFormat {
            pnm_sample_encoding: Some(image::pnm::SampleEncoding::Ascii),
            jpeg_quality: None,
        };

        let result = format_determiner.by_identifier("pgm").unwrap();
        let expected = image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Graymap(
            image::pnm::SampleEncoding::Ascii,
        ));

        assert_eq!(result, expected);
    }

    // non default: pnm ascii + "ppm"
    #[test]
    fn identifier_custom_pnm_sample_encoding_ascii_ppm() {
        let format_determiner = DetermineEncodingFormat {
            pnm_sample_encoding: Some(image::pnm::SampleEncoding::Ascii),
            jpeg_quality: None,
        };

        let result = format_determiner.by_identifier("ppm").unwrap();
        let expected = image::ImageOutputFormat::PNM(image::pnm::PNMSubtype::Pixmap(
            image::pnm::SampleEncoding::Ascii,
        ));

        assert_eq!(result, expected);
    }

    // non default: jpeg custom, quality lower bound
    #[test]
    fn identifier_custom_jpeg_quality_in_range_lower() {
        let format_determiner = DetermineEncodingFormat {
            pnm_sample_encoding: None,
            jpeg_quality: Some(JPEGQuality::try_from(1).unwrap()),
        };

        let result = format_determiner.by_identifier("jpg").unwrap();
        let expected = image::ImageOutputFormat::JPEG(1);

        assert_eq!(result, expected);
    }

    // non default: jpeg custom, quality upper bound
    #[test]
    fn identifier_custom_jpeg_quality_in_range_upper() {
        let format_determiner = DetermineEncodingFormat {
            pnm_sample_encoding: None,
            jpeg_quality: Some(JPEGQuality::try_from(100).unwrap()),
        };

        let result = format_determiner.by_identifier("jpg").unwrap();
        let expected = image::ImageOutputFormat::JPEG(100);

        assert_eq!(result, expected);
    }

    // if we were to test 'identifier_custom_jpeg_quality_OUT_range_[lower/upper]'
    //                                                    ^^^
    // our DetermineEncodingFormat would fail on creation by JPEGQuality::try_from which fails
    // on outbound ranges

    //
    #[test]
    fn jpeg_quality_in_range_lower() {
        let result = JPEGQuality::try_from(1).unwrap();
        let expected = JPEGQuality { quality: 1 };

        assert_eq!(result, expected);
    }

    //
    #[test]
    fn jpeg_quality_in_range_upper() {
        let result = JPEGQuality::try_from(100).unwrap();
        let expected = JPEGQuality { quality: 100 };

        assert_eq!(result, expected);
    }

    //
    #[test]
    #[should_panic]
    fn jpeg_quality_out_range_lower() {
        let result = JPEGQuality::try_from(0).unwrap();
        let expected = JPEGQuality { quality: 0 };

        assert_eq!(result, expected);
    }

    //
    #[test]
    #[should_panic]
    fn jpeg_quality_out_range_upper() {
        let result = JPEGQuality::try_from(101).unwrap();
        let expected = JPEGQuality { quality: 101 };

        assert_eq!(result, expected);
    }

    // DetermineEncodingFormat has None, while Some required: pbm
    #[test]
    #[should_panic]
    fn identifier_requires_pnm_sample_encoding_to_be_set_pbm() {
        let format_determiner = DetermineEncodingFormat {
            pnm_sample_encoding: None,
            jpeg_quality: None,
        };

        format_determiner.by_identifier("pbm").unwrap();
    }

    // DetermineEncodingFormat has None, while Some required: pbm
    #[test]
    #[should_panic]
    fn identifier_requires_pnm_sample_encoding_to_be_set_pgm() {
        let format_determiner = DetermineEncodingFormat {
            pnm_sample_encoding: None,
            jpeg_quality: None,
        };

        format_determiner.by_identifier("pgm").unwrap();
    }

    // DetermineEncodingFormat has None, while Some required: ppm
    #[test]
    #[should_panic]
    fn identifier_requires_pnm_sample_encoding_to_be_set_ppm() {
        let format_determiner = DetermineEncodingFormat {
            pnm_sample_encoding: None,
            jpeg_quality: None,
        };

        format_determiner.by_identifier("ppm").unwrap();
    }

    // DetermineEncodingFormat has None, while Some required: jpg
    #[test]
    #[should_panic]
    fn identifier_requires_pnm_sample_encoding_to_be_set_jpg() {
        let format_determiner = DetermineEncodingFormat {
            pnm_sample_encoding: None,
            jpeg_quality: None,
        };

        format_determiner.by_identifier("jpg").unwrap();
    }
}
