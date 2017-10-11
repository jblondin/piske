//! Implementation of run-time operator traits.

use sindra::operator::{BinaryOperator, UnaryOperator};
use value::Value;
use ast::{PType, InfixOp, PrefixOp, PostfixOp};

impl BinaryOperator<PType, Value> for InfixOp {
    fn op(&self, ty: PType, left: &Value, right: &Value) -> Result<Value, String> {
        match ty {
            PType::Float => {
                let left = left.extract_float()?;
                let right = right.extract_float()?;
                match *self {
                    InfixOp::Add => { Ok(Value::Float(left + right)) },
                    InfixOp::Subtract => { Ok(Value::Float(left - right)) },
                    InfixOp::Divide => { Ok(Value::Float(left / right)) },
                    InfixOp::Multiply => { Ok(Value::Float(left * right)) },
                    InfixOp::Power => { Ok(Value::Float(left.powf(right))) }
                }
            },
            PType::Int => {
                let left = left.extract_int()?;
                let right = right.extract_int()?;
                match *self {
                    InfixOp::Add => { Ok(Value::Int(left + right)) },
                    InfixOp::Subtract => { Ok(Value::Int(left - right)) },
                    InfixOp::Divide => { Ok(Value::Int(left / right)) },
                    InfixOp::Multiply => { Ok(Value::Int(left * right)) },
                    InfixOp::Power => {
                        if right >= 0 {
                            Ok(Value::Int(left.pow(right as u32)))
                        } else {
                            Err("attempt to raise integer value to negative power".to_string())
                        }
                    }
                }
            },
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
                }
            },
            PType::Int => {
                let operand = operand.extract_int()?;
                match *self {
                    PostfixOp::Conjugate => Ok(Value::Float(1.0 / (operand as f64))),
                }
            }
            _ => Err(format!("prefix operators invalid for type {}", ty))
        }
    }
}
