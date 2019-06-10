use std::error::Error;
use std::fmt::{Debug, Formatter};

// Wrapper for image::FilterType.
// Does only exists, because image::FilterType does not implement PartialEq and Debug.
pub enum FilterTypeWrap {
    Inner(image::FilterType),
}

impl PartialEq<FilterTypeWrap> for FilterTypeWrap {
    fn eq(&self, other: &FilterTypeWrap) -> bool {
        match (self, other) {
            (
                FilterTypeWrap::Inner(image::FilterType::CatmullRom),
                FilterTypeWrap::Inner(image::FilterType::CatmullRom),
            ) => true,
            (
                FilterTypeWrap::Inner(image::FilterType::Gaussian),
                FilterTypeWrap::Inner(image::FilterType::Gaussian),
            ) => true,
            (
                FilterTypeWrap::Inner(image::FilterType::Lanczos3),
                FilterTypeWrap::Inner(image::FilterType::Lanczos3),
            ) => true,
            (
                FilterTypeWrap::Inner(image::FilterType::Nearest),
                FilterTypeWrap::Inner(image::FilterType::Nearest),
            ) => true,
            (
                FilterTypeWrap::Inner(image::FilterType::Triangle),
                FilterTypeWrap::Inner(image::FilterType::Triangle),
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
            FilterTypeWrap::Inner(image::FilterType::CatmullRom) => {
                "image::FilterType::CatmullRom (Wrapper)"
            }
            FilterTypeWrap::Inner(image::FilterType::Gaussian) => {
                "image::FilterType::Gaussian (Wrapper)"
            }
            FilterTypeWrap::Inner(image::FilterType::Lanczos3) => {
                "image::FilterType::Lanczos3 (Wrapper)"
            }
            FilterTypeWrap::Inner(image::FilterType::Nearest) => {
                "image::FilterType::Nearest (Wrapper)"
            }
            FilterTypeWrap::Inner(image::FilterType::Triangle) => {
                "image::FilterType::Triangle (Wrapper)"
            }
        };

        f.write_str(msg)
    }
}

impl From<FilterTypeWrap> for image::FilterType {
    fn from(wrap: FilterTypeWrap) -> Self {
        match wrap {
            FilterTypeWrap::Inner(w) => w,
        }
    }
}

impl FilterTypeWrap {
    pub fn try_from_str(val: &str) -> Result<FilterTypeWrap, Box<dyn Error>> {
        match val.to_lowercase().as_str() {
            "catmullrom" | "cubic" => Ok(FilterTypeWrap::Inner(image::FilterType::CatmullRom)),
            "gaussian" => Ok(FilterTypeWrap::Inner(image::FilterType::Gaussian)),
            "lanczos3" => Ok(FilterTypeWrap::Inner(image::FilterType::Lanczos3)),
            "nearest" => Ok(FilterTypeWrap::Inner(image::FilterType::Nearest)),
            "triangle" => Ok(FilterTypeWrap::Inner(image::FilterType::Triangle)),
            fail => Err(format!("No such sampling filter: {}", fail).into()),
        }
    }
}
