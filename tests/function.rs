extern crate piske;

use piske::parse::program;
use piske::visitor::symbol::DefineSymbols;
use piske::visitor::type_visitor::ComputeTypes;
use piske::visitor::eval::Evaluate;

use piske::value::Value;

#[test]
fn test_function_one_param() {
    let prog = r#"
fn add5(a:int) -> int {
    a + 5
}
let b = 5;
add5(b)
    "#;

    let mut ast = program(prog).unwrap();
    ast.define_symbols().unwrap();
    ast.compute_types().unwrap();
    assert_eq!(ast.eval(), Ok(Value::Int(10)));
}

#[test]
fn test_function_two_params() {
    let prog = r#"
fn add(a: int, b: float) -> int {
    a + b
}
let b = 5.0;
add(2, b)
    "#;

    let mut ast = program(prog).unwrap();
    ast.define_symbols().unwrap();
    ast.compute_types().unwrap();
    assert_eq!(ast.eval(), Ok(Value::Float(7.0)));
}
