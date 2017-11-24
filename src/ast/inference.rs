//! Implementation of inference and promotion traits for abstract syntax tree.

use sindra::inference::{InferTypesBinary, BinaryOpTypes, InferTypesUnary, UnaryOpTypes,
    InferPromotion};
use ast::*;
use PType;

lazy_static! {
    /// Result type definition for all arithmetic infix operations. `Some(...)` indicates that the
    /// operation is possible and has the given result type, `None` indicates that the operation
    /// is invalid on the supplied types.
    pub static ref ARITH_RESULT_TABLE: [[Option<PType>; 7]; 7] = [
        // String + [String, Float, Int, Boolean, Complex, Set, Void]
        [None, None, None, None, None, None, None],
        // Float + [String, Float, Int, Boolean, Complex, Set, Void]
        [None, Some(PType::Float), Some(PType::Float), None, Some(PType::Complex), None, None],
        // Int + [String, Float, Int, Boolean, Complex, Set, Void]
        [None, Some(PType::Float), Some(PType::Int), None, Some(PType::Complex), None, None],
        // Boolean + [String, Float, Int, Boolean, Complex, Set, Void]
        [None, None, None, None, None, None, None],
        // Complex + [String, Float, Int, Boolean, Complex, Set, Void]
        [None, Some(PType::Complex), Some(PType::Complex), None, Some(PType::Complex), None, None],
        // Set + [String, Float, Int, Boolean, Complex, Set, Void]
        [None, None, None, None, None, None, None],
        // Void + [String, Float, Int, Boolean, Complex, Set, Void]
        [None, None, None, None, None, None, None],
    ];
}

lazy_static! {
    /// Table of comparable (via comparison operators; e.g. <, <=, >, >=) types. 'true' indicates
    /// the types are comparable, 'false' indicates they are not.
    pub static ref COMPARABLE: [[bool; 7]; 7] = [
        // String == [String, Float, Int, Boolean, Complex, Set, Void]
        [true, false, false, false, false, false, false],
        // Float == [String, Float, Int, Boolean, Complex, Set, Void]
        [false, true, true, false, true, false, false],
        // Int == [String, Float, Int, Boolean, Complex, Set, Void]
        [false, true, true, false, true, false, false],
        // Boolean == [String, Float, Int, Boolean, Complex, Set, Void]
        [false, false, false, true, false, false, false],
        // Complex == [String, Float, Int, Boolean, Complex, Set, Void]
        [false, true, true, false, true, false, false],
        // Set == [String, Float, Int, Boolean, Complex, Set, Void]
        [false, false, false, false, false, true, false],
        // Void == [String, Float, Int, Boolean, Complex, Set, Void]
        [false, false, false, false, false, false, false],
    ];
}

impl InferTypesBinary for InfixOp {
    type Operand = PType;

    fn infer_types(&self, left: PType, right: PType) -> Option<BinaryOpTypes<PType>> {
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
            },
            InfixOp::Comparison(_) => {
                if COMPARABLE[left as usize][right as usize] {
                    Some(PType::Boolean)
                } else {
                    None
                }
            }
        }.map(|t| BinaryOpTypes { result: t, left: t, right: t })
    }
}

impl InferTypesUnary for PrefixOp {
    type Operand = PType;

    fn infer_types(&self, operand: PType) -> Option<UnaryOpTypes<PType>> {
        match operand {
            PType::Float | PType::Int | PType::Complex => Some(operand),
            _ => None,
        }.map(|t| UnaryOpTypes { result: t, operand: t })
    }
}

impl InferTypesUnary for PostfixOp {
    type Operand = PType;

    fn infer_types(&self, operand: PType) -> Option<UnaryOpTypes<PType>> {
        match *self {
            PostfixOp::Conjugate => {
                match operand {
                    PType::Float | PType::Int => Some(PType::Float),
                    PType::Complex => Some(PType::Complex),
                    _ => None,
                }.map(|t| UnaryOpTypes { result: t, operand: t })
            },
            PostfixOp::Imaginary => {
                match operand {
                    PType::Float | PType::Int => Some(UnaryOpTypes {
                        result: PType::Complex,
                        operand: operand
                    }),
                    _ => None
                }
            }
        }
    }
}

impl InferPromotion for PType {
    fn infer_promotion(&self, dest_ty: PType) -> Option<PType> {
        match (self, dest_ty) {
            (&PType::Int, PType::Float) => {
                Some(PType::Float)
            },
            (&PType::Int, PType::Complex) | (&PType::Float, PType::Complex) => {
                Some(PType::Complex)
            },
            _ => None
        }
    }
}
