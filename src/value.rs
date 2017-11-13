//! Value type and implementation for run-time memory space values.

use std::fmt;

use sindra;
use sindra::value::{Coerce, Cast};

use ast::Literal;
use PType;

/// Value type for run-time memory values.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Storage for string value
    String(String),
    /// Storage for floating point number value
    Float(f64),
    /// Storage for integer number value
    Int(i64),
    /// Indication of empty / null value
    Empty
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            Value::String(ref s) => write!(f, "{}", s),
            Value::Float(ref fl) => write!(f, "{}", fl),
            Value::Int(ref i)    => write!(f, "{}", i),
            Value::Empty         => write!(f, "<null>")
        }
    }
}
impl sindra::Value for Value {}

impl From<Literal> for Value {
    fn from(lit: Literal) -> Value {
        match lit {
            Literal::String(s) => Value::String(s),
            Literal::Float(f) => Value::Float(f),
            Literal::Int(i) => Value::Int(i),
        }
    }
}

impl<'a> From<&'a Value> for PType {
    fn from(v: &'a Value) -> PType {
        match *v {
            Value::String(_) => PType::String,
            Value::Float(_) => PType::Float,
            Value::Int(_) => PType::Int,
            Value::Empty  => PType::Void,
        }
    }
}

impl Cast<PType> for Value {
    fn cast(self, dest_ty: PType) -> Value {
        match dest_ty {
            PType::Float => {
                match self {
                    Value::Int(i) => Value::Float(i as f64),
                    _ => self
                }
            },
            _ => self
        }
    }
}

impl Coerce<PType> for Value {
    fn coerce(self, dest_ty: Option<PType>) -> Value {
        match dest_ty {
            Some(dest) => {
                self.cast(dest)
            },
            None => self
        }
    }
}

impl Value {
    /// Extract an f64 value from a Value.
    ///
    /// #Failures
    /// Returns an `Err` if the Value is not the right variant.
    pub fn extract_float(&self) -> Result<f64, String> {
        match *self {
            Value::Float(f) => Ok(f),
            _ => Err(format!("unable to extract float from type {}", PType::from(self)))
        }
    }
    /// Extract an i64 value from a Value.
    ///
    /// #Failures
    /// Returns an `Err` if the Value is not the right variant.
    pub fn extract_int(&self) -> Result<i64, String> {
        match *self {
            Value::Int(i) => Ok(i),
            _ => Err(format!("unable to extract int from type {}", PType::from(self)))
        }
    }
}
