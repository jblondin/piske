//! Evaluator abstract syntax tree visitor.
//!
//! This module contains the trait and implementation for walking an annotated abstract syntax tree
//! and evaluating it. This implementation expects that the symbol table and type computation
//! annotations already exist on the tree.

use std::io;
use std::cell::RefCell;
use std::rc::Rc;

use sindra::Node;
use sindra::Typed;
use sindra::log::LogListener;
use sindra::scope::{Scoped, MemoryStore, Stack};
use sindra::operator::{UnaryOperator, BinaryOperator};
use sindra::value::Coerce;

use ast::*;
use value::Value;


type Result = ::std::result::Result<Value, String>;

/// State carried throughout the tree walker. Contains logger.
pub struct State {
    /// Logger
    pub logger: LogListener<String, io::Stdout, io::Stderr>,
}
impl State {
    /// Create a new state.
    pub fn new() -> State {
        State {
            logger: LogListener::new(io::stdout(), io::stderr()),
        }
    }
}

/// Evaluator entry point method. Handles setting up the state and initiating the tree walk.
pub fn eval<A>(program: &mut Node<Program<A>, A>) -> Result
        where A: Default + Scoped + Typed<PType>,
              A::Scope: MemoryStore<Value>,
              Rc<RefCell<A::Scope>>: Stack {
    let mut state = State::new();
    let res = program.eval(&mut state);
    state.logger.flush();
    res
}

/// Trait for evaluation visitor; implemented for all abstract syntax tree nodes.
pub trait Evaluator {
    /// Walk the tree, evaluating and producing a result from the program.
    fn eval(&mut self, &mut State) -> Result;
}

impl<A> Evaluator for Node<Program<A>, A>
        where A: Default + Scoped + Typed<PType>,
              A::Scope: MemoryStore<Value>,
              Rc<RefCell<A::Scope>>: Stack {
    fn eval(&mut self, state: &mut State) -> Result {
        self.item_mut().0.eval(state)
    }
}

impl<A> Evaluator for Node<Block<A>, A>
        where A: Default + Scoped + Typed<PType>,
              A::Scope: MemoryStore<Value>,
              Rc<RefCell<A::Scope>>: Stack {
    fn eval(&mut self, state: &mut State) -> Result {
        let mut last_result: Value = Value::Empty;
        for statement in self.item_mut().0.iter_mut() {
            last_result = statement.eval(state)?;
        }
        Ok(last_result)
    }
}

impl<A> Evaluator for Node<Statement<A>, A>
        where A: Default + Scoped + Typed<PType>,
              A::Scope: MemoryStore<Value>,
              Rc<RefCell<A::Scope>>: Stack {
    fn eval(&mut self, state: &mut State) -> Result {
        match self.elems_mut() {
            (&mut Statement::Declare(ref ident, ref mut expr), &mut ref mut annotation) => {
                match annotation.scope() {
                    Some(ref mut scope) => {
                        let value = expr.eval(state)?;
                        scope.borrow_mut().set(ident.item().clone(), value.clone())?;
                        Ok(value)
                    },
                   None => Err("no associated scope in declaration statement".to_string())
                }
            },
            (&mut Statement::Assign(ref ident, ref mut expr), &mut ref mut annotation) => {
                match annotation.scope() {
                    Some(ref mut scope) => {
                        let value = expr.eval(state)?;
                        scope.borrow_mut().set(ident.item().clone(), value.clone())?;
                        Ok(value)
                    },
                    None => Err("no associated scope in assignment statement".to_string())
                }
            },
            (&mut Statement::Expression(ref mut expr), _) => {
                expr.eval(state)
            }
        }
    }
}

impl<A> Evaluator for Node<Expression<A>, A>
        where A: Default + Scoped + Typed<PType>,
              A::Scope: MemoryStore<Value>,
              Rc<RefCell<A::Scope>>: Stack {
    fn eval(&mut self, state: &mut State) -> Result {
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
                let lval = left.eval(state)?;
                let rval = right.eval(state)?;

                op.op(
                    annotation.ty().unwrap(),
                    &lval.coerce(left.annotation().promote_type()),
                    &rval.coerce(right.annotation().promote_type())
                )
            },
            (&mut Expression::Prefix { ref op, ref mut right }, &mut ref mut annotation) => {
                let rval = right.eval(state)?;

                op.op(
                    annotation.ty().unwrap(),
                    &rval.coerce(right.annotation().promote_type())
                )
            },
            (&mut Expression::Postfix { ref op, ref mut left }, &mut ref mut annotation) => {
                let lval = left.eval(state)?;

                op.op(
                    annotation.ty().unwrap(),
                    &lval.coerce(left.annotation().promote_type())
                )
            },
            (&mut Expression::Block(ref mut block), _) => {
                block.eval(state)
            }
        }
    }
}
