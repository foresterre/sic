use std::fmt::{Debug, Formatter};
use std::hash::Hash;

use crate::errors::SicImageEngineError;
use sic_core::image::FilterType;

#[derive(Clone, Copy)]
pub struct FilterTypeWrap {
    inner: FilterType,
}

impl FilterTypeWrap {
    pub fn new(with: FilterType) -> Self {
        Self { inner: with }
    }
}

impl PartialEq<FilterTypeWrap> for FilterTypeWrap {
    fn eq(&self, other: &FilterTypeWrap) -> bool {
        std::mem::discriminant(&self.inner) == std::mem::discriminant(&other.inner)
    }
}

impl Eq for FilterTypeWrap {}

impl Hash for FilterTypeWrap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(&self.inner).hash(state)
    }
}

impl Debug for FilterTypeWrap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let msg = match self.inner {
            FilterType::CatmullRom => "image::FilterType::CatmullRom (Wrapper)",
            FilterType::Gaussian => "image::FilterType::Gaussian (Wrapper)",
            FilterType::Lanczos3 => "image::FilterType::Lanczos3 (Wrapper)",
            FilterType::Nearest => "image::FilterType::Nearest (Wrapper)",
            FilterType::Triangle => "image::FilterType::Triangle (Wrapper)",
        };

        f.write_str(msg)
    }
}

impl From<FilterTypeWrap> for FilterType {
    fn from(wrap: FilterTypeWrap) -> Self {
        wrap.inner
    }
}

impl FilterTypeWrap {
    pub fn try_from_str(val: &str) -> Result<FilterTypeWrap, SicImageEngineError> {
        match val.to_lowercase().as_str() {
            "catmullrom" | "cubic" => Ok(FilterTypeWrap::new(FilterType::CatmullRom)),
            "gaussian" => Ok(FilterTypeWrap::new(FilterType::Gaussian)),
            "lanczos3" => Ok(FilterTypeWrap::new(FilterType::Lanczos3)),
            "nearest" => Ok(FilterTypeWrap::new(FilterType::Nearest)),
            "triangle" => Ok(FilterTypeWrap::new(FilterType::Triangle)),
            fail => Err(SicImageEngineError::UnknownFilterType(fail.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq() {
        let wrapped_cat1 = FilterTypeWrap::new(FilterType::CatmullRom);
        let wrapped_cat2 = FilterTypeWrap::new(FilterType::CatmullRom);

        assert!(wrapped_cat1.eq(&wrapped_cat2));
    }

    #[test]
    fn partial_ne() {
        let wrapped_cat = FilterTypeWrap::new(FilterType::CatmullRom);
        let wrapped_gauss = FilterTypeWrap::new(FilterType::Gaussian);

        assert!(wrapped_cat.ne(&wrapped_gauss));
    }
}
