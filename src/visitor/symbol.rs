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
    fn define_symbols(&mut self) -> Result;
}
impl DefineSymbols for Node<Program> {
    fn define_symbols(&mut self) -> Result {
        let mut state = State::default();
        let res = self.visit(&mut state);
        res
    }
}

/// Trait for symbol definition visitor; implemented for all abstract syntax tree nodes.
pub trait SymbolDefineVisitor {
    /// Verify and populate symbol table symbols for this node, and visit any children.
    fn visit(&mut self, &mut State) -> Result;
}


// impl<T, Sc> Visitor<Sc> for T where T: SymbolDefineVisitor<Sc> {
//     fn visit(&mut self, state: &mut State<Sc>) -> Result {
//         self.visit(state)
//     }
// }

impl SymbolDefineVisitor for Node<Program> {
    fn visit(&mut self, state: &mut State) -> Result {
        // define builtins in top-level (global) scope
        state.define_builtins();
        state.scope = state.scope.push();
        self.item.0.borrow_mut().visit(state)?;
        match state.scope.pop() {
            Some(parent_scope) => { state.scope = parent_scope; }
            None => {
                return Err("invalid descoping".to_string());
            }
        }
        self.annotation.set_scope(Some(Rc::clone(&state.scope)));
        Ok(())
    }
}

impl SymbolDefineVisitor for Node<Block> {
    fn visit(&mut self, state: &mut State) -> Result {
        for statement in self.item.0.iter_mut() {
            statement.borrow_mut().visit(state)?;
        }
        self.annotation.set_scope(Some(Rc::clone(&state.scope)));
        Ok(())
    }
}

impl SymbolDefineVisitor for Node<Statement> {
    fn visit(&mut self, state: &mut State) -> Result {
        self.annotation.set_scope(Some(Rc::clone(&state.scope)));
        match self.item {
            Statement::Declare(ref id, ref mut expr) => {
                expr.borrow_mut().visit(state)?;
                let id = id.borrow().item.clone();
                state.scope.borrow_mut().define(id.clone(),
                    Symbol::variable(id.clone(), None));
                Ok(())
            },
            Statement::Assign(ref id, ref mut expr) => {
                expr.borrow_mut().visit(state)?;
                let id = id.borrow().item.clone();
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
            Statement::Expression(ref mut expr) => {
                expr.borrow_mut().visit(state)
            },
            Statement::FnDefine(FunctionDef { ref name, ref mut body, ref params, .. }) => {
                // make sure function definition is at top scope
                let parent = state.scope.peek();
                if parent.is_some() && Rc::ptr_eq(&parent.unwrap(), &state.global) {
                    let prev_scope = Rc::clone(&state.scope);
                    // create new branch of scope tree under global
                    state.scope = state.global.push();
                    // define parameters in the new scope
                    for param in params.iter() {
                        let param = param.borrow();
                        let param_name = param.item.name.borrow().item.clone();
                        state.scope.borrow_mut().define(param_name.clone(),
                            Symbol::variable(param_name.clone(), None));
                    }
                    // define symbols in the body of the function
                    body.borrow_mut().visit(state)?;
                    // return to previous top-level scope
                    state.scope = prev_scope;
                    // add function symbol to scope
                    state.scope.borrow_mut().define(name.borrow().item.clone(),
                        Symbol::function(name.borrow().item.clone(), None, body.clone(),
                            params.clone()));
                    Ok(())
                } else {
                    state.logger.error(format!("function definition '{}' only allowed at \
                        global scope", name.borrow().item));
                    Ok(())
                }
            }
        }
    }
}

impl SymbolDefineVisitor for Node<Expression> {
    fn visit(&mut self, state: &mut State) -> Result {
        self.annotation.set_scope(Some(Rc::clone(&state.scope)));
        match self.item {
            Expression::Literal(_) => {
                Ok(())
            },
            Expression::Identifier(ref id) => {
                let sym: Option<Symbol> = state.scope.borrow().resolve(&id.borrow().item);
                match sym {
                    Some(_) => Ok(()),
                    None => {
                        state.logger.error(format!("symbol '{}' does not exist in scope",
                            id.borrow().item));
                        Ok(())
                    }
                }
            },
            Expression::Infix { ref mut left, ref mut right, .. } => {
                left.borrow_mut().visit(state)?;
                right.borrow_mut().visit(state)?;
                Ok(())
            },
            Expression::Prefix { ref mut right, .. } => {
                right.borrow_mut().visit(state)?;
                Ok(())
            },
            Expression::Postfix { ref mut left, .. } => {
                left.borrow_mut().visit(state)?;
                Ok(())
            },
            Expression::Block(ref mut block) => {
                state.scope = state.scope.push();
                block.borrow_mut().visit(state)?;
                match state.scope.pop() {
                    Some(parent_scope) => { state.scope = parent_scope; }
                    None => {
                        return Err("invalid descoping".to_string());
                    }
                }
                Ok(())
            },
            Expression::FnCall { name: ref ident, ref mut args } => {
                for ref mut arg in args.iter_mut() {
                    arg.borrow_mut().visit(state)?;
                }
                let id = ident.borrow().item.clone();
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
            }
        }
    }
}
