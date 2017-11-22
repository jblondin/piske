extern crate piske;

use piske::parse::program;
use piske::visitor::symbol::DefineSymbols;
use piske::visitor::type_visitor::ComputeTypes;
use piske::visitor::eval::Evaluate;
use piske::value::Value;

fn expect_prog(prog: &str, val: Value) {
    let ast = program(prog).unwrap();
    ast.define_symbols().unwrap();
    ast.compute_types().unwrap();
    assert_eq!(ast.eval(), Ok(val));
}

#[test]
fn test_if() {
    let prog = r#"
let a = 5;
if true {
    a = 10;
}
a
    "#;

    expect_prog(prog, Value::Int(10));
}

#[test]
fn test_cond() {
    expect_prog(r#"let a = 5; a == 5"#, Value::Boolean(true));
    expect_prog(r#"let a = 5.0; a == 5.0"#, Value::Boolean(true));
    expect_prog(r#"let a = 5.001; a >= 5"#, Value::Boolean(true));
    expect_prog(r#"let a = 5.001; a <= 5"#, Value::Boolean(false));
}

#[test]
fn test_if_cond() {
    let prog = r#"
let a = 5;
if a < 10 {
    a = 10;
}
if a > 10 {
    a = 15;
}
a
    "#;

    expect_prog(prog, Value::Int(10));
}

#[test]
fn test_ifelse() {
    let prog = r#"
let a = 10;
if a < 10 {
    a = 5
} else {
    a = 100
}
a
    "#;

    expect_prog(prog, Value::Int(100));
}

#[test]
fn test_if_expr() {
    let prog = r#"
let a = 5.0;
let b = if a < 5.1 {
    100
} else {
    10
}
b
    "#;
    expect_prog(prog, Value::Int(100));
}
