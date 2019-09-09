use std::env::args;
use std::io::stdin;
use std::io::Read;
use std::path::Path;

use crate::conversion::{AutomaticColorTypeAdjustment, ConversionWriter};

// importing
pub mod import;

// exporting
pub mod export;

pub mod conversion;
pub mod encoding_format;
