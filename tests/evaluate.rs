extern crate piske;
extern crate sindra;

use sindra::PNode;

use piske::ast::Program;
use piske::parse::program;
use piske::visitor::eval::Evaluate;
use piske::visitor::symbol::DefineSymbols;
use piske::visitor::type_visitor::ComputeTypes;
use piske::value::Value;

fn parse_annotate(prog: &str) -> PNode<Program> {
    let mut node = program(prog).unwrap();
    node.borrow_mut().define_symbols().unwrap();
    node.borrow_mut().compute_types().unwrap();
    node
}

#[test]
fn test_eval_assign() {
    let prog = r#"
let a = 4;
a = 3;
a
    "#;
    let mut node = parse_annotate(prog);
    assert_eq!(node.borrow_mut().eval(), Ok(Value::Int(3)));
}

#[test]
fn test_eval_add() {
    let prog = r#"
let a = 4;
a = a + 3;
a
    "#;
    let mut node = parse_annotate(prog);
    assert_eq!(node.borrow_mut().eval(), Ok(Value::Int(7)));
}


#[test]
fn test_eval_add_mixed() {
    let prog = r#"4 + 3.4"#;
    let mut node = parse_annotate(prog);
    assert_eq!(node.borrow_mut().eval(), Ok(Value::Float(7.4)));
}

#[test]
fn test_eval_raise() {
    let prog = r#"2^3"#;
    let mut node = parse_annotate(prog);
    assert_eq!(node.borrow_mut().eval(), Ok(Value::Float(8.0)));
}

#[test]
fn test_eval_divide() {
    let prog = r#"7 / 2"#;
    let mut node = parse_annotate(prog);
    assert_eq!(node.borrow_mut().eval(), Ok(Value::Int(3)));

    let prog = r#"7.0 / 2"#;
    let mut node = parse_annotate(prog);
    assert_eq!(node.borrow_mut().eval(), Ok(Value::Float(3.5)));
}

#[test]
fn test_eval_conjugate() {
    let prog = r#"7`"#;
    let mut node = parse_annotate(prog);
    assert_eq!(node.borrow_mut().eval(), Ok(Value::Float(1.0 / 7.0)));

    let prog = r#"7.0`"#;
    let mut node = parse_annotate(prog);
    assert_eq!(node.borrow_mut().eval(), Ok(Value::Float(1.0 / 7.0)));
}
