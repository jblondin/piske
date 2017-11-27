//! Evaluation abstract syntax tree visitor.
//!
//! This module contains the trait and implementation for walking an annotated abstract syntax tree
//! and evaluating it. This implementation expects that the symbol table and type computation
//! annotations already exist on the tree.

use std::rc::Rc;

use sindra::Node;
use sindra::Typed;
use sindra::scope::{SymbolStore, MemoryStore, Scoped};
use sindra::operator::{UnaryOperator, BinaryOperator};
use sindra::value::Coerce;

use ast::*;
use Symbol;
use symbol::FunctionBody;
use value::{Value, ValueSet, SetInterval};
use visitor::State;

type Result = ::std::result::Result<Value, String>;

/// Trait for evaluation visitor; implemented for all abstract syntax tree nodes.
pub trait EvaluateVisitor {
    /// Walk the tree, evaluating and producing a result from the program.
    fn visit(&self, &mut State) -> Result;
}

impl EvaluateVisitor for Node<Program> {
    fn visit(&self, state: &mut State) -> Result {
        self.item.0.visit(state)
    }
}

impl EvaluateVisitor for Node<Block> {
    fn visit(&self, state: &mut State) -> Result {
        let mut last_result: Value = Value::Empty;
        for statement in self.item.0.iter() {
            last_result = statement.visit(state)?;
            // if we have a return / break value, short-circuit this block and return it
            match last_result {
                Value::Return(_) | Value::Break(_) => { return Ok(last_result); }
                _ => {}
            }
        }
        Ok(last_result)
    }
}

impl EvaluateVisitor for Node<Statement> {
    fn visit(&self, state: &mut State) -> Result {
        match (&self.item, &self.annotation) {
            (&Statement::Declare(ref ident, ref expr), &ref annotation) => {
                match annotation.borrow().scope() {
                    Some(ref scope) => {
                        let value = expr.visit(state)?;
                        scope.borrow_mut().set(ident.item.clone(), value.clone())?;
                        Ok(value)
                    },
                   None => Err("no associated scope in declaration statement".to_string())
                }
            },
            (&Statement::Assign(ref ident, ref expr), &ref annotation) => {
                match annotation.borrow().scope() {
                    Some(ref scope) => {
                        let value = expr.visit(state)?;
                        scope.borrow_mut().set(ident.item.clone(), value.clone())?;
                        Ok(value)
                    },
                    None => Err("no associated scope in assignment statement".to_string())
                }
            },
            (&Statement::Expression(ref expr), _) => {
                expr.visit(state)
            },
            (&Statement::FnDefine { .. }, _) => {
                // nothing to evaluate for function definitions
                Ok(Value::Empty)
            },
            (&Statement::Return(ref expr), _) => {
                Ok(Value::Return(Box::new(expr.visit(state)?)))
            },
            (&Statement::Break(ref expr), _) => {
                Ok(Value::Break(Box::new(expr.visit(state)?)))
            },
            (&Statement::Print(ref exprs), _) => {
                for expr in exprs {
                    let value = expr.visit(state)?;
                    write!(&mut state.std_env.stdout, "{}", value).unwrap();
                }
                writeln!(&mut state.std_env.stdout, "").unwrap();
                Ok(Value::Empty)
            }
        }
    }
}

