//! Standard library for piske.

#![warn(missing_docs)]

extern crate image as img;

mod image;
mod extrema;
pub mod stdlib;
mod environment;
pub use self::environment::*;

pub mod step_range;
