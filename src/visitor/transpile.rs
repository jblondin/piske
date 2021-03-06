//! Transpiler abstract syntax tree visitor.
//!
//! This module contains the trait and implementation for walking an annotated abstract syntax tree
//! and translating it into Rust code.. This implementation expects that the symbol table and type
//! computation annotations already exist on the tree.

use quote::{Tokens, ToTokens, Ident};

use sindra::{Typed, Identifier, Node};
use sindra::scope::{SymbolStore, Scoped};

use symbol::Symbol;
use PType;
use ast::*;
use visitor::state::State;

type Result = ::std::result::Result<Tokens, String>;

/// Trait for transpiler visitor; implemented for all abstract syntax tree nodes.
pub trait TranspileVisitor {
    /// Walk the tree, translating and producing a result from the program.
    fn visit(&self, &mut State) -> Result;
}

impl TranspileVisitor for Node<Program> {
    fn visit(&self, state: &mut State) -> Result {
        let prog = self.item.0.visit(state)?;
        let pref = preface();
        let nl = nl();
        Ok(quote! {
#pref #nl

fn run() -> Result<(), String> { #nl
    #![allow(unused_mut, unused_variables)]
    let mut env = Environment::default(); #nl
    #prog
    ; Ok(())
}
        })
    }
}

fn raw(s: &str) -> Tokens {
    let mut t = Tokens::new();
    t.append(s);
    t
}
fn nl() -> Tokens {
    raw("\n")
}
fn loop_var_name(state: &State) -> Tokens {
    raw(&format!("loop_return_value_{}", state.loop_depth))
}
fn outer_loop_var_name(state: &State) -> Tokens {
    raw(&format!("loop_return_value_{}", state.loop_depth - 1))
}

