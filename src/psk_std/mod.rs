//! Standard library functions for piske.

#[macro_use] mod macros;
mod image;
mod extrema;
mod extfunc;
pub use self::extfunc::{ExtFuncIdent, StdFuncTable};
pub mod stdlib;
mod environment;
pub use self::environment::*;

pub mod step_range;
