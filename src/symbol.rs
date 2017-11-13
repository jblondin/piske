//! Symbol type

use std::fmt;

use sindra::Identifier;
use sindra::node::PNode;

use ast::{Block, Parameter};
use PType;

/// Symbol object (for use in symbol tables).
#[derive(Clone, Debug, PartialEq)]
pub enum Symbol {
    /// Built-in types
    BuiltinType {
        /// Name of the built-in type (a keyword)
        name: Identifier,
        /// PType associated with the name
        ty: PType,
    },
    /// Variables
    Variable {
        /// Name of the variable
        name: Identifier,
        /// Type of this variable (an Option, since this type is not known at all times of the
        /// computation)
        ty: Option<PType>,
    },
    /// Functions
    Function {
        /// Name of the function
        name: Identifier,
        /// Return type of the function (an Option since this type is not known at all times of
        /// the computation)
        ret_ty: Option<PType>,
        /// Function Body
        body: PNode<Block>,
        /// Function parameters,
        params: Vec<PNode<Parameter>>,
    }
}

impl Symbol {
    /// Create a builtin type Symbol, with specified type
    pub fn builtin(name: Identifier, ty: PType) -> Symbol {
        Symbol::BuiltinType {
            name: name,
            ty: ty
        }
    }
    /// Create a function Symbol, with specified type
    pub fn function(name: Identifier, ty: Option<PType>, body: PNode<Block>,
            params: Vec<PNode<Parameter>>) -> Symbol {
        Symbol::Function {
            name: name,
            ret_ty: ty,
            body: body,
            params: params,
        }
    }
    /// Create a variable Symbol
    pub fn variable(name: Identifier, ty: Option<PType>) -> Symbol {
        Symbol::Variable {
            name: name,
            ty: ty
        }
    }
}


impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        let (kind, name, ty) = match *self {
            Symbol::Variable { ref name,  ref ty } => ("var", name, ty.clone()),
            Symbol::Function { ref name, ref ret_ty, .. } => ("fn", name, ret_ty.clone()),
            Symbol::BuiltinType { ref name, ref ty } => ("bi", name, Some(ty.clone())),
        };
        match ty {
            Some(ty) => write!(f, "{} {}: {}", kind, name, ty),
            None => write!(f, "{} {}", kind, name)
        }
    }
}
