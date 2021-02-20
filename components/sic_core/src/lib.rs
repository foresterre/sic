#![deny(clippy::all)]

//! This crate contains a re-export of the image crate and a few fundamental data types which all
//! or almost all sic components interact with. The re-export of the image crate makes sure all
//! components depend on the same version of the image crate, which is required for binary
//! compatibility.

/// The re-export of image ensures all sic components use the same version.
pub use image;

use crate::errors::SicCoreError;

use image::{DynamicImage, Frames};
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};

pub mod errors;

#[derive(Clone, Debug)]
pub enum SicImage {
    Animated(AnimatedImage),
    Static(image::DynamicImage),
}

impl SicImage {
    // TODO: remove porting function
    pub fn collect_static_image(&self) -> image::DynamicImage {
        match self {
            Self::Animated(_) => unimplemented!(),
            Self::Static(image) => image.clone(),
        }
    }

    // TODO: remove porting function
    pub fn inner(&self) -> &image::DynamicImage {
        match self {
            Self::Animated(_) => unimplemented!(),
            Self::Static(image) => image,
        }
    }
    // TODO: remove porting function
    pub fn inner_mut(&mut self) -> &mut image::DynamicImage {
        match self {
            Self::Animated(_) => unimplemented!(),
            Self::Static(image) => image,
        }
    }
}

// TODO: remove porting function
impl AsRef<image::DynamicImage> for SicImage {
    fn as_ref(&self) -> &DynamicImage {
        self.inner()
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
    pub fn frames(&self) -> &[image::Frame] {
        &self.frames
    }

    pub fn frames_mut(&mut self) -> &mut [image::Frame] {
        &mut self.frames
    }

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
