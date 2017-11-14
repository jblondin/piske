//! Type computation abstract syntax tree visitor.
//!
//! This module contains the trait and implementation for walking an annotated abstract syntax tree
//! and computing types, enforcing static type safety, and computing type promotion. This
//! expects that the symbol table annotations already exist on the tree.

use std::rc::Rc;

use sindra::Typed;
use sindra::scope::{Scoped, SymbolStore};
use sindra::inference::{InferResultBinary, InferResultUnary, InferPromotion};

use ast::ast::*;

use sindra::Node;

use PType;
use Symbol;
use visitor::State;

type Result = ::std::result::Result<(), String>;

/// Trait to provide easy entry point method to type computation visitor.
pub trait ComputeTypes {
    /// Type computation entry point method. Handles setting up the state and initiating the tree
    /// walk.
    fn compute_types(&mut self) -> Result;
}
impl ComputeTypes for Node<Program> {
    fn compute_types(&mut self) -> Result {
        let mut state = State::default();
        let res = self.visit(&mut state);
        state.logger.flush();
        res
    }
}

/// Trait for type computation visitor; implemented for all abstract syntax tree nodes.
pub trait TypeComputationVisitor {
    /// Infer types, enforce type safety, and compute type promotion for this node, and visit any
    /// children.
    fn visit(&mut self, &mut State) -> Result;
}

impl TypeComputationVisitor for Node<Program> {
    fn visit(&mut self, state: &mut State) -> Result {
        self.item.0.visit(state)?;
        Ok(())
    }
}

impl TypeComputationVisitor for Node<Block> {
    fn visit(&mut self, state: &mut State) -> Result {
        let mut last_ty: Option<PType> = None;
        for statement in self.item.0.iter_mut() {
            statement.visit(state)?;
            last_ty = statement.annotation.borrow_mut().ty();
        }
        self.annotation.borrow_mut().set_type(last_ty);
        Ok(())
    }
}

