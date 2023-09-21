//! The primary definition of an animated image, for the `sic` project.

use crate::errors::SicCoreError;
use image::{DynamicImage, Frames};
use std::fmt::{Debug, Formatter};

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
    pub fn try_into_static_image(mut self, index: usize) -> Result<DynamicImage, SicCoreError> {
        let len = self.frames.len();
        if index < len {
            Ok(DynamicImage::ImageRgba8(
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

impl<'frames> TryFrom<Frames<'frames>> for AnimatedImage {
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