impl TranspileVisitor for Node<Block> {
    fn visit(&self, state: &mut State) -> Result {
        let mut statements = vec![];
        let nl = nl();
        for statement in self.item.0.iter() {
            let qstatement = statement.visit(state)?;
            statements.push(quote! { #qstatement #nl });
        }
        Ok(quote! { #(#statements)* })
    }
}

impl TranspileVisitor for Node<Statement> {
    fn visit(&self, state: &mut State) -> Result {
        match (&self.item, &self.annotation) {
            (&Statement::Declare(ref ident, ref expr), _) => {
                let rhs = expr.visit(state)?;
                let lhs = ident.visit(state)?;
                Ok(quote! { let mut #lhs = #rhs; })
            },
            (&Statement::Assign(ref ident, ref expr), _) => {
                let rhs = expr.visit(state)?;
                let lhs = ident.visit(state)?;
                Ok(quote! { #lhs = #rhs; })
            },
            (&Statement::Expression(ref expr), _) => {
                let qexpr = expr.visit(state)?;
                // only add a semi-colon to expressions if the return type is void
                match expr.annotation.borrow().ty() {
                    Some(PType::Void) => Ok(quote! { #qexpr; }),
                    _ => Ok(quote! { #qexpr })
                }
            },
            (&Statement::FnDefine(FunctionDef { ref name, ref ret_type, ref params, ref body } ),
                    _) => {
                let mut qparams = vec![];
                for param in params {
                    qparams.push(param.visit(state)?);
                }
                let qname = name.visit(state)?;
                let qret_ty = ret_type.visit(state)?;
                let qbody = body.visit(state)?;
                Ok(quote! {
                    fn #qname(#(#qparams),*) -> #qret_ty {
                        #qbody
                    }
                })
            },
            (&Statement::Return(ref expr), _) => {
                let qexpr = expr.visit(state)?;
                Ok(quote! { return #qexpr; })
            }
            (&Statement::Break(ref expr), _) => {
                let qexpr = expr.visit(state)?;
                let loop_var_name = outer_loop_var_name(state);
                Ok(quote! { #loop_var_name = #qexpr; break; })
            },
            (&Statement::Print(ref exprs), _) => {
                let mut qexprs = vec![];
                let mut pattern = String::new();
                for expr in exprs {
                    qexprs.push(expr.visit(state)?);
                    pattern.push_str("{}");
                }
                //TODO: update this to use writeln! to an arbitrary output
                Ok(quote! { println!(#pattern, #(#qexprs),*); })
            }

        }

    }
}

impl TranspileVisitor for Node<Parameter> {
    fn visit(&self, state: &mut State) -> Result {
        let qname = self.item.name.visit(state)?;
        let qty = self.item.name.visit(state)?;
        Ok(quote ! { #qname: #qty })
    }
}

impl TranspileVisitor for Node<Expression> {
    fn visit(&self, state: &mut State) -> Result {
        match (&self.item, &self.annotation) {
            (&Expression::Literal(ref literal), ref annotation) => {
                let qlit = add_cast(literal.visit(state)?, annotation.borrow().ty(),
                    annotation.borrow().promote_type())?;
                Ok(quote! { #qlit })
            },
            (&Expression::Identifier(ref ident), ref annotation) => {
                let qident = add_cast(ident.visit(state)?, annotation.borrow().ty(),
                    annotation.borrow().promote_type())?;
                Ok(quote! { #qident })
            },
            (&Expression::Infix { ref op, ref left, ref right }, ref annotation) => {
                let infix_op = add_cast(infix_to_tokens(op, left, right, state)?,
                    annotation.borrow().ty(), annotation.borrow().promote_type())?;
                Ok(quote! { #infix_op })
            },
            (&Expression::Prefix { ref op, ref right }, ref annotation) => {
                let prefix_op = add_cast(prefix_to_tokens(op, right, state)?,
                    annotation.borrow().ty(), annotation.borrow().promote_type())?;
                Ok(quote! { #prefix_op })
            },
            (&Expression::Postfix { ref op, ref left }, ref annotation) => {
                let postfix_op = add_cast(postfix_to_tokens(op, left, state)?,
                    annotation.borrow().ty(), annotation.borrow().promote_type())?;
                Ok(quote! { #postfix_op })
            },
            (&Expression::Block(ref block), ref annotation) => {
                let qblock = block.visit(state)?;
                let braced_qblock = quote! { { #qblock } };
                add_cast(braced_qblock, annotation.borrow().ty(),
                    annotation.borrow().promote_type())
            },
            (&Expression::FnCall { ref name, ref args }, ref annotation) => {
                let mut qargs = vec![];
                let scope = annotation.borrow().scope().unwrap();
                let symbol: Option<Symbol> = scope.borrow().resolve(&name.item);
                let is_stdlib_call = if let Some(sym) = symbol {
                    sym.is_stdlib_func()
                } else {
                    false
                };
                if is_stdlib_call {
                    qargs.push(quote! { &mut env });
                }
                for arg in args {
                    qargs.push(arg.visit(state)?);
                }
                let qname = name.visit(state)?;
                let fn_call = if is_stdlib_call {
                    quote! { #qname(#(#qargs),*)? }
                } else {
                    quote! { #qname(#(#qargs),*) }
                };
                add_cast(fn_call, annotation.borrow().ty(), annotation.borrow().promote_type())
            },
            (&Expression::IfElse { ref cond, ref if_block, ref else_block }, ref annotation) => {
                let qcond = cond.visit(state)?;
                let qif = if_block.visit(state)?;
                let nl = nl();
                match *else_block {
                    Some(ref else_block) => {
                        let qelse = else_block.visit(state)?;
                        add_cast(quote! { if #qcond { #nl #qif } else { #nl #qelse } },
                            annotation.borrow().ty(), annotation.borrow().promote_type())
                    },
                    None => {
                        add_cast(quote! { if #qcond { #nl #qif } }, annotation.borrow().ty(),
                            annotation.borrow().promote_type())
                    }
                }
            },
            (&Expression::Loop { ref variant, ref set, ref body }, ref annotation) => {
                let nl = nl();
                let loop_var_name = loop_var_name(state);
                state.loop_depth += 1;
                let qset = set.visit(state)?;
                let qbody = body.visit(state)?;
                let qvar = match *variant {
                    Some(ref variant) => {
                        variant.visit(state)?
                    },
                    None => {
                        raw("_")
                    }
                };
                state.loop_depth -= 1;
                match start_value(annotation.borrow().ty().unwrap()) {
                    Some(start_value) => {
                        add_cast(quote! { {
                            let mut #loop_var_name = #start_value;
                            for #qvar in #qset { #nl #loop_var_name = { #nl #qbody }; }
                            #loop_var_name
                        } }, annotation.borrow().ty(), annotation.borrow().promote_type())
                    },
                    None => {
                        add_cast(quote! {
                            for #qvar in #qset { #nl #qbody }
                        }, annotation.borrow().ty(), annotation.borrow().promote_type())
                    }
                }
            },
        }
    }
}

fn start_value(ty: PType) -> Option<Tokens> {
    match ty {
        PType::String => Some(raw("String::new()")),
        PType::Float => Some(raw("0.0")),
        PType::Int => Some(raw("0")),
        PType::Boolean => Some(raw("false")),
        PType::Complex => Some(raw("Complex::new(0.0, 0.0)")),
        PType::Set | PType::Void => None,
    }
}

impl TranspileVisitor for Node<Set> {
    fn visit(&self, state: &mut State) -> Result {
        match self.item {
            Set::Interval { ref start, ref end, end_inclusive, ref step } => {
                let qstart = start.visit(state)?;
                let qend = end.visit(state)?;
                let qstep = step.visit(state)?;
                Ok(quote!{ StepRange::new(#qstart, #qend, #end_inclusive, #qstep) })
            }
        }
    }
}

impl TranspileVisitor for Node<Literal> {
    fn visit(&self, _: &mut State) -> Result {
        let item = &self.item;
        Ok(quote! { #item })
    }
}


impl TranspileVisitor for Node<Identifier> {
    fn visit(&self, _: &mut State) -> Result {
        let ident_name = Ident::new(self.item.0.clone());
        Ok(quote! { #ident_name })
    }
}

fn add_cast<T: ToTokens + ::std::fmt::Display>(elem: T, actual_ty: Option<PType>,
        promote_ty: Option<PType>) -> Result {
    match actual_ty {
        Some(_) => {
            match promote_ty {
                Some(promote_ty) => {
                    Ok(match promote_ty {
                        PType::String => { return Err(format!("invalid promotion to string")); },
                        PType::Float => quote! { (#elem as f64) },
                        PType::Int => quote! { (#elem as i64) },
                        PType::Boolean => quote! { #elem as bool },
                        PType::Complex => quote! { Complex::new((#elem) as f64, 0.0) },
                        PType::Set => { return Err(format!("invalid promotion to set")); },
                        PType::Void => { return Err(format!("invliad promotion to void")); },
                    })
                },
                None => Ok(quote! { #elem })
            }
        },
        None => Err(format!("no type found for {}", elem))
    }
}

fn infix_to_tokens(op: &InfixOp, left: &Node<Expression>, right: &Node<Expression>,
        state: &mut State) -> Result {
    let qleft = left.visit(state)?;
    let qright = right.visit(state)?;
    Ok(match *op {
        InfixOp::Add => { quote! { #qleft + #qright } },
        InfixOp::Subtract => { quote! { #qleft - #qright } },
        InfixOp::Multiply => { quote! { #qleft * #qright } },
        InfixOp::Divide => { quote! { #qleft / #qright } },
        InfixOp::Power => {
            match (left.annotation.borrow().promoted(), right.annotation.borrow().promoted()) {
                (Some(PType::Int), Some(PType::Int)) => quote! { #qleft.pow(#qright) },
                (Some(PType::Int), Some(PType::Float)) => {
                    return Err("integer raised to floating point power".to_string());
                },
                (Some(PType::Float), Some(PType::Int)) => quote! { #qleft.powi(#qright) },
                (Some(PType::Float), Some(PType::Float)) => quote! { #qleft.powf(#qright) },
                _ => { return Err("invalid exponentiation".to_string()) }
            }
        },
        InfixOp::Comparison(compare_op) => {
            match compare_op {
                CompareOp::LessThan => { quote! { #qleft < #qright } },
                CompareOp::LessThanEqual => { quote! { #qleft <= #qright } },
                CompareOp::GreaterThan => { quote! { #qleft > #qright } },
                CompareOp::GreaterThanEqual => { quote! { #qleft >= #qright } },
                CompareOp::Equal => { quote! { #qleft == #qright } },
                CompareOp::NotEqual => { quote! { #qleft != #qright } },
            }
        },
    })
}

fn prefix_to_tokens(op: &PrefixOp, right: &Node<Expression>, state: &mut State) -> Result {
    let qright = right.visit(state)?;
    Ok(match *op {
        PrefixOp::UnaryMinus => { quote! { -#qright } },
        PrefixOp::UnaryPlus => { quote! { #qright } },
    })
}

fn postfix_to_tokens(op: &PostfixOp, left: &Node<Expression>, state: &mut State) -> Result {
    let qleft = left.visit(state)?;
    Ok(match *op {
        PostfixOp::Imaginary => { quote! { Complex::new(0.0, (#qleft) as f64) } },
        PostfixOp::Conjugate => {
            match left.annotation.borrow().promoted() {
                Some(PType::Int) => { quote! { 1.0 / (#qleft as f64) } },
                Some(PType::Float) => { quote! { 1.0 / #qleft } },
                Some(PType::Complex) => { quote! { #qleft.conj() } },
                Some(ty) => { return Err(format!("invalid type for conjugation: {}", ty)) },
                _ => { return Err("missing type information".to_string()) }
            }
        }
    })
}

impl ToTokens for Literal {
    fn to_tokens(&self, tokens: &mut Tokens) {
        match *self {
            Literal::String(ref s) => { tokens.append(format!("\"{}\".to_string()", s)); },
            Literal::Float(f) => { tokens.append(format!("{}f64", f)); },
            Literal::Int(i) => { tokens.append(format!("{}i64", i)); },
            Literal::Boolean(b) => { tokens.append(format!("{}", b)); },
        }
    }
}

fn preface() -> Tokens {
    raw(r#"
#![allow(unused_imports)]

extern crate psk_std;

use std::io::Write;

use psk_std::step_range::StepRange;
use psk_std::stdlib::*;
use psk_std::complex::Complex;
use psk_std::Environment;

fn main() {
    if let Err(e) = run() {
        writeln!(::std::io::stderr(), "ERROR: {}", e).unwrap();
        ::std::process::exit(1);
    }
}

"#)
}
