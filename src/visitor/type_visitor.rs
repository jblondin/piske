//! Type computation abstract syntax tree visitor.
//!
//! This module contains the trait and implementation for walking an annotated abstract syntax tree
//! and computing types, enforcing static type safety, and computing type promotion. This
//! expects that the symbol table annotations already exist on the tree.

use std::rc::Rc;
use std::cell::RefCell;
use std::io;

use sindra::{self, Typed};
use sindra::scope::{Stack, Scoped, SymbolStore};
use sindra::inference::{InferResultBinary, InferResultUnary, InferPromotion};
use sindra::log::LogListener;

use ast::PType;
use ast::ast::*;

use sindra::{Node, Symbol};

type Result = ::std::result::Result<(), String>;

/// State carried throughout the tree walker. Contains logger.
pub struct State {
    /// Logger
    pub logger: LogListener<String, io::Stdout, io::Stderr>
}
impl Default for State {
    fn default() -> State {
        State {
            logger: LogListener::new(io::stdout(), io::stderr()),
        }
    }
}

/// Type computation entry point method. Handles setting up the state and initiating the tree walk.
pub fn compute_types<A>(program: &mut Node<Program<A>, A>) -> Result
        where A: Default + Typed<PType> + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    let mut state = State::default();
    let res = program.compute_types(&mut state);
    state.logger.flush();
    res
}

/// Trait for type computation visitor; implemented for all abstract syntax tree nodes.
pub trait TypeComputationVisitor {
    /// Infer types, enforce type safety, and compute type promotion for this node, and visit any
    /// children.
    fn compute_types(&mut self, &mut State) -> Result;
}

impl<A> TypeComputationVisitor for Node<Program<A>, A>
        where A: Default + Typed<PType> + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    fn compute_types(&mut self, state: &mut State) -> Result {
        self.item_mut().0.compute_types(state)?;
        Ok(())
    }
}

impl<A> TypeComputationVisitor for Node<Block<A>, A>
        where A: Default + Typed<PType> + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    fn compute_types(&mut self, state: &mut State) -> Result {
        let mut last_ty: Option<PType> = None;
        for statement in self.item_mut().0.iter_mut() {
            statement.compute_types(state)?;
            last_ty = statement.annotation().ty();
        }
        self.annotation_mut().set_type(last_ty);
        Ok(())
    }
}

impl<A> TypeComputationVisitor for Node<Statement<A>, A>
        where A: Default + Typed<PType> + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    fn compute_types(&mut self, state: &mut State) -> Result {
        let ty = match self.elems_mut() {
            (&mut Statement::Declare(ref ident, ref mut expr), &mut ref mut annotation) => {
                expr.compute_types(state)?;
                let ty = expr.annotation().ty();
                // update the variable type in scope
                let ident = ident.item().clone();
                if let Some(ref mut scope) = annotation.scope() {
                    if let Some(ty) = ty {
                        scope.borrow_mut().define(ident.clone(),
                            sindra::Symbol::Variable(ident.clone(), Some(ty)));
                    }
                }
                ty
            },
            (&mut Statement::Assign(ref ident, ref mut expr), &mut ref mut annotation) => {
                expr.compute_types(state)?;
                let ty = expr.annotation().ty();
                let ident = ident.item().clone();
                if let Some(ref mut scope) = annotation.scope() {
                    if let Some(ty) = ty {
                        if let Some(existing) = scope.borrow().resolve(&ident) {
                            match existing {
                                sindra::Symbol::Variable(_, existing_ty) => {
                                    if let Some(dest_ty) = existing_ty {
                                        if ty != dest_ty {
                                            match ty.infer_promotion(dest_ty) {
                                                Some(promoted) => {
                                                    annotation.set_promote_type(Some(promoted));
                                                },
                                                None => {
                                                    state.logger.error(format!(
                                                        "attempt to change variable type of '{}'",
                                                        ident));
                                                }
                                            }
                                        }
                                    } else {
                                        // ident exists in scope but doesn't have a type,
                                        // update it
                                        scope.borrow_mut().define(ident.clone(),
                                            sindra::Symbol::Variable(ident.clone(), Some(ty)));
                                    }
                                },
                                sindra::Symbol::BuiltinType(..) => {
                                    return Err(
                                        format!("attempt to redefine built-in type"));
                                }
                            }
                        } else {
                            state.logger.error(
                                format!("attempt to assign to undefined variable '{}'", ident));
                        }
                    }
                }
                ty
            },
            (&mut Statement::Expression(ref mut expr), &mut ref mut annotation) => {
                expr.compute_types(state)?;
                annotation.ty()
            },
        };
        self.annotation_mut().set_type(ty);
        Ok(())
    }
}

