//! Evaluation abstract syntax tree visitor.
//!
//! This module contains the trait and implementation for walking an annotated abstract syntax tree
//! and evaluating it. This implementation expects that the symbol table and type computation
//! annotations already exist on the tree.

use std::cell::RefCell;
use std::rc::Rc;

use sindra::Node;
use sindra::Typed;
use sindra::scope::{Scoped, MemoryStore, Stack};
use sindra::operator::{UnaryOperator, BinaryOperator};
use sindra::value::Coerce;

use ast::*;
use value::Value;

type Result = ::std::result::Result<Value, String>;

/// State carried throughout the tree walker. Currently contains nothing; exists for furutre use.
pub struct State {
}
impl Default for State {
    fn default() -> State {
        State {}
    }
}

/// Trait to provide easy entry point method to evaluation visitor.
pub trait Evaluate {
    /// Evaluate entry point method. Handles setting up the state and initiating the tree walk.
    fn eval(&mut self) -> Result;
}
impl<A> Evaluate for  Node<Program<A>, A>
        where A: Default + Scoped + Typed<PType>,
              A::Scope: MemoryStore<Value>,
              Rc<RefCell<A::Scope>>: Stack {
    fn eval(&mut self) -> Result {
        let mut state = State::default();
        let res = self.visit(&mut state);
        res
    }
}

/// Trait for evaluation visitor; implemented for all abstract syntax tree nodes.
pub trait EvaluateVisitor {
    /// Walk the tree, evaluating and producing a result from the program.
    fn visit(&mut self, &mut State) -> Result;
}

impl<A> EvaluateVisitor for Node<Program<A>, A>
        where A: Default + Scoped + Typed<PType>,
              A::Scope: MemoryStore<Value>,
              Rc<RefCell<A::Scope>>: Stack {
    fn visit(&mut self, state: &mut State) -> Result {
        self.item_mut().0.visit(state)
    }
}

impl<A> EvaluateVisitor for Node<Block<A>, A>
        where A: Default + Scoped + Typed<PType>,
              A::Scope: MemoryStore<Value>,
              Rc<RefCell<A::Scope>>: Stack {
    fn visit(&mut self, state: &mut State) -> Result {
        let mut last_result: Value = Value::Empty;
        for statement in self.item_mut().0.iter_mut() {
            last_result = statement.visit(state)?;
        }
        Ok(last_result)
    }
}

impl<A> EvaluateVisitor for Node<Statement<A>, A>
        where A: Default + Scoped + Typed<PType>,
              A::Scope: MemoryStore<Value>,
              Rc<RefCell<A::Scope>>: Stack {
    fn visit(&mut self, state: &mut State) -> Result {
        match self.elems_mut() {
            (&mut Statement::Declare(ref ident, ref mut expr), &mut ref mut annotation) => {
                match annotation.scope() {
                    Some(ref mut scope) => {
                        let value = expr.visit(state)?;
                        scope.borrow_mut().set(ident.item().clone(), value.clone())?;
                        Ok(value)
                    },
                   None => Err("no associated scope in declaration statement".to_string())
                }
            },
            (&mut Statement::Assign(ref ident, ref mut expr), &mut ref mut annotation) => {
                match annotation.scope() {
                    Some(ref mut scope) => {
                        let value = expr.visit(state)?;
                        scope.borrow_mut().set(ident.item().clone(), value.clone())?;
                        Ok(value)
                    },
                    None => Err("no associated scope in assignment statement".to_string())
                }
            },
            (&mut Statement::Expression(ref mut expr), _) => {
                expr.visit(state)
            }
        }
    }
}

impl<A> EvaluateVisitor for Node<Expression<A>, A>
        where A: Default + Scoped + Typed<PType>,
              A::Scope: MemoryStore<Value>,
              Rc<RefCell<A::Scope>>: Stack {
    fn visit(&mut self, state: &mut State) -> Result {
        match self.elems_mut() {
            (&mut Expression::Literal(ref literal), _) => {
                Ok(Value::from(literal.item().clone()))
            },
            (&mut Expression::Identifier(ref ident), &mut ref mut annotation) => {
                let ident = ident.item();
                match annotation.scope() {
                    Some(ref scope) => {
                        match scope.borrow().get(ident) {
                            Some(ref value) => Ok(value.clone()),
                            None => Err(format!("uninitialized variable: {}", ident))
                        }
                    },
                    None => Err("invalid scope".to_string())
                }
            },
            (&mut Expression::Infix { ref op, ref mut left, ref mut right },
                    &mut ref mut annotation) => {
                let lval = left.visit(state)?;
                let rval = right.visit(state)?;

                op.op(
                    annotation.ty().unwrap(),
                    &lval.coerce(left.annotation().promote_type()),
                    &rval.coerce(right.annotation().promote_type())
                )
            },
            (&mut Expression::Prefix { ref op, ref mut right }, &mut ref mut annotation) => {
                let rval = right.visit(state)?;

                op.op(
                    annotation.ty().unwrap(),
                    &rval.coerce(right.annotation().promote_type())
                )
            },
            (&mut Expression::Postfix { ref op, ref mut left }, &mut ref mut annotation) => {
                let lval = left.visit(state)?;

                op.op(
                    annotation.ty().unwrap(),
                    &lval.coerce(left.annotation().promote_type())
                )
            },
            (&mut Expression::Block(ref mut block), _) => {
                block.visit(state)
            }
        }
    }
}
