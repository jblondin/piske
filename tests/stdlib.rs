extern crate piske;

use piske::parse::program;
use piske::visitor::{State, SymbolDefineVisitor, TypeComputationVisitor, EvaluateVisitor};
use piske::value::Value;

fn expect_prog(prog: &str, val: Value) {
    let ast = program(prog).unwrap();
    let mut state = State::default();
    SymbolDefineVisitor::visit(&ast, &mut state).unwrap();
    TypeComputationVisitor::visit(&ast, &mut state).unwrap();
    let evaluated = EvaluateVisitor::visit(&ast, &mut state);
    assert_eq!(evaluated, Ok(val));
}

#[test]
fn test_image_dims() {
    let prog = r#"
set_image_dims(50, 60);
get_image_height()
    "#;
    expect_prog(prog, Value::Int(50));

    let prog = r#"
set_image_dims(50, 60);
get_image_width()
    "#;
    expect_prog(prog, Value::Int(60));
}

#[test]
fn test_print() {
    let prog = r#"
print_int(5);
    "#;
    expect_prog(prog, Value::Empty);
}
