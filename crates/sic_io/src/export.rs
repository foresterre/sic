use std::path::Path;

use sic_core::{image, SicImage};

use crate::conversion::{AutomaticColorTypeAdjustment, ConversionWriter, RepeatAnimation};
use crate::errors::SicIoError;
use crate::WriteSeek;

pub fn export<W: WriteSeek>(
    image: &SicImage,
    writer: &mut W,
    format: image::ImageOutputFormat,
    export_settings: ExportSettings,
) -> Result<(), SicIoError> {
    let conv = ConversionWriter::new(image);
    conv.write_all(writer, format, &export_settings)
}

#[derive(Debug, Default)]
pub struct ExportSettings {
    pub adjust_color_type: AutomaticColorTypeAdjustment,
    pub gif_repeat: RepeatAnimation,
}

pub struct EmptyPath;

impl AsRef<Path> for EmptyPath {
    fn as_ref(&self) -> &Path {
        Path::new("")
    }
}
