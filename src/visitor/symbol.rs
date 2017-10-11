//! Type computation abstract syntax tree visitor.
//!
//! This module contains the trait and implementation for walking an abstract syntax tree and
//! filling in the symbol table. No previous annotation information needs to exist.

use std::cell::RefCell;
use std::rc::Rc;

use sindra::{Symbol, Node};
use sindra::scope::{Scoped, Stack, SymbolStore};

use ast::PType;
use ast::ast::*;

/// State carried throughout the tree walker. Contains scope information and logger.
pub struct State<Sc> {
    /// Reference to the current scope stack.
    pub scope: Rc<RefCell<Sc>>,
    //TODO: add log listener
}
impl<Sc: Default> State<Sc> {
    /// Create a new state using the default `Sc` scope constructor.
    pub fn new() -> State<Sc> {
        State {
            scope: Rc::new(RefCell::new(Sc::default()))
        }
    }
}

type Result = ::std::result::Result<(), String>;

/// Symbol definition entry point method. Handles setting up the state and initiating the tree walk.
pub fn define_symbols<A>(program: &mut Node<Program<A>, A>) -> Result
        where A: Default + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    let mut state = State::new();
    let res = program.define_symbols(&mut state);
    res
}

/// Trait for symbol definition visitor; implemented for all abstract syntax tree nodes.
pub trait SymbolDefineVisitor<Sc> {
    /// Verify and populate symbol table symbols for this node, and visit any children.
    fn define_symbols(&mut self, &mut State<Sc>) -> Result;
}

impl<A> SymbolDefineVisitor<A::Scope> for Node<Program<A>, A>
        where A: Default + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    fn define_symbols(&mut self, state: &mut State<A::Scope>) -> Result {
        state.scope = state.scope.push();
        self.item_mut().0.define_symbols(state)?;
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
    fn define_symbols(&mut self, state: &mut State<A::Scope>) -> Result {
        for statement in self.item_mut().0.iter_mut() {
            statement.define_symbols(state)?;
        }
        Ok(())
    }
}

impl<A> SymbolDefineVisitor<A::Scope> for Node<Statement<A>, A>
        where A: Default + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    fn define_symbols(&mut self, state: &mut State<A::Scope>) -> Result {
        self.annotation_mut().set_scope(Some(Rc::clone(&state.scope)));
        match *self.item_mut() {
            Statement::Declare(ref id, ref mut expr) => {
                expr.define_symbols(state)?;
                state.scope.borrow_mut().define(id.item().clone(),
                    Symbol::Variable(id.item().clone(), None));
                Ok(())
            },
            Statement::Assign(ref id, ref mut expr) => {
                expr.define_symbols(state)?;
                match state.scope.borrow().resolve(id.item()) {
                    Some(_) => Ok(()),
                    None => Err(format!("symbol {} does not exist in scope", id.item()))
                }
            },
            Statement::Expression(ref mut expr) => {
                expr.define_symbols(state)
            },
        }
    }
}

impl<A> SymbolDefineVisitor<A::Scope> for Node<Expression<A>, A>
        where A: Default + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    fn define_symbols(&mut self, state: &mut State<A::Scope>) -> Result {
        self.annotation_mut().set_scope(Some(Rc::clone(&state.scope)));
        match *self.item_mut() {
            Expression::Literal(_) => {
                Ok(())
            },
            Expression::Identifier(ref id) => {
                match state.scope.borrow().resolve(id.item()) {
                    Some(_) => Ok(()),
                    None => Err(format!("symbol {} does not exist in scope", id.item()))
                }
            },
            Expression::Infix { ref mut left, ref mut right, .. } => {
                left.define_symbols(state)?;
                right.define_symbols(state)?;
                Ok(())
            },
            Expression::Prefix { ref mut right, .. } => {
                right.define_symbols(state)?;
                Ok(())
            },
            Expression::Postfix { ref mut left, .. } => {
                left.define_symbols(state)?;
                Ok(())
            },
            Expression::Block(ref mut block) => {
                state.scope = state.scope.push();
                block.define_symbols(state)?;
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
