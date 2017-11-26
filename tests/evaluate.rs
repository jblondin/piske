extern crate piske;
extern crate sindra;

use piske::parse::program;
use piske::visitor::{State, SymbolDefineVisitor, TypeComputationVisitor, EvaluateVisitor};
use piske::value::Value;

fn eval(prog: &str) -> Result<Value, String> {
    let node = program(prog).unwrap();
    let mut state = State::default();
    SymbolDefineVisitor::visit(&node, &mut state)?;
    TypeComputationVisitor::visit(&node, &mut state)?;
    EvaluateVisitor::visit(&node, &mut state)
}

#[test]
fn test_eval_assign() {
    let prog = r#"
let a = 4;
a = 3;
a
    "#;
    assert_eq!(eval(prog), Ok(Value::Int(3)));
}

#[test]
fn test_eval_add() {
    let prog = r#"
let a = 4;
a = a + 3;
a
    "#;
    assert_eq!(eval(prog), Ok(Value::Int(7)));
}


#[test]
fn test_eval_add_mixed() {
    let prog = r#"4 + 3.4"#;
    assert_eq!(eval(prog), Ok(Value::Float(7.4)));
}

#[test]
fn test_eval_raise() {
    let prog = r#"2^3"#;
    assert_eq!(eval(prog), Ok(Value::Float(8.0)));
}

#[test]
fn test_eval_divide() {
    let prog = r#"7 / 2"#;
    assert_eq!(eval(prog), Ok(Value::Int(3)));

    let prog = r#"7.0 / 2"#;
    assert_eq!(eval(prog), Ok(Value::Float(3.5)));
}

#[test]
fn test_eval_conjugate() {
    let prog = r#"7`"#;
    assert_eq!(eval(prog), Ok(Value::Float(1.0 / 7.0)));

    let prog = r#"7.0`"#;
    assert_eq!(eval(prog), Ok(Value::Float(1.0 / 7.0)));
}
