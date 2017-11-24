//! Implementation of run-time operator traits.

use sindra::operator::{BinaryOperator, UnaryOperator};

use value::Value;
use ast::{InfixOp, PrefixOp, PostfixOp, CompareOp};
use PType;

#[inline(always)]
fn compare_primitives<T: PartialOrd + PartialEq>(op: CompareOp, left: T, right: T) -> bool {
    match op {
        CompareOp::LessThan => left < right,
        CompareOp::LessThanEqual => left <= right,
        CompareOp::GreaterThan => left > right,
        CompareOp::GreaterThanEqual => left >= right,
        CompareOp::Equal => left == right,
        CompareOp::NotEqual => left != right,
    }
}

#[inline(always)]
fn compare(op: CompareOp, left: &Value, right: &Value) -> Result<Value, String> {
    match (left, right) {
        (&Value::Int(l), &Value::Int(r)) => {
            Ok(Value::Boolean(compare_primitives(op, l, r)))
        }
        (&Value::Float(l), &Value::Float(r)) => {
            Ok(Value::Boolean(compare_primitives(op, l, r)))
        }
        (&Value::Int(l), &Value::Float(r)) => {
            Ok(Value::Boolean(compare_primitives(op, l as f64, r)))
        }
        (&Value::Float(l), &Value::Int(r)) => {
            Ok(Value::Boolean(compare_primitives(op, l, r as f64)))
        }
        _ => {
            Err(format!("unable to compare values of type '{}' and '{}'", PType::from(left),
                PType::from(right)))
        }
    }
}

impl BinaryOperator<PType, Value> for InfixOp {
    fn op(&self, ty: PType, left: &Value, right: &Value) -> Result<Value, String> {
        match ty {
            PType::Float => {
                let left = left.extract_float()?;
                let right = right.extract_float()?;
                match *self {
                    InfixOp::Add => Ok(Value::Float(left + right)),
                    InfixOp::Subtract => Ok(Value::Float(left - right)),
                    InfixOp::Divide => Ok(Value::Float(left / right)),
                    InfixOp::Multiply => Ok(Value::Float(left * right)),
                    InfixOp::Power => Ok(Value::Float(left.powf(right))),
                    InfixOp::Comparison(_) => Err(
                        "comparisons cannot be interpreted as integers".to_string())
                }
            },
            PType::Int => {
                let left = left.extract_int()?;
                let right = right.extract_int()?;
                match *self {
                    InfixOp::Add => Ok(Value::Int(left + right)),
                    InfixOp::Subtract => Ok(Value::Int(left - right)),
                    InfixOp::Divide => Ok(Value::Int(left / right)),
                    InfixOp::Multiply => Ok(Value::Int(left * right)),
                    InfixOp::Power => {
                        if right >= 0 {
                            Ok(Value::Int(left.pow(right as u32)))
                        } else {
                            Err("attempt to raise integer value to negative power".to_string())
                        }
                    },
                    InfixOp::Comparison(_) => Err(
                        "comparisons cannot be interpreted as floating point".to_string())
                }
            },
            PType::Boolean => {
                match *self {
                    InfixOp::Comparison(op) => {
                        compare(op, left, right)
                    },
                    _ => Err(format!("unable to interpret type '{}' as boolean", ty))
                }
            }
            PType::Complex => {
                let (a, b) = left.extract_complex()?;
                let (c, d) = right.extract_complex()?;
                match *self {
                    InfixOp::Add => Ok(Value::Complex(a + c, b + d)),
                    InfixOp::Subtract => Ok(Value::Complex(a - c, b - d)),
                    InfixOp::Multiply => Ok(Value::Complex(
                        a * c - b * d,
                        b * c + a * d
                    )),
                    InfixOp::Divide => {
                        let denom = c * c + d * d;
                        Ok(Value::Complex(
                            (a * c + b * d) / denom,
                            (b * c - a * d) / denom
                        ))
                    },
                    InfixOp::Power => {
                        Err("exponentiation of complex numbers currently unimplemented".to_string())
                    },
                    InfixOp::Comparison(_) => Err(
                        "comparisons cannot be interpreted as complex".to_string())
                }
            }
            _ => Err(format!("infix operators invalid for type {}", ty))
        }
    }
}

impl UnaryOperator<PType, Value> for PrefixOp {
    fn op(&self, ty: PType, operand: &Value) -> Result<Value, String> {
        match ty {
            PType::Float => {
                let operand = operand.extract_float()?;
                match *self {
                    PrefixOp::UnaryMinus => Ok(Value::Float(-operand)),
                    PrefixOp::UnaryPlus => Ok(Value::Float(operand))
                }
            },
            PType::Int => {
                let operand = operand.extract_int()?;
                match *self {
                    PrefixOp::UnaryMinus => Ok(Value::Int(-operand)),
                    PrefixOp::UnaryPlus => Ok(Value::Int(operand)),
                }
            }
            _ => Err(format!("prefix operators invalid for type {}", ty))
        }
    }
}

impl UnaryOperator<PType, Value> for PostfixOp {
    fn op(&self, ty: PType, operand: &Value) -> Result<Value, String> {
        match ty {
            PType::Float => {
                let operand = operand.extract_float()?;
                match *self {
                    PostfixOp::Conjugate => Ok(Value::Float(1.0 / operand)),
                    PostfixOp::Imaginary => Err(format!("invalid destination type '{}' for \
                        imaginary operation", ty)),
                }
            },
            PType::Int => {
                let operand = operand.extract_int()?;
                match *self {
                    PostfixOp::Conjugate => Ok(Value::Float(1.0 / (operand as f64))),
                    PostfixOp::Imaginary => Err(format!("invalid destination type '{}' for \
                        imaginary operation", ty)),
                }
            },
            PType::Complex => {
                match *self {
                    PostfixOp::Conjugate => {
                        match *operand {
                            Value::Complex(re, im) => {
                                Ok(Value::Complex(re, -im))
                            },
                            _ => {
                                Err(format!("invalid value '{}' for conjugation with destination \
                                    type '{}'", operand, ty))
                            }
                        }
                    },
                    PostfixOp::Imaginary => {
                        match *operand {
                            Value::Float(f) => Ok(Value::Complex(0.0, f)),
                            Value::Int(i) => Ok(Value::Complex(0.0, i as f64)),
                            _ => {
                                Err(format!("unable to convert value of type '{}' to imaginary \
                                    number", PType::from(operand)))
                            }
                        }
                    },

                }
            }
            _ => Err(format!("postfix operator {} invalid for type {} on operand {}", self, ty,
                    operand))
        }
    }
}
