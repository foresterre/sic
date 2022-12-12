#![deny(clippy::all)]

//! This crate contains a re-export of the image crate and a few fundamental data types which all
//! or almost all sic components interact with. The re-export of the image crate makes sure all
//! components depend on the same version of the image crate, which is required for binary
//! compatibility.

/// The re-export of image ensures all sic components use the same version.
pub use image;

#[cfg(feature = "imageproc-ops")]
pub use {imageproc, rusttype};

use crate::errors::SicCoreError;

use image::{DynamicImage, Frames};
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};

pub mod errors;

/// The fundamental image data structure in `sic`.
/// An image can either be animated, in which case it consists of a collection of `image::Frame` frames,
/// or static, in which case it's represented as an `image::DynamicImage`.
#[derive(Clone, Debug)]
pub enum SicImage {
    Animated(AnimatedImage),
    Static(image::DynamicImage),
}

// Should not be used outside of tests, as it doesn't support animated images
#[doc(hidden)]
impl AsRef<image::DynamicImage> for SicImage {
    fn as_ref(&self) -> &DynamicImage {
        match self {
            Self::Animated(_) => unimplemented!(),
            Self::Static(image) => image,
        }
    }
}

impl From<image::DynamicImage> for SicImage {
    fn from(item: DynamicImage) -> Self {
        Self::Static(item)
    }
}

impl TryFrom<SicImage> for image::DynamicImage {
    type Error = SicCoreError;

    fn try_from(value: SicImage) -> Result<Self, Self::Error> {
        match value {
            SicImage::Static(image) => Ok(image),
            _ => Err(SicCoreError::RequiresStaticImage),
        }
    }
}

#[derive(Clone)]
pub struct AnimatedImage {
    frames: Vec<image::Frame>,
}

impl AnimatedImage {
    /// Consume a collection of frames to produce an `AnimatedImage`
    pub fn from_frames(frames: Vec<image::Frame>) -> Self {
        Self { frames }
    }

    /// Returns the selected frame from the animated image as static image
    pub fn try_into_static_image(
        mut self,
        index: usize,
    ) -> Result<image::DynamicImage, SicCoreError> {
        let len = self.frames.len();
        if index < len {
            Ok(image::DynamicImage::ImageRgba8(
                self.frames.remove(index).into_buffer(),
            ))
        } else {
            Err(SicCoreError::InvalidFrameIndex { index, len })
        }
    }

    /// Returns a slice of image Frames
    pub fn frames(&self) -> &[image::Frame] {
        &self.frames
    }

    /// Returns a mutable slice of image frames
    pub fn frames_mut(&mut self) -> &mut [image::Frame] {
        &mut self.frames
    }

    /// Collects and returns an owned collection of image frames
    pub fn collect_frames(&self) -> Vec<image::Frame> {
        self.frames.clone()
    }
}

impl<'frames> TryFrom<image::Frames<'frames>> for AnimatedImage {
    type Error = SicCoreError;

    fn try_from(item: Frames<'frames>) -> Result<Self, Self::Error> {
        let frames = item.collect_frames().map_err(SicCoreError::ImageError)?;

        Ok(Self { frames })
    }
}

impl Debug for AnimatedImage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "AnimatedImage(frame_count={})",
            self.frames.len()
        ))
    }
}
