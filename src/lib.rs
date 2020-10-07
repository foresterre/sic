#![deny(clippy::all)]

#[macro_use]
extern crate strum_macros;

#[cfg(test)]
#[macro_use]
extern crate parameterized;

pub mod cli;
pub mod combinators;
