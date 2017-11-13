//! Implementation of Display trait for abstract syntax tree.

use std::fmt::{self, Write};

use ast::*;

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        write!(f, "{}", self.0.borrow().item)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        write!(f, "\n{{\n")?;
        for statement in &self.0 {
            write!(f, "{}\n", statement.borrow().item)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            Statement::Expression(ref expr) => write!(f, "expr:{}", expr.borrow().item),
            Statement::Declare(ref ident, ref expr) => write!(f, "decl({}->{})",
                ident.borrow().item, expr.borrow().item),
            Statement::Assign(ref ident, ref expr) => write!(f, "assign({}->{})",
                ident.borrow().item, expr.borrow().item),
            Statement::FnDefine(FunctionDef { ref name, ref body, ref params, ref ret_type }) => {
            // Statement::FnDefine { ref name, ref ret_type, ref params, ref body } => {
                let mut pl = String::new();
                let mut first = true;
                for expr in params {
                    if first {
                        first = false;
                    } else {
                        write!(&mut pl, ",")?;
                    }
                    write!(&mut pl, "{}", expr.borrow().item)?;
                }
                write!(f, "def({}({}) -> {}) {}", name.borrow().item, pl, ret_type.borrow().item,
                    body.borrow().item)
            }
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        write!(f, "{}: {}", self.name.borrow().item, self.ty.borrow().item)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            Expression::Literal(ref lit) => write!(f, "lit:{}", lit.borrow().item),
            Expression::Identifier(ref ident) => write!(f, "ident:{}", ident.borrow().item),
            Expression::Infix { ref op, ref left, ref right } =>
                write!(f, "infix:{}{}{}", left.borrow().item, op, right.borrow().item),
            Expression::Prefix { ref op, ref right } =>
                write!(f, "prefix:{}{}", op, right.borrow().item),
            Expression::Postfix { ref op, ref left } =>
                write!(f, "postfix:{}{}", left.borrow().item, op),
            Expression::Block(ref block) =>
                write!(f, "block:{}", block.borrow().item),
            Expression::FnCall { name: ref ident, ref args } => {
                let mut pl = String::new();
                let mut first = true;
                for expr in args {
                    if first {
                        first = false;
                    } else {
                        write!(&mut pl, ",")?;
                    }
                    write!(&mut pl, "{}", expr.borrow().item)?;
                }
                write!(f, "fn{{{}}}({})", ident.borrow().item, pl)
            }
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            Literal::String(ref s) => write!(f, "\"{}\"", s),
            Literal::Float(ref fl) => write!(f, "{}", fl),
            Literal::Int(ref i) => write!(f, "{}", *i),
        }
    }
}

impl fmt::Display for PrefixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            PrefixOp::UnaryMinus => write!(f, "-"),
            PrefixOp::UnaryPlus => write!(f, "+"),
        }
    }
}

impl fmt::Display for InfixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            InfixOp::Add => write!(f, "+"),
            InfixOp::Subtract => write!(f, "-"),
            InfixOp::Multiply => write!(f, "*"),
            InfixOp::Divide => write!(f, "/"),
            InfixOp::Power => write!(f, "^"),
        }
    }
}

impl fmt::Display for PostfixOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            PostfixOp::Conjugate => write!(f, "`"),
        }
    }
}

