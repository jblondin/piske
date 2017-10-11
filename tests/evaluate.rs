extern crate piske;
extern crate sindra;

use sindra::Node;

use piske::ast::{Program, MemAnnotation};
use piske::interp_parse::program;
use piske::visitor::eval::eval;
use piske::visitor::symbol::define_symbols;
use piske::visitor::type_visitor::compute_types;
use piske::value::Value;

fn parse_annotate(prog: &str) -> Node<Program<MemAnnotation>, MemAnnotation> {
    let mut node = program(prog).unwrap();
    define_symbols(&mut node).unwrap();
    compute_types(&mut node).unwrap();
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
    assert_eq!(eval(&mut node), Ok(Value::Int(3)));
}

#[test]
fn test_eval_add() {
    let prog = r#"
let a = 4;
a = a + 3;
a
    "#;
    let mut node = parse_annotate(prog);
    assert_eq!(eval(&mut node), Ok(Value::Int(7)));
}


#[test]
fn test_eval_add_mixed() {
    let prog = r#"4 + 3.4"#;
    let mut node = parse_annotate(prog);
    assert_eq!(eval(&mut node), Ok(Value::Float(7.4)));
}

#[test]
fn test_eval_raise() {
    let prog = r#"2^3"#;
    let mut node = parse_annotate(prog);
    assert_eq!(eval(&mut node), Ok(Value::Float(8.0)));
}

#[test]
fn test_eval_divide() {
    let prog = r#"7 / 2"#;
    let mut node = parse_annotate(prog);
    assert_eq!(eval(&mut node), Ok(Value::Int(3)));

    let prog = r#"7.0 / 2"#;
    let mut node = parse_annotate(prog);
    assert_eq!(eval(&mut node), Ok(Value::Float(3.5)));
}

#[test]
fn test_eval_conjugate() {
    let prog = r#"7`"#;
    let mut node = parse_annotate(prog);
    assert_eq!(eval(&mut node), Ok(Value::Float(1.0 / 7.0)));

    let prog = r#"7.0`"#;
    let mut node = parse_annotate(prog);
    assert_eq!(eval(&mut node), Ok(Value::Float(1.0 / 7.0)));
}