impl TypeComputationVisitor for Node<Statement> {
    fn visit(&mut self, state: &mut State) -> Result {
        let ty = match (&mut self.item, &mut self.annotation) {
            (&mut Statement::Declare(ref ident, ref mut expr), &mut ref mut annotation) => {
                expr.visit(state)?;
                let ty = expr.annotation.borrow().ty();
                // update the variable type in scope
                let ident = ident.item.clone();
                if let Some(ref mut scope) = annotation.borrow().scope() {
                    if let Some(ty) = ty {
                        scope.borrow_mut().define(ident.clone(),
                            Symbol::variable(ident.clone(), Some(ty)));
                    }
                }
                ty
            },
            (&mut Statement::Assign(ref ident, ref mut expr), &mut ref mut annotation) => {
                expr.visit(state)?;
                let expr_ty = expr.annotation.borrow().ty();
                let ident = ident.item.clone();
                let scope = annotation.borrow().scope().ok_or(
                    format!("no scope associated with identifier {}", ident))?;
                let expr_ty = expr_ty.ok_or(
                    format!("type computation for expression failed in assignment of {}", ident))?;

                if let Some(existing) = scope.borrow().resolve(&ident) {
                    match existing {
                        Symbol::Variable { ty: existing_ty, .. } => {
                            if let Some(dest_ty) = existing_ty {
                                if expr_ty != dest_ty {
                                    match expr_ty.infer_promotion(dest_ty) {
                                        Some(promoted) => {
                                            annotation.borrow_mut().set_promote_type(Some(promoted));
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
                                    Symbol::variable(ident.clone(), Some(expr_ty)));
                            }
                        },
                        Symbol::Function { .. } => {
                            state.logger.error(format!(
                                "function '{}' invalid as lvalue", ident));
                        },
                        Symbol::BuiltinType { .. } => {
                            state.logger.error(
                                format!("attempt to redefine built-in type '{}'", ident));
                        }
                    }
                } else {
                    state.logger.error(
                        format!("attempt to assign to undefined variable '{}'", ident));
                }
                Some(expr_ty)
            },
            (&mut Statement::Expression(ref mut expr), _) => {
                expr.visit(state)?;
                expr.annotation.borrow().ty()
            },
            (&mut Statement::FnDefine(FunctionDef { ref name, ref mut body, ref ret_type,
                    ref params }), &mut ref mut annotation) => {
                for param in params.iter() {
                    let param_name = param.item.name.item.clone();
                    let param_ty = param.item.ty.item.clone();

                    let fn_scope = body.annotation.borrow().scope().unwrap();
                    let pm_ty = if let Some(param_ty_sym) = fn_scope.borrow().resolve(&param_ty) {
                        match param_ty_sym {
                            Symbol::Variable { .. } => {
                                state.logger.error(format!("variable '{}'borrow(). not valid as type \
                                    for parameter '{}'", param_ty, param_name));
                                None
                            },
                            Symbol::Function { .. } => {
                                state.logger.error(format!("function '{}' not valid as type \
                                    for parameter '{}'", param_ty, param_name));
                                None
                            },
                            Symbol::BuiltinType { ty, .. } => {
                                Some(ty)
                            }
                        }
                    } else {
                        state.logger.error(format!("type '{}' unknown for parameter '{}'",
                            param_ty, param_name));
                        None
                    };
                    fn_scope.borrow_mut().define(param_name.clone(),
                        Symbol::variable(param_name.clone(), pm_ty));
                }

                body.visit(state)?;
                let body_ty = body.annotation.borrow_mut().ty();
                // lookup the declared return type
                let name = name.item.clone();
                let scope = annotation.borrow().scope().ok_or(
                    format!("no scope associated with function {}", name))?;
                let body_ty = body_ty.ok_or(
                    format!("type computation for body failed in function {}", name))?;
                let ret_ty = &ret_type.item;

                let r_ty = if let Some(ret_type_sym) = scope.borrow().resolve(&ret_ty) {
                    match ret_type_sym {
                        Symbol::Variable { .. } => {
                            state.logger.error(format!("variable '{}' not valid as return type \
                                for function '{}'", ret_ty, name));
                            None
                        },
                        Symbol::Function { .. } => {
                            state.logger.error(format!("function '{}' not valid as return type \
                                for function '{}'", ret_ty, name));
                            None
                        },
                        Symbol::BuiltinType { ty, .. } => {
                            // return type symbol exists, and it's a valid type. check if the
                            // symbol already exists, and update the symbol in scope if not

                            if let Some(existing) = scope.borrow().resolve(&name) {
                                match existing {
                                    Symbol::Variable { .. } => {
                                        return Err(format!("symbol mismatch for '{}': expected \
                                            Function, found Variable", name));
                                    },
                                    Symbol::BuiltinType { .. } => {
                                        return Err(format!("symbol mismatch for '{}': expected \
                                            Function, found BuiltinType", name));
                                    }
                                    Symbol::Function { ret_ty: ref existing_ret_ty, .. } => {
                                        if let Some(existing_ret_ty) = *existing_ret_ty {
                                            if existing_ret_ty != ty {
                                                state.logger.error(format!(
                                                    "attempt to change return type of '{}'",
                                                    name));
                                            }
                                            Some(ty)
                                        } else {
                                            // return type not previously specified, update it
                                            // with computed type
                                            Some(ty)
                                        }
                                    },
                                }
                            } else {
                                // function does not exist to update, failure mode
                                return Err(format!("function '{}' does not exist in scope", name));
                            }
                        }
                    }
                } else {
                    state.logger.error(format!("return type '{}' for function '{}' unknown",
                        ret_ty, name));
                    None
                };
                scope.borrow_mut().define(name.clone(),
                        Symbol::function(name.clone(), r_ty,
                            body.clone(), params.clone()));

                Some(body_ty)
            },
        };
        self.annotation.borrow_mut().set_type(ty);
        Ok(())
    }
}

impl TypeComputationVisitor for Node<Expression> {
    fn visit(&mut self, state: &mut State) -> Result {
        // borrow the scope
        let scope = match self.annotation.borrow().scope() {
            Some(ref s) => Rc::clone(&s),
            None => {
                return Err("type computation attempted without defined symbols".to_string());
            }
        };

        let ty = match (&mut self.item, &mut self.annotation) {
            (&mut Expression::Literal(ref node), _) => {
                match node.item {
                    Literal::String(_) => { Some(PType::String) },
                    Literal::Float(_) => { Some(PType::Float) },
                    Literal::Int(_) => { Some(PType::Int) }
                }
            },
            (&mut Expression::Identifier(ref node), _) => {
                match scope.borrow().resolve(&node.item) {
                    Some(ref sym) => {
                        match *sym {
                            Symbol::Variable { ty, .. } => { ty },
                            Symbol::Function { ret_ty, .. } => ret_ty,
                            Symbol::BuiltinType { ty, .. } => Some(ty),
                        }
                    },
                    None => None
                }
            },
            (&mut Expression::Infix { ref mut left, ref mut right, ref op }, _) => {
                left.visit(state)?;
                right.visit(state)?;
                let tleft = left.annotation.borrow().ty().unwrap();
                let tright = right.annotation.borrow().ty().unwrap();
                match op.infer_result_type(tleft, tright) {
                    Some(ty) => {
                        left.annotation.borrow_mut().set_promote_type(tleft.infer_promotion(ty));
                        right.annotation.borrow_mut().set_promote_type(tright.infer_promotion(ty));
                        Some(ty)
                    },
                    None => {
                        state.logger.error(format!("incompatible types for {}: {}, {}",
                            op, left.item, right.item));
                        None
                    }
                }

            },
            (&mut Expression::Prefix { ref mut right, ref op }, _) => {
                right.visit(state)?;
                let tright = right.annotation.borrow().ty().unwrap();
                match op.infer_result_type(tright) {
                    Some(result_ty) => {
                        right.annotation.borrow_mut().set_promote_type(
                            tright.infer_promotion(result_ty));
                        Some(result_ty)
                    },
                    None => {
                        state.logger.error(format!("incompatible type for {}: {}", op,
                            right.item));
                        None
                    }
                }
            },
            (&mut Expression::Postfix { ref mut left, ref op }, _) => {
                left.visit(state)?;
                let tleft = left.annotation.borrow().ty().unwrap();
                match op.infer_result_type(tleft) {
                    Some(result_ty) => {
                        left.annotation.borrow_mut().set_promote_type(
                            tleft.infer_promotion(result_ty));
                        Some(result_ty)
                    },
                    None => {
                        state.logger.error(format!("incompatible type for {}: {}", op,
                            left.item));
                        None
                    }
                }
            },
            (&mut Expression::Block(ref mut block), _) => {
                block.visit(state)?;
                block.annotation.borrow().ty()
            },
            (&mut Expression::FnCall { name: ref ident, ref mut args }, _) => {
                for ref mut arg in args.iter_mut() {
                    arg.visit(state)?;
                }
                let id = &ident.item;
                match scope.borrow().resolve(&id) {
                    Some(ref sym) => {
                        match *sym {
                            Symbol::Variable { .. } => {
                                state.logger.error(format!(
                                    "attempt to call function on variable {}", id));
                                None
                            },
                            Symbol::Function { ret_ty, .. } => ret_ty,
                            Symbol::BuiltinType { .. } => {
                                state.logger.error(format!(
                                    "attempt to call function on built-in type {}", id));
                                None
                            },
                        }
                    },
                    None => {
                        return Err(format!("function '{}' does not exist", id));
                    }
                }
            }
        };

        self.annotation.borrow_mut().set_type(ty);
        Ok(())
    }
}