impl<A> TypeComputationVisitor for Node<Expression<A>, A>
        where A: Default + Typed<PType> + Scoped,
              A::Scope: SymbolStore<Symbol<PType>>,
              Rc<RefCell<A::Scope>>: Stack {
    fn compute_types(&mut self, state: &mut State) -> Result {
        // borrow the scope
        let scope = match self.annotation().scope() {
            Some(ref s) => Rc::clone(&s),
            None => {
                return Err("type computation attempted without defined symbols".to_string());
            }
        };

        let ty = match self.elems_mut() {
            (&mut Expression::Literal(ref node), _) => {
                match *node.item() {
                    Literal::String(_) => { Some(PType::String) },
                    Literal::Float(_) => { Some(PType::Float) },
                    Literal::Int(_) => { Some(PType::Int) }
                }
            },
            (&mut Expression::Identifier(ref node), _) => {
                match scope.borrow().resolve(&node.item()) {
                    Some(ref sym) => {
                        match *sym {
                            sindra::Symbol::Variable(_, ty) => ty,
                            sindra::Symbol::BuiltinType(_, ty) => Some(ty),
                        }
                    },
                    None => None
                }
            },
            (&mut Expression::Infix { ref mut left, ref mut right, ref op }, _) => {
                left.compute_types(state)?;
                right.compute_types(state)?;
                let tleft = left.annotation().ty().unwrap();
                let tright = right.annotation().ty().unwrap();
                match op.infer_result_type(tleft, tright) {
                    Some(ty) => {
                        left.annotation_mut().set_promote_type(tleft.infer_promotion(ty));
                        right.annotation_mut().set_promote_type(tright.infer_promotion(ty));
                        Some(ty)
                    },
                    None => {
                        state.logger.error(format!("incompatible types for {}: {}, {}",
                            op, left.item(), right.item()));
                        None
                    }
                }

            },
            (&mut Expression::Prefix { ref mut right, ref op }, _) => {
                right.compute_types(state)?;
                let tright = right.annotation().ty().unwrap();
                match op.infer_result_type(tright) {
                    Some(result_ty) => {
                        right.annotation_mut().set_promote_type(tright.infer_promotion(result_ty));
                        Some(result_ty)
                    },
                    None => {
                        state.logger.error(format!("incompatible type for {}: {}", op,
                            right.item()));
                        None
                    }
                }
            },
            (&mut Expression::Postfix { ref mut left, ref op }, _) => {
                left.compute_types(state)?;
                let tleft = left.annotation().ty().unwrap();
                match op.infer_result_type(tleft) {
                    Some(result_ty) => {
                        left.annotation_mut().set_promote_type(tleft.infer_promotion(result_ty));
                        Some(result_ty)
                    },
                    None => {
                        state.logger.error(format!("incompatible type for {}: {}", op,
                            left.item()));
                        None
                    }
                }
            },
            (&mut Expression::Block(ref mut block), _) => {
                block.compute_types(state)?;
                block.annotation().ty()
            }
        };
        self.annotation_mut().set_type(ty);
        Ok(())
    }
}
