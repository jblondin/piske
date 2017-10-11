//! Implementation of Display trait for abstract syntax tree.

use std::fmt;
use ast::*;

impl<A: Default> fmt::Display for Program<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        write!(f, "{}", self.0.item())
    }
}

impl<A: Default> fmt::Display for Block<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        write!(f, "\n{{\n")?;
        for statement in &self.0 {
            write!(f, "{}\n", statement.item())?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<A: Default> fmt::Display for Statement<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            Statement::Expression(ref expr) => write!(f, "expr:{}", expr.item()),
            Statement::Declare(ref ident, ref expr) => write!(f, "decl({}->{})", ident.item(),
                expr.item()),
            Statement::Assign(ref ident, ref expr) => write!(f, "assign({}->{})", ident.item(),
                expr.item()),
        }
    }
}

impl<A: Default> fmt::Display for Expression<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match *self {
            Expression::Literal(ref lit) => write!(f, "lit:{}", lit.item()),
            Expression::Identifier(ref ident) => write!(f, "ident:{}", ident.item()),
            Expression::Infix { ref op, ref left, ref right } =>
                write!(f, "infix:{}{}{}", left.item(), op, right.item()),
            Expression::Prefix { ref op, ref right } =>
                write!(f, "prefix:{}{}", op, right.item()),
            Expression::Postfix { ref op, ref left } =>
                write!(f, "postfix:{}{}", left.item(), op),
            Expression::Block(ref block) =>
                write!(f, "block:{}", block.item())
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

