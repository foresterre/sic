use std::error::Error;
use std::fmt::{Debug, Formatter};

use sic_core::image::FilterType;

// Wrapper for image::FilterType.
// Does only exists, because image::FilterType does not implement PartialEq and Debug.
#[derive(Copy)]
pub enum FilterTypeWrap {
    Inner(FilterType),
}

impl PartialEq<FilterTypeWrap> for FilterTypeWrap {
    fn eq(&self, other: &FilterTypeWrap) -> bool {
        match (self, other) {
            (
                FilterTypeWrap::Inner(FilterType::CatmullRom),
                FilterTypeWrap::Inner(FilterType::CatmullRom),
            ) => true,
            (
                FilterTypeWrap::Inner(FilterType::Gaussian),
                FilterTypeWrap::Inner(FilterType::Gaussian),
            ) => true,
            (
                FilterTypeWrap::Inner(FilterType::Lanczos3),
                FilterTypeWrap::Inner(FilterType::Lanczos3),
            ) => true,
            (
                FilterTypeWrap::Inner(FilterType::Nearest),
                FilterTypeWrap::Inner(FilterType::Nearest),
            ) => true,
            (
                FilterTypeWrap::Inner(FilterType::Triangle),
                FilterTypeWrap::Inner(FilterType::Triangle),
            ) => true,
            _ => false,
        }
    }
}

impl Clone for FilterTypeWrap {
    fn clone(&self) -> Self {
        match self {
            FilterTypeWrap::Inner(a) => FilterTypeWrap::Inner(*a),
        }
    }
}

impl Eq for FilterTypeWrap {}

impl Debug for FilterTypeWrap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let msg = match self {
            FilterTypeWrap::Inner(FilterType::CatmullRom) => {
                "image::FilterType::CatmullRom (Wrapper)"
            }
            FilterTypeWrap::Inner(FilterType::Gaussian) => "image::FilterType::Gaussian (Wrapper)",
            FilterTypeWrap::Inner(FilterType::Lanczos3) => "image::FilterType::Lanczos3 (Wrapper)",
            FilterTypeWrap::Inner(FilterType::Nearest) => "image::FilterType::Nearest (Wrapper)",
            FilterTypeWrap::Inner(FilterType::Triangle) => "image::FilterType::Triangle (Wrapper)",
        };

        f.write_str(msg)
    }
}

impl From<FilterTypeWrap> for FilterType {
    fn from(wrap: FilterTypeWrap) -> Self {
        match wrap {
            FilterTypeWrap::Inner(w) => w,
        }
    }
}

impl FilterTypeWrap {
    pub fn try_from_str(val: &str) -> Result<FilterTypeWrap, Box<dyn Error>> {
        match val.to_lowercase().as_str() {
            "catmullrom" | "cubic" => Ok(FilterTypeWrap::Inner(FilterType::CatmullRom)),
            "gaussian" => Ok(FilterTypeWrap::Inner(FilterType::Gaussian)),
            "lanczos3" => Ok(FilterTypeWrap::Inner(FilterType::Lanczos3)),
            "nearest" => Ok(FilterTypeWrap::Inner(FilterType::Nearest)),
            "triangle" => Ok(FilterTypeWrap::Inner(FilterType::Triangle)),
            fail => Err(format!("No such sampling filter: {}", fail).into()),
        }
    }
}
