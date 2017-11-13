//! Implementation of inference and promotion traits for abstract syntax tree.

use sindra::inference::{InferResultBinary, InferResultUnary, InferPromotion};
use ast::*;
use PType;

lazy_static! {
    /// Result type definition for all arithmetic infix operations. `Some(...)` indicates that the
    /// operation is possible and has the given result type, `None` indicates that the operation
    /// is invalid on the supplied types.
    pub static ref ARITH_RESULT_TABLE: [[Option<PType>; 3]; 3] = [
    // Right:  String,              Float,                Int    // Left:
              [  None,               None,               None ], // String
              [  None, Some(PType::Float), Some(PType::Float) ], // Float
              [  None, Some(PType::Float),   Some(PType::Int) ]  // Int
    ];
}

impl InferResultBinary for InfixOp {
    type Operand = PType;

    fn infer_result_type(&self, left: PType, right: PType) -> Option<PType> {
        match *self {
            InfixOp::Subtract | InfixOp::Multiply | InfixOp::Divide | InfixOp::Add => {
                ARITH_RESULT_TABLE[left as usize][right as usize]
            },
            InfixOp::Power => {
                if left == PType::String || right == PType::String {
                    None
                } else {
                    Some(PType::Float)
                }
            }

        }
    }
}

impl InferResultUnary for PrefixOp {
    type Operand = PType;

    fn infer_result_type(&self, operand: PType) -> Option<PType> {
        match operand {
            PType::Float | PType::Int => Some(operand),
            _ => None,
        }
    }
}

impl InferResultUnary for PostfixOp {
    type Operand = PType;

    fn infer_result_type(&self, operand: PType) -> Option<PType> {
        match operand {
            PType::Float | PType::Int => Some(PType::Float),
            _ => None,
        }
    }
}

lazy_static! {
    /// Promotion requirements for possible target types. `Some(...)` indicates that promotion is
    /// required for particular type, `None` indicates that either no promotion is required or that
    /// promotion is impossible.
    pub static ref PROMOTE_TABLE: [[Option<PType>; 3]; 3] = [
    // Dest:   String,              Float,                Int    // Src:
              [  None,               None,               None ], // String
              [  None,               None,               None ], // Float
              [  None, Some(PType::Float),               None ]  // Int
    ];
}

impl InferPromotion for PType {
    fn infer_promotion(&self, dest_ty: PType) -> Option<PType> {
        PROMOTE_TABLE[*self as usize][dest_ty as usize]
    }
}
