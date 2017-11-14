extern crate piske;
extern crate sindra;

use sindra::Node;

use piske::ast::Program;
use piske::parse::program;
use piske::visitor::eval::Evaluate;
use piske::visitor::symbol::DefineSymbols;
use piske::visitor::type_visitor::ComputeTypes;
use piske::value::Value;

fn parse_annotate(prog: &str) -> Node<Program> {
    let node = program(prog).unwrap();
    node.define_symbols().unwrap();
    node.compute_types().unwrap();
    node
}

#[test]
fn test_eval_assign() {
    let prog = r#"
let a = 4;
a = 3;
a
    "#;
    let node = parse_annotate(prog);
    assert_eq!(node.eval(), Ok(Value::Int(3)));
}

#[test]
fn test_eval_add() {
    let prog = r#"
let a = 4;
a = a + 3;
a
    "#;
    let node = parse_annotate(prog);
    assert_eq!(node.eval(), Ok(Value::Int(7)));
}


#[test]
fn test_eval_add_mixed() {
    let prog = r#"4 + 3.4"#;
    let node = parse_annotate(prog);
    assert_eq!(node.eval(), Ok(Value::Float(7.4)));
}

#[test]
fn test_eval_raise() {
    let prog = r#"2^3"#;
    let node = parse_annotate(prog);
    assert_eq!(node.eval(), Ok(Value::Float(8.0)));
}

#[test]
fn test_eval_divide() {
    let prog = r#"7 / 2"#;
    let node = parse_annotate(prog);
    assert_eq!(node.eval(), Ok(Value::Int(3)));

    let prog = r#"7.0 / 2"#;
    let node = parse_annotate(prog);
    assert_eq!(node.eval(), Ok(Value::Float(3.5)));
}

#[test]
fn test_eval_conjugate() {
    let prog = r#"7`"#;
    let node = parse_annotate(prog);
    assert_eq!(node.eval(), Ok(Value::Float(1.0 / 7.0)));

    let prog = r#"7.0`"#;
    let node = parse_annotate(prog);
    assert_eq!(node.eval(), Ok(Value::Float(1.0 / 7.0)));
}
