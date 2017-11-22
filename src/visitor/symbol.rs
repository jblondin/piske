//! Type computation abstract syntax tree visitor.
//!
//! This module contains the trait and implementation for walking an abstract syntax tree and
//! filling in the symbol table. No previous annotation information needs to exist.

use std::rc::Rc;

use sindra::Node;
use sindra::scope::{Stack, Scoped, SymbolStore};

use visitor::State;
use Symbol;
use ast::ast::*;

type Result = ::std::result::Result<(), String>;

/// Trait to provide easy entry point method to symbol definition visitor.
pub trait DefineSymbols {
    /// Symbol definition entry point method. Handles setting up the state and initiating the tree
    /// walk.
    fn define_symbols(&self) -> Result;
}
impl DefineSymbols for Node<Program> {
    fn define_symbols(&self) -> Result {
        let mut state = State::default();
        let res = self.visit(&mut state);
        res
    }
}

/// Trait for symbol definition visitor; implemented for all abstract syntax tree nodes.
pub trait SymbolDefineVisitor {
    /// Verify and populate symbol table symbols for this node, and visit any children.
    fn visit(&self, &mut State) -> Result;
}


fn visit_block(block: &Node<Block>, state: &mut State) -> Result {
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

impl SymbolDefineVisitor for Node<Program> {
    fn visit(&self, state: &mut State) -> Result {
        // define builtins in top-level (global) scope
        state.define_builtins();
        visit_block(&self.item.0, state)?;
        self.annotation.borrow_mut().set_scope(Some(Rc::clone(&state.scope)));
        Ok(())
    }
}

impl SymbolDefineVisitor for Node<Block> {
    fn visit(&self, state: &mut State) -> Result {
        for statement in self.item.0.iter() {
            statement.visit(state)?;
        }
        self.annotation.borrow_mut().set_scope(Some(Rc::clone(&state.scope)));
        Ok(())
    }
}

impl SymbolDefineVisitor for Node<Statement> {
    fn visit(&self, state: &mut State) -> Result {
        self.annotation.borrow_mut().set_scope(Some(Rc::clone(&state.scope)));
        match self.item {
            Statement::Declare(ref id, ref expr) => {
                expr.visit(state)?;
                let id = id.item.clone();
                state.scope.borrow_mut().define(id.clone(),
                    Symbol::variable(id.clone(), None));
                Ok(())
            },
            Statement::Assign(ref id, ref expr) => {
                expr.visit(state)?;
                let id = id.item.clone();
                let sym: Option<Symbol> = state.scope.borrow().resolve(&id);
                match sym {
                    Some(_) => {
                        //TODO: check for attempted redefinitions of symbols are different variants
                        Ok(())
                    },
                    None => {
                        state.logger.error(format!("symbol '{}' does not exist in scope",
                            id));
                        Ok(())
                    }
                }
            },
            Statement::Expression(ref expr) => {
                expr.visit(state)
            },
            Statement::FnDefine(FunctionDef { ref name, ref body, ref params, .. }) => {
                // make sure function definition is at top scope
                let parent = state.scope.peek();
                if parent.is_some() && Rc::ptr_eq(&parent.unwrap(), &state.global) {
                    let prev_scope = Rc::clone(&state.scope);
                    // create new branch of scope tree under global
                    state.scope = state.global.push();
                    // define parameters in the new scope
                    for param in params.iter() {
                        let param_name = param.item.name.item.clone();
                        state.scope.borrow_mut().define(param_name.clone(),
                            Symbol::variable(param_name.clone(), None));
                    }
                    // define symbols in the body of the function
                    body.visit(state)?;
                    // return to previous top-level scope
                    state.scope = prev_scope;
                    // add function symbol to scope
                    state.scope.borrow_mut().define(name.item.clone(),
                        Symbol::function(name.item.clone(), None, body.clone(),
                            params.clone()));
                    Ok(())
                } else {
                    state.logger.error(format!("function definition '{}' only allowed at \
                        global scope", name.item));
                    Ok(())
                }
            },
            Statement::Return(ref expr) | Statement::Break(ref expr) => {
                expr.visit(state)?;
                Ok(())
            }
        }
    }
}

impl SymbolDefineVisitor for Node<Expression> {
    fn visit(&self, state: &mut State) -> Result {
        self.annotation.borrow_mut().set_scope(Some(Rc::clone(&state.scope)));
        match self.item {
            Expression::Literal(_) => {
                Ok(())
            },
            Expression::Identifier(ref id) => {
                let sym: Option<Symbol> = state.scope.borrow().resolve(&id.item);
                match sym {
                    Some(_) => Ok(()),
                    None => {
                        state.logger.error(format!("symbol '{}' does not exist in scope",
                            id.item));
                        Ok(())
                    }
                }
            },
            Expression::Infix { ref left, ref right, .. } => {
                left.visit(state)?;
                right.visit(state)?;
                Ok(())
            },
            Expression::Prefix { ref right, .. } => {
                right.visit(state)?;
                Ok(())
            },
            Expression::Postfix { ref left, .. } => {
                left.visit(state)?;
                Ok(())
            },
            Expression::Block(ref block) => {
                visit_block(block, state)
            },
            Expression::FnCall { name: ref ident, ref args } => {
                for ref arg in args.iter() {
                    arg.visit(state)?;
                }
                let id = ident.item.clone();
                match state.scope.borrow().resolve(&id) {
                    Some(Symbol::Function { ref params, .. }) => {
                        // verify that number of arguments matches number of parameters
                        if args.len() != params.len() {
                            state.logger.error(format!("invalid number of arguments for function \
                                '{}': expected {}, found {}", id, params.len(),
                                args.len()));
                        }
                        Ok(())
                    },
                    Some(_) => {
                        state.logger.error(format!("attempt to call non-function '{}' as function",
                            id));
                        Ok(())
                    }
                    None => {
                        state.logger.error(format!("function '{}' does not exist in scope",
                            id));
                        Ok(())
                    }
                }
            },
            Expression::IfElse { ref cond, ref if_block, ref else_block } => {
                cond.visit(state)?;
                visit_block(if_block, state)?;
                if let Some(ref else_block) = *else_block {
                    visit_block(else_block, state)?;
                }
                Ok(())
            },
            Expression::Loop { ref variant, ref set, ref body } => {
                // create new scope for loop variant and body
                state.scope = state.scope.push();
                set.visit(state)?;
                // define loop variant symbol
                match *variant {
                    Some(ref var) => {
                        state.scope.borrow_mut().define(var.item.clone(),
                            Symbol::variable(var.item.clone(), None));
                    },
                    None => {}
                }
                // visit body (in this scope)
                body.visit(state)?;
                // return to parent
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

impl SymbolDefineVisitor for Node<Set> {
    fn visit(&self, state: &mut State) -> Result {
        self.annotation.borrow_mut().set_scope(Some(Rc::clone(&state.scope)));
        match self.item {
            Set::Interval { ref start, ref end, ref step, .. } => {
                start.visit(state)?;
                end.visit(state)?;
                step.visit(state)?;
                Ok(())
            }
        }
    }
}
