//! Type computation abstract syntax tree visitor.
//!
//! This module contains the trait and implementation for walking an annotated abstract syntax tree
//! and computing types, enforcing static type safety, and computing type promotion. This
//! expects that the symbol table annotations already exist on the tree.

use std::rc::Rc;

use sindra::Typed;
use sindra::scope::{Scoped, SymbolStore};
use sindra::inference::{InferTypesBinary, BinaryOpTypes, InferTypesUnary, UnaryOpTypes,
    InferPromotion};

use ast::ast::*;

use sindra::Node;

use PType;
use Symbol;
use visitor::State;

type Result = ::std::result::Result<(), String>;

/// Trait for type computation visitor; implemented for all abstract syntax tree nodes.
pub trait TypeComputationVisitor {
    /// Infer types, enforce type safety, and compute type promotion for this node, and visit any
    /// children.
    fn visit(&self, &mut State) -> Result;
}

impl TypeComputationVisitor for Node<Program> {
    fn visit(&self, state: &mut State) -> Result {
        self.item.0.visit(state)?;
        Ok(())
    }
}

impl TypeComputationVisitor for Node<Block> {
    fn visit(&self, state: &mut State) -> Result {
        let mut last_ty: Option<PType> = None;
        let mut ret_ty: Option<PType> = None;
        for statement in self.item.0.iter() {
            statement.visit(state)?;
            last_ty = statement.annotation.borrow_mut().ty();

            // if statement is return, see if it matches previous return types
            if let Statement::Return(_) = statement.item {
                match ret_ty {
                    Some(_) => {
                        if last_ty != ret_ty {
                            state.logger.error(format!("return types do not match"));
                        }
                    },
                    None => {
                        ret_ty = last_ty.clone();
                    }
                }
            }

        }
        match ret_ty {
            Some(_) => {
                if last_ty != ret_ty {
                    state.logger.error("return type does not match type of \
                        final block statement".to_string());
                }
            },
            None => {}
        }
        self.annotation.borrow_mut().set_type(last_ty);
        Ok(())
    }
}

