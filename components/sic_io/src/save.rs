use std::io::Write;
use std::path::Path;

use sic_core::image;

use crate::conversion::{AutomaticColorTypeAdjustment, ConversionWriter};
use crate::errors::SicIoError;

pub fn export<W: Write>(
    image: &image::DynamicImage,
    writer: &mut W,
    format: image::ImageOutputFormat,
    export_settings: ExportSettings,
) -> Result<(), SicIoError> {
    let conv = ConversionWriter::new(image);
    conv.write(writer, format, export_settings.adjust_color_type)
}

#[derive(Debug)]
pub struct ExportSettings {
    pub adjust_color_type: AutomaticColorTypeAdjustment,
}

pub struct EmptyPath;

impl AsRef<Path> for EmptyPath {
    fn as_ref(&self) -> &Path {
        Path::new("")
    }
}
