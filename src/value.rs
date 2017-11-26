//! Value type and implementation for run-time memory space values.

use std::fmt;
use std::ops::Add;
use std::cmp::Ordering;

use sindra;
use sindra::value::{Coerce, Cast, Extract};

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
    /// Storage for a boolean,
    Boolean(bool),
    /// Storage for a complex number,
    Complex(f64, f64),
    /// Storage for a set
    Set(Box<ValueSet>),
    /// Indication of a value returned from a function
    Return(Box<Value>),
    /// Indication of a value resulting from a break in a loop
    Break(Box<Value>),
    /// Indication of empty / null value
    Empty
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            Value::String(ref s)  => write!(f, "{}", s),
            Value::Float(ref fl)  => write!(f, "{}", fl),
            Value::Int(ref i)     => write!(f, "{}", i),
            Value::Boolean(ref b) => write!(f, "{}", b),
            Value::Complex(ref re, ref im)
                                  => write!(f, "{}+{}i", re, im),
            Value::Set(ref s)     => write!(f, "{}", s),
            Value::Return(ref v)  => write!(f, "{}", *v),
            Value::Break(ref v)   => write!(f, "{}", *v),
            Value::Empty          => write!(f, "<null>")
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
            Literal::Boolean(b) => Value::Boolean(b),
        }
    }
}

impl<'a> From<&'a Value> for PType {
    fn from(v: &'a Value) -> PType {
        match *v {
            Value::String(_)      => PType::String,
            Value::Float(_)       => PType::Float,
            Value::Int(_)         => PType::Int,
            Value::Boolean(_)     => PType::Boolean,
            Value::Complex(_, _)  => PType::Complex,
            Value::Set(_)         => PType::Set,
            Value::Return(ref v)  => PType::from(v.as_ref()),
            Value::Break(ref v)   => PType::from(v.as_ref()),
            Value::Empty          => PType::Void,
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
            PType::Complex => {
                match self {
                    Value::Int(i) => Value::Complex(i as f64, 0.0),
                    Value::Float(f) => Value::Complex(f, 0.0),
                    _ => self
                }
            }
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

impl<'a> Add for &'a Value {
    type Output = Value;

    fn add(self, rhs: &Value) -> Value {
        match (self, rhs) {
            (&Value::Float(ref left), &Value::Float(ref right)) => {
                Value::Float(left + right)
            },
            (&Value::Int(ref left), &Value::Int(ref right)) => {
                Value::Int(left + right)
            },
            _ => panic!("unable to add value of type '{}' with a rhs of type '{}'",
                PType::from(self), PType::from(rhs))
        }
    }
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match (self, other) {
            (&Value::Float(ref left), &Value::Float(ref right)) => {
                left.partial_cmp(&right)
            },
            (&Value::Int(ref left), &Value::Int(ref right)) => {
                left.partial_cmp(&right)
            },
            _ => panic!("unable to add value of type '{}' with a rhs of type '{}'",
                PType::from(self), PType::from(other))
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
    /// Extract a boolean value from a Value.
    ///
    /// #Failures
    /// Returns an `Err` if the Value is not the right variant.
    pub fn extract_bool(&self) -> Result<bool, String> {
        match *self {
            Value::Boolean(b) => Ok(b),
            _ => Err(format!("unable to extract boolean from type {}", PType::from(self)))
        }
    }
    /// Extract a complex value from a Value.
    ///
    /// #Failures
    /// Returns an 'Err' if the Value is not the right variant.
    pub fn extract_complex(&self) -> Result<(f64, f64), String> {
        match *self {
            Value::Complex(re, im) => Ok((re, im)),
            _ => Err(format!("unable to extract complex number from type {}", PType::from(self)))
        }
    }

    /// Returns true if the associated type for this value is the same type as the associated
    /// type of the other value. Returns false otherwise.
    pub fn has_same_type(&self, other: &Value) -> bool {
        PType::from(self) == PType::from(other)
    }
}

impl Extract<u64> for Value {
    fn extract(&self) -> Result<u64, String> {
        match *self {
            Value::Int(i) => {
                Ok(i as u64)
            },
            _ => Err(format!("unable to extract unsigned int from type {}", PType::from(self)))
        }
    }
}
impl Extract<i64> for Value {
    fn extract(&self) -> Result<i64, String> {
        match *self {
            Value::Int(i) => {
                Ok(i)
            },
            _ => Err(format!("unable to extract int from type {}", PType::from(self)))
        }
    }
}
impl Extract<String> for Value {
    fn extract(&self) -> Result<String, String> {
        match *self {
            Value::String(ref s) => {
                Ok(s.clone())
            },
            _ => Err(format!("unable to extract string from type {}", PType::from(self)))
        }
    }
}



/// Value type for sets
#[derive(Debug, Clone, PartialEq)]
pub enum ValueSet {
    /// A set represented by an interval specification
    Interval(SetInterval)
}

impl ValueSet {
    /// Return an iterator over the elements of the set.
    ///
    /// #Failures
    /// Returns an `Err` if the set definition is inconsistent (e.g. has inconsistent types in
    /// the type definition)
    pub fn iter<'a>(&'a self) -> Result<SetIter<'a>, String> {
        let iter = match *self {
            ValueSet::Interval(ref interval) => {
                // verify interval types
                if !interval.start.has_same_type(&interval.end) ||
                        !interval.start.has_same_type(&interval.step) {
                    return Err("interval must have same value type for start, end, and step \
                        to be iterated".to_string())
                }
                SetIter::Interval {
                    set: interval,
                    prev: None
                }
            }
        };
        Ok(iter)
    }
}

impl fmt::Display for ValueSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            ValueSet::Interval(ref set) => write!(f, "{}", set),
        }
    }
}

/// Set specification using an interval specification.
#[derive(Debug, Clone, PartialEq)]
pub struct SetInterval {
    /// Starting value for the interval (inclusive).
    pub start: Value,
    /// Ending value for the interval (exclusive).
    pub end: Value,
    /// Whether the ending value for the interval is inclusive or exclusive.
    pub end_inclusive: bool,
    /// Step level for the interval.
    pub step: Value,
}

impl fmt::Display for SetInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        write!(f, "{}..{}..{}", self.start, self.step, self.end)
    }
}

/// Iterator over a set.
pub enum SetIter<'a> {
    /// Iterator over a set represented by an interval.
    Interval {
        /// Reference to the underlying set interval specification.
        set: &'a SetInterval,
        /// Previous value returned by iterator
        prev: Option<Value>,
    }
}
impl<'a> Iterator for SetIter<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        match *self {
            SetIter::Interval { ref set, ref mut prev } => {
                let next = match *prev {
                    Some(ref prev) => {
                        prev + &set.step
                    },
                    None => {
                        set.start.clone()
                    }
                };
                // terminate only when completely past the end of the set when the set is inclusive,
                // terminate when at the end or beyond when the set is exclusive
                let terminate = if set.end_inclusive {
                    next > set.end
                } else {
                    next >= set.end
                };
                if terminate {
                    None
                } else {
                    *prev = Some(next.clone());
                    Some(next)
                }

            }
        }
    }
}
