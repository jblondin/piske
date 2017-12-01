//! Tools to call standard library functions from the interpreter.

#[macro_use] mod macros;
mod extfunc;
pub use self::extfunc::{ExtFuncIdent, StdFuncTable};
