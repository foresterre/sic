#![deny(clippy::all)]
#![allow(clippy::upper_case_acronyms)]

pub mod export;
pub mod import;

pub mod conversion;
pub mod errors;
pub mod format;

pub trait WriteSeek: std::io::Write + std::io::Seek {}
