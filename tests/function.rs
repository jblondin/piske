extern crate piske;

use piske::parse::program;
use piske::visitor::{State, SymbolDefineVisitor, TypeComputationVisitor, EvaluateVisitor};

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

    let ast = program(prog).unwrap();
    let mut state = State::default();
    SymbolDefineVisitor::visit(&ast, &mut state).unwrap();
    TypeComputationVisitor::visit(&ast, &mut state).unwrap();
    assert_eq!(EvaluateVisitor::visit(&ast, &mut state), Ok(Value::Int(10)));
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

    let ast = program(prog).unwrap();
    let mut state = State::default();
    SymbolDefineVisitor::visit(&ast, &mut state).unwrap();
    TypeComputationVisitor::visit(&ast, &mut state).unwrap();
    assert_eq!(EvaluateVisitor::visit(&ast, &mut state), Ok(Value::Float(7.0)));
}

#[test]
fn test_return() {
    let prog = r#"
fn greater_than_five(a: int) -> bool {
    if a > 5 {
        return true;
    }
    false
}
greater_than_five(6)
    "#;

    let ast = program(prog).unwrap();
    let mut state = State::default();
    SymbolDefineVisitor::visit(&ast, &mut state).unwrap();
    TypeComputationVisitor::visit(&ast, &mut state).unwrap();
    assert_eq!(EvaluateVisitor::visit(&ast, &mut state), Ok(Value::Boolean(true)));

    let prog = r#"
fn greater_than_five(a: int) -> bool {
    if a > 5 {
        return true;
    }
    false
}
greater_than_five(5)
    "#;

    let ast = program(prog).unwrap();
    let mut state = State::default();
    SymbolDefineVisitor::visit(&ast, &mut state).unwrap();
    TypeComputationVisitor::visit(&ast, &mut state).unwrap();
    assert_eq!(EvaluateVisitor::visit(&ast, &mut state), Ok(Value::Boolean(false)));

}