impl TypeComputationVisitor for Node<Statement> {
    fn visit(&self, state: &mut State) -> Result {
        let ty = match (&self.item, &self.annotation) {
            (&Statement::Declare(ref ident, ref expr), &ref annotation) => {
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
            (&Statement::Assign(ref ident, ref expr), &ref annotation) => {
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
            (&Statement::Expression(ref expr), _) => {
                expr.visit(state)?;
                expr.annotation.borrow().ty()
            },
            (&Statement::FnDefine(FunctionDef { ref name, ref body, ref ret_type,
                    ref params }), &ref annotation) => {
                for param in params.iter() {
                    let param_name = param.item.name.item.clone();
                    let param_ty = param.item.ty.item.clone();

                    let fn_scope = body.annotation.borrow().scope().unwrap();
                    let pm_ty = if let Some(param_ty_sym) = fn_scope.borrow().resolve(&param_ty) {
                        match param_ty_sym {
                            Symbol::Variable { .. } => {
                                state.logger.error(format!("variable '{}' not valid as type \
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
                    // re-declare parameter as a variable with computed type in the funciton scope
                    fn_scope.borrow_mut().define(param_name.clone(),
                        Symbol::variable(param_name.clone(), pm_ty));
                    // set the parameter type
                    param.annotation.borrow_mut().set_type(pm_ty);
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
            (&Statement::Return(ref expr), _) | (&Statement::Break(ref expr), _) => {
                expr.visit(state)?;
                expr.annotation.borrow().ty()
            },
            (&Statement::Print(ref exprs), _)  => {
                for expr in exprs {
                    expr.visit(state)?;
                }
                Some(PType::Void)
            }
        };
        self.annotation.borrow_mut().set_type(ty);
        Ok(())
    }
}

impl TypeComputationVisitor for Node<Expression> {
    fn visit(&self, state: &mut State) -> Result {
        // borrow the scope
        let scope = match self.annotation.borrow().scope() {
            Some(ref s) => Rc::clone(&s),
            None => {
                return Err("type computation attempted without defined symbols".to_string());
            }
        };

        let ty = match (&self.item, &self.annotation) {
            (&Expression::Literal(ref node), _) => {
                match node.item {
                    Literal::String(_) => Some(PType::String),
                    Literal::Float(_) => Some(PType::Float),
                    Literal::Int(_) => Some(PType::Int),
                    Literal::Boolean(_) => Some(PType::Boolean),
                }
            },
            (&Expression::Identifier(ref node), _) => {
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
            (&Expression::Infix { ref left, ref right, ref op }, _) => {
                left.visit(state)?;
                right.visit(state)?;
                let tleft = left.annotation.borrow().ty().unwrap();
                let tright = right.annotation.borrow().ty().unwrap();
                match op.infer_types(tleft, tright) {
                    Some(BinaryOpTypes { result: ty, left: promo_left, right: promo_right }) => {
                        left.annotation.borrow_mut().set_promote_type(
                            tleft.infer_promotion(promo_left));
                        right.annotation.borrow_mut().set_promote_type(
                            tright.infer_promotion(promo_right));
                        Some(ty)
                    },
                    None => {
                        state.logger.error(format!("incompatible types for {}: {}, {}",
                            op, left.item, right.item));
                        None
                    }
                }

            },
            (&Expression::Prefix { ref right, ref op }, _) => {
                right.visit(state)?;
                let tright = right.annotation.borrow().ty().unwrap();
                match op.infer_types(tright) {
                    Some(UnaryOpTypes { result: result_ty, operand: promo_ty }) => {
                        right.annotation.borrow_mut().set_promote_type(
                            tright.infer_promotion(promo_ty));
                        Some(result_ty)
                    },
                    None => {
                        state.logger.error(format!("incompatible type for {}: {}", op,
                            right.item));
                        None
                    }
                }
            },
            (&Expression::Postfix { ref left, ref op }, _) => {
                left.visit(state)?;
                let tleft = left.annotation.borrow().ty().unwrap();
                match op.infer_types(tleft) {
                    Some(UnaryOpTypes { result: result_ty, operand: promo_ty }) => {
                        left.annotation.borrow_mut().set_promote_type(
                            tleft.infer_promotion(promo_ty));
                        Some(result_ty)
                    },
                    None => {
                        state.logger.error(format!("incompatible type for {}: {}", op,
                            left.item));
                        None
                    }
                }
            },
            (&Expression::Block(ref block), _) => {
                block.visit(state)?;
                block.annotation.borrow().ty()
            },
            (&Expression::FnCall { name: ref ident, ref args }, _) => {
                for ref arg in args.iter() {
                    arg.visit(state)?;
                }
                let id = &ident.item;
                match scope.borrow().resolve(&id) {
                    Some(Symbol::Variable { .. }) => {
                        state.logger.error(format!(
                            "attempt to call function on variable {}", id));
                        None
                    },
                    Some(Symbol::Function { ret_ty, ref name, ref params, .. }) => {
                        if args.len() != params.len() {
                            state.logger.error(format!("function '{}' expects {} arguments,
                                {} found", name, params.len(), args.len()));
                            None
                        } else {
                            // check the parameter types
                            for (ref param, ref arg) in params.iter().zip(args) {
                                let arg_ty = arg.annotation.borrow().ty().unwrap();
                                let param_ty = param.annotation.borrow().ty().unwrap();

                                if param_ty != arg_ty {
                                    match arg_ty.infer_promotion(param_ty) {
                                        Some(promoted) => {
                                            arg.annotation.borrow_mut().set_promote_type(
                                                Some(promoted));
                                        },
                                        None => {
                                            state.logger.error(format!(
                                                "invalid argument type for parameter '{}' \
                                                of function '{}': expected '{}', found '{}'",
                                                param.item.name.item, name, param_ty, arg_ty));
                                        }
                                    }
                                }
                            }
                            if ret_ty.is_none() {
                                state.logger.error(format!(
                                    "function '{}' does not have a valid return type", name));
                            }
                            ret_ty
                        }
                    },
                    Some(Symbol::BuiltinType { .. }) => {
                        state.logger.error(format!(
                            "attempt to call function on built-in type {}", id));
                        None
                    },
                    None => {
                        return Err(format!("function '{}' does not exist", id));
                    }
                }
            },
            (&Expression::IfElse { ref cond, ref if_block, ref else_block }, _) => {
                cond.visit(state)?;
                // check type of conditional
                let tcond = cond.annotation.borrow().ty().unwrap();
                if tcond == PType::Boolean {
                    if_block.visit(state)?;
                    if let Some(ref else_block) = *else_block {
                        else_block.visit(state)?;

                        let tif = if_block.annotation.borrow().ty().unwrap();
                        let telse = else_block.annotation.borrow().ty().unwrap();

                        if tif == telse {
                            Some(tif)
                        } else {
                            state.logger.error(format!(
                                "invalid if-else construct: if block returns type '{}', \
                                else block returns type '{}'", tif, telse));
                            Some(PType::Void)
                        }
                    } else {
                        Some(PType::Void)
                    }
                } else {
                    state.logger.error(format!(
                        "conditional expression must be boolean, found type '{}'", tcond));
                    Some(PType::Void)
                }
            },
            (&Expression::Loop { ref variant, ref set, ref body }, _) => {
                set.visit(state)?;
                let var_ty = set.annotation.borrow().ty().unwrap();
                match *variant {
                    Some(ref var) => {
                        body.annotation.borrow_mut().define(var.item.clone(),
                            Symbol::variable(var.item.clone(), Some(var_ty)));
                    },
                    None => {}
                }
                body.visit(state)?;
                Some(body.annotation.borrow().ty().unwrap())
            }
        };
        self.annotation.borrow_mut().set_type(ty);
        Ok(())
    }
}

impl TypeComputationVisitor for Node<Set> {
    fn visit(&self, state: &mut State) -> Result {
        let ty = match self.item {
            Set::Interval { ref start, ref end, ref step, .. } => {
                start.visit(state)?;
                end.visit(state)?;
                step.visit(state)?;

                //TODO: handle promotion from int to float
                let (tstart, tend, tstep) = (
                    start.annotation.borrow().ty().unwrap(),
                    end.annotation.borrow().ty().unwrap(),
                    step.annotation.borrow().ty().unwrap(),
                );

                if tstart == tend && tstart == tstep {
                    Some(tstart)
                } else {
                    state.logger.error(format!("'start' ({}), 'end' ({}), and 'step' ({}) values \
                        are required to be of same type", tstart, tend, tstep));
                    None
                }
            }
        };
        self.annotation.borrow_mut().set_type(ty);
        Ok(())
    }
}
