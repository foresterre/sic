/// This struct ensures no invalid JPEG qualities can be stored.
/// Using this struct instead of `u8` directly should ensure no panics occur because of invalid
/// quality values.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct JpegQuality(pub u8);

impl Default for JpegQuality {
    /// The default JPEG quality is `80`.
    fn default() -> Self {
        Self(80)
    }
}

impl JpegQuality {
    /// Returns an Ok result if the quality requested is between 1 and 100 (inclusive).
    pub fn try_from(quality: u8) -> Result<Self, JpegQualityError> {
        if (1u8..=100u8).contains(&quality) {
            Ok(JpegQuality(quality))
        } else {
            Err(JpegQualityError { value: quality })
        }
    }

    /// Return the valid quality value.
    pub fn as_u8(self) -> u8 {
        self.0
    }
}

#[derive(Debug, thiserror::Error)]
#[error("JPEG quality should range between 1 and 100 (inclusive), but was {}", .value)]
pub struct JpegQualityError {
    pub value: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jpeg_quality_in_range_lower() {
        let result = JpegQuality::try_from(1).unwrap();
        let expected = JpegQuality(1);

        assert_eq!(result, expected);
    }

    #[test]
    fn jpeg_quality_in_range_upper() {
        let result = JpegQuality::try_from(100).unwrap();
        let expected = JpegQuality(100);

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic]
    fn jpeg_quality_out_range_lower() {
        JpegQuality::try_from(0).unwrap();
    }

    #[test]
    #[should_panic]
    fn jpeg_quality_out_range_upper() {
        JpegQuality::try_from(101).unwrap();
    }
}
