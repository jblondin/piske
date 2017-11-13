//! Evaluation abstract syntax tree visitor.
//!
//! This module contains the trait and implementation for walking an annotated abstract syntax tree
//! and evaluating it. This implementation expects that the symbol table and type computation
//! annotations already exist on the tree.

use std::cell::RefCell;
use std::rc::Rc;

use sindra::Node;
use sindra::Typed;
use sindra::scope::{SymbolStore, MemoryStore, MemoryScope, Scoped, Stack};
use sindra::operator::{UnaryOperator, BinaryOperator};
use sindra::value::Coerce;

use ast::*;
use Symbol;
use value::Value;
use visitor::State;

type Result = ::std::result::Result<Value, String>;
type Scope = MemoryScope<Symbol, Value>;

/// Trait to provide easy entry point method to evaluation visitor.
pub trait Evaluate {
    /// Evaluate entry point method. Handles setting up the state and initiating the tree walk.
    fn eval(&mut self) -> Result;
}
impl Evaluate for Node<Program> where Rc<RefCell<Scope>>: Stack {
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

impl EvaluateVisitor for Node<Program> where Rc<RefCell<Scope>>: Stack {
    fn visit(&mut self, state: &mut State) -> Result {
        self.item.0.borrow_mut().visit(state)
    }
}

impl EvaluateVisitor for Node<Block> where Rc<RefCell<Scope>>: Stack {
    fn visit(&mut self, state: &mut State) -> Result {
        let mut last_result: Value = Value::Empty;
        for statement in self.item.0.iter_mut() {
            last_result = statement.borrow_mut().visit(state)?;
        }
        Ok(last_result)
    }
}

impl EvaluateVisitor for Node<Statement> {
    fn visit(&mut self, state: &mut State) -> Result {
        match (&mut self.item, &mut self.annotation) {
            (&mut Statement::Declare(ref ident, ref mut expr), &mut ref mut annotation) => {
                match (annotation as &Scoped<Scope>).scope() {
                    Some(ref mut scope) => {
                        let value = expr.borrow_mut().visit(state)?;
                        scope.borrow_mut().set(ident.borrow().item.clone(), value.clone())?;
                        Ok(value)
                    },
                   None => Err("no associated scope in declaration statement".to_string())
                }
            },
            (&mut Statement::Assign(ref ident, ref mut expr), &mut ref mut annotation) => {
                match (annotation as &Scoped<Scope>).scope() {
                    Some(ref mut scope) => {
                        let value = expr.borrow_mut().visit(state)?;
                        scope.borrow_mut().set(ident.borrow().item.clone(), value.clone())?;
                        Ok(value)
                    },
                    None => Err("no associated scope in assignment statement".to_string())
                }
            },
            (&mut Statement::Expression(ref mut expr), _) => {
                expr.borrow_mut().visit(state)
            },
            (&mut Statement::FnDefine { .. }, _) => {
                // Err("unimplemented".to_string())
                Ok(Value::Empty)
            }
        }
    }
}

impl EvaluateVisitor for Node<Expression> {
    fn visit(&mut self, state: &mut State) -> Result {
        match (&mut self.item, &mut self.annotation) {
            (&mut Expression::Literal(ref literal), _) => {
                Ok(Value::from(literal.borrow().item.clone()))
            },
            (&mut Expression::Identifier(ref ident), &mut ref mut annotation) => {
                let ident = &ident.borrow().item;
                match annotation.scope() {
                    Some(ref scope) => {
                        match scope.borrow().get(&ident) {
                            Some(ref value) => Ok(value.clone()),
                            None => Err(format!("uninitialized variable: {}", ident))
                        }
                    },
                    None => Err("invalid scope".to_string())
                }
            },
            (&mut Expression::Infix { ref op, ref mut left, ref mut right },
                    &mut ref mut annotation) => {
                let lval = left.borrow_mut().visit(state)?;
                let rval = right.borrow_mut().visit(state)?;

                op.op(
                    annotation.ty().unwrap(),
                    &lval.coerce(left.borrow().annotation.promote_type()),
                    &rval.coerce(right.borrow().annotation.promote_type())
                )
            },
            (&mut Expression::Prefix { ref op, ref mut right }, &mut ref mut annotation) => {
                let rval = right.borrow_mut().visit(state)?;

                op.op(
                    annotation.ty().unwrap(),
                    &rval.coerce(right.borrow().annotation.promote_type())
                )
            },
            (&mut Expression::Postfix { ref op, ref mut left }, &mut ref mut annotation) => {
                let lval = left.borrow_mut().visit(state)?;

                op.op(
                    annotation.ty().unwrap(),
                    &lval.coerce(left.borrow().annotation.promote_type())
                )
            },
            (&mut Expression::Block(ref mut block), _) => {
                block.borrow_mut().visit(state)
            }
            (&mut Expression::FnCall { ref name, ref mut args, .. } , &mut ref mut annotation) => {
                let mut evaluated_args = vec![];
                for arg in args {
                    evaluated_args.push(arg.borrow_mut().visit(state)?);
                }

                let scope = annotation.scope().ok_or(format!("invalid scope when calling funciton \
                    '{}'", name.borrow().item))?;

                let mut sym: Symbol = scope.borrow().resolve(&name.borrow().item).ok_or(format!(
                    "unintialized variable '{}'", name.borrow().item))?;

                match sym {
                    Symbol::Function { ref name, ref mut body, ref params, .. } => {
                        let fn_scope = body.borrow().annotation.scope().ok_or(
                            format!("missing function scope for function \
                                '{}'", name)
                        )?;

                        // establish arguments as parameters in new scope
                        for (i, arg) in evaluated_args.iter().enumerate() {
                            let param = &params[i].borrow();
                            let pname = param.item.name.borrow().item.clone();
                            fn_scope.borrow_mut().set(pname.clone(), arg.clone())?;
                        }

                        let prev_scope = Rc::clone(&state.scope);
                        state.scope = fn_scope;
                        // evaluate body
                        let val = body.borrow_mut().visit(state)?;
                        // reset scope
                        state.scope = Rc::clone(&prev_scope);
                        // return result
                        Ok(val)
                    }
                    _ => Err(format!("unable to call symbol '{}' as function",
                        name.borrow().item))
                }
            }
        }
    }
}
