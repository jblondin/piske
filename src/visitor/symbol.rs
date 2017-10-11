//! Type computation abstract syntax tree visitor.
//!
//! This module contains the trait and implementation for walking an abstract syntax tree and
//! filling in the symbol table. No previous annotation information needs to exist.

use std::cell::RefCell;
use std::rc::Rc;
use std::io;

use sindra::{Symbol, Node};
use sindra::scope::{Scoped, Stack, SymbolStore};
use sindra::log::LogListener;

use ast::PType;
use ast::ast::*;

/// State carried throughout the tree walker. Contains scope information and logger.
pub struct State<Sc> {
    /// Reference to the current scope stack.
    pub scope: Rc<RefCell<Sc>>,
    /// Logger
    pub logger: LogListener<String, io::Stdout, io::Stderr>,
}
impl<Sc: Default> Default for State<Sc> {
    fn default() -> State<Sc> {
        State {
            scope: Rc::new(RefCell::new(Sc::default())),
            logger: LogListener::new(io::stdout(), io::stderr()),
        }
    }
}

type Result = ::std::result::Result<(), String>;

/// Trait to provide easy entry point method to symbol definition visitor.
pub trait DefineSymbols {
    /// Symbol definition entry point method. Handles setting up the state and initiating the tree
    /// walk.
    fn define_symbols(&mut self) -> Result;
}
impl<A> DefineSymbols for Node<Program<A>, A>
        where A: Default + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    fn define_symbols(&mut self) -> Result {
        let mut state = State::default();
        let res = self.visit(&mut state);
        res
    }
}

/// Trait for symbol definition visitor; implemented for all abstract syntax tree nodes.
pub trait SymbolDefineVisitor<Sc> {
    /// Verify and populate symbol table symbols for this node, and visit any children.
    fn visit(&mut self, &mut State<Sc>) -> Result;
}

impl<A> SymbolDefineVisitor<A::Scope> for Node<Program<A>, A>
        where A: Default + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    fn visit(&mut self, state: &mut State<A::Scope>) -> Result {
        state.scope = state.scope.push();
        self.item_mut().0.visit(state)?;
        match state.scope.pop() {
            Some(parent_scope) => { state.scope = parent_scope; }
            None => {
                return Err("invalid descoping".to_string());
            }
        }
        self.annotation_mut().set_scope(Some(Rc::clone(&state.scope)));
        Ok(())
    }
}

impl<A> SymbolDefineVisitor<A::Scope> for Node<Block<A>, A>
        where A: Default + Scoped,
        A::Scope: SymbolStore<Symbol<PType>>,
        Rc<RefCell<A::Scope>>: Stack {
    fn visit(&mut self, state: &mut State<A::Scope>) -> Result {
        for statement in self.item_mut().0.iter_mut() {
            statement.visit(state)?;
        }
        Ok(())
    }
}

impl<A> SymbolDefineVisitor<A::Scope> for Node<Statement<A>, A>
        where A: Default + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    fn visit(&mut self, state: &mut State<A::Scope>) -> Result {
        self.annotation_mut().set_scope(Some(Rc::clone(&state.scope)));
        match *self.item_mut() {
            Statement::Declare(ref id, ref mut expr) => {
                expr.visit(state)?;
                state.scope.borrow_mut().define(id.item().clone(),
                    Symbol::Variable(id.item().clone(), None));
                Ok(())
            },
            Statement::Assign(ref id, ref mut expr) => {
                expr.visit(state)?;
                match state.scope.borrow().resolve(id.item()) {
                    Some(_) => Ok(()),
                    None => {
                        state.logger.error(format!("symbol {} does not exist in scope",
                            id.item()));
                        Ok(())
                    }
                }
            },
            Statement::Expression(ref mut expr) => {
                expr.visit(state)
            },
        }
    }
}

impl<A> SymbolDefineVisitor<A::Scope> for Node<Expression<A>, A>
        where A: Default + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    fn visit(&mut self, state: &mut State<A::Scope>) -> Result {
        self.annotation_mut().set_scope(Some(Rc::clone(&state.scope)));
        match *self.item_mut() {
            Expression::Literal(_) => {
                Ok(())
            },
            Expression::Identifier(ref id) => {
                match state.scope.borrow().resolve(id.item()) {
                    Some(_) => Ok(()),
                    None => {
                        state.logger.error(format!("symbol {} does not exist in scope",
                            id.item()));
                        Ok(())
                    }
                }
            },
            Expression::Infix { ref mut left, ref mut right, .. } => {
                left.visit(state)?;
                right.visit(state)?;
                Ok(())
            },
            Expression::Prefix { ref mut right, .. } => {
                right.visit(state)?;
                Ok(())
            },
            Expression::Postfix { ref mut left, .. } => {
                left.visit(state)?;
                Ok(())
            },
            Expression::Block(ref mut block) => {
                state.scope = state.scope.push();
                block.visit(state)?;
                match state.scope.pop() {
                    Some(parent_scope) => { state.scope = parent_scope; }
                    None => {
                        return Err("invalid descoping".to_string());
                    }
                }
                Ok(())
            }
        }
    }
}
