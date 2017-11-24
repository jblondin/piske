//! The valid types of the piske programming language.

use sindra::Type;
use std::fmt;

/// The types available in the piske programming language. Implements the sindra `Type` trait.
#[derive(Copy, Debug, Clone, Hash, PartialEq)]
pub enum PType {
    /// Built-in string type
    String,
    /// Floating-point numbers
    Float,
    /// Integers (signed)
    Int,
    /// Boolean (true / false)
    Boolean,
    /// Complex
    Complex,
    /// Set (collection)
    Set,
    /// Empty type
    Void
}
impl Type for PType {
    fn name(&self) -> &str {
        match *self {
            PType::String => "string",
            PType::Float => "float",
            PType::Int => "int",
            PType::Boolean => "bool",
            PType::Complex => "complex",
            PType::Set => "set",
            PType::Void => "void",
        }
    }
}

impl fmt::Display for PType {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        write!(f, "{}", self.name())
    }
}