impl EvaluateVisitor for Node<Expression> {
    fn visit(&self, state: &mut State) -> Result {
        match (&self.item, &self.annotation) {
            (&Expression::Literal(ref literal), _) => {
                Ok(Value::from(literal.item.clone()))
            },
            (&Expression::Identifier(ref ident), &ref annotation) => {
                let ident = &ident.item;
                match annotation.borrow().scope() {
                    Some(ref scope) => {
                        match scope.borrow().get(&ident) {
                            Some(ref value) => Ok(value.clone()),
                            None => Err(format!("uninitialized variable: {}", ident))
                        }
                    },
                    None => Err("invalid scope".to_string())
                }
            },
            (&Expression::Infix { ref op, ref left, ref right },
                    &ref annotation) => {
                let lval = left.visit(state)?;
                let rval = right.visit(state)?;

                op.op(
                    annotation.borrow().ty().unwrap(),
                    &lval.coerce(left.annotation.borrow().promote_type()),
                    &rval.coerce(right.annotation.borrow().promote_type())
                )
            },
            (&Expression::Prefix { ref op, ref right }, &ref annotation) => {
                let rval = right.visit(state)?;

                op.op(
                    annotation.borrow().ty().unwrap(),
                    &rval.coerce(right.annotation.borrow().promote_type())
                )
            },
            (&Expression::Postfix { ref op, ref left }, &ref annotation) => {
                let lval = left.visit(state)?;

                op.op(
                    annotation.borrow().ty().unwrap(),
                    &lval.coerce(left.annotation.borrow().promote_type())
                )
            },
            (&Expression::Block(ref block), _) => {
                block.visit(state)
            }
            (&Expression::FnCall { ref name, ref args, .. } , &ref annotation) => {
                let mut evaluated_args = vec![];
                for arg in args {
                    evaluated_args.push(arg.visit(state)?);
                }

                let scope = annotation.borrow().scope().ok_or(format!(
                    "invalid scope when calling function \
                    '{}'", name.item))?;

                let sym: Symbol = scope.borrow().resolve(&name.item).ok_or(format!(
                    "symbol not found: '{}'", name.item))?;

                match sym {
                    Symbol::Function { ref name, body: FunctionBody::Ast(ref body),
                            ref params, .. } => {
                        let fn_scope = body.annotation.borrow().scope().ok_or(
                            format!("missing function scope for function \
                                '{}'", name)
                        )?;

                        // establish arguments as parameters in new scope
                        for (i, arg) in evaluated_args.iter().enumerate() {
                            let param = &params[i];
                            let pname = param.item.name.item.clone();
                            fn_scope.borrow_mut().set(pname.clone(), arg.clone())?;
                        }

                        let prev_scope = Rc::clone(&state.scope);
                        state.scope = fn_scope;
                        // evaluate body, and handle possible return values (by unwrapping them)
                        let val = match body.visit(state)? {
                            Value::Return(returned_val) => {
                                *returned_val
                            },
                            val => val,
                        };

                        // reset scope
                        state.scope = Rc::clone(&prev_scope);
                        // return result
                        Ok(val)
                    },
                    Symbol::Function { body: FunctionBody::External(ext_func_id), .. } => {
                        state.std_env.call(ext_func_id, evaluated_args)
                    },
                    _ => Err(format!("unable to call symbol '{}' as function", name.item))
                }
            }
            (&Expression::IfElse { ref cond, ref if_block, ref else_block }, _) => {
                match cond.visit(state)? {
                    Value::Boolean(b) => {
                        if b {
                            if_block.visit(state)
                        } else {
                            if let Some(ref eblock) = *else_block {
                                eblock.visit(state)
                            } else {
                                // only if block exists; thus no value is generated
                                Ok(Value::Empty)
                            }
                        }
                    },
                    _ => Err(format!("conditional expression expected to be boolean"))
                }
            }
            (&Expression::Loop { ref variant, ref set, ref body }, _) => {
                let value_set = match set.visit(state)? {
                    Value::Set(value_set) => value_set,
                    _ => { return Err("loop specification did not evaluate as a set".to_string()); }
                };
                let mut val = Value::Empty;
                for elem in value_set.iter()? {
                    if let Some(scope) = body.annotation.borrow().scope() {
                        match *variant {
                            Some(ref var) => {
                                scope.borrow_mut().set(var.item.clone(), elem.clone())?;
                            },
                            None => {}
                        }
                    } else {
                        return Err(
                            "missing scope when trying to evaluate loop".to_string());
                    }
                    val = match body.visit(state)? {
                        Value::Break(returned_val) => {
                            *returned_val
                        },
                        v => v
                    }
                }
                Ok(val)
            }
        }
    }
}

impl EvaluateVisitor for Node<Set> {
    fn visit(&self, state: &mut State) -> Result {
        match self.item {
            Set::Interval { ref start, ref end, end_inclusive, ref step } => {
                Ok(Value::Set(Box::new(ValueSet::Interval(SetInterval {
                    start: start.visit(state)?,
                    end: end.visit(state)?,
                    end_inclusive: end_inclusive,
                    step: step.visit(state)?,
                }))))
            }
        }
    }
}
