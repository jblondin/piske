extern crate piske;
extern crate sindra;

use sindra::PNode;

use piske::parse::{literal};
use piske::ast::ast::*;

#[test]
fn test_num() {
    assert_eq!(literal("0"), Ok(PNode::new(Literal::Int(0))));
    assert_eq!(literal("0.0"), Ok(PNode::new(Literal::Float(0.0))));

    assert_eq!(literal("4"), Ok(PNode::new(Literal::Int(4))));
    assert_eq!(literal("4.3"), Ok(PNode::new(Literal::Float(4.3))));

    assert_eq!(literal("4.3e2"), Ok(PNode::new(Literal::Float(430.0))));
    assert_eq!(literal("4.3e-2"), Ok(PNode::new(Literal::Float(0.043))));
    assert_eq!(literal("4.3E2"), Ok(PNode::new(Literal::Float(430.0))));
    assert_eq!(literal("4.3E-2"), Ok(PNode::new(Literal::Float(0.043))));

    assert_eq!(literal("43e2"), Ok(PNode::new(Literal::Float(4300.0))));
    assert_eq!(literal("43e-2"), Ok(PNode::new(Literal::Float(0.43))));
}

#[test]
fn test_str() {
    assert_eq!(literal(r#""foo""#), Ok(PNode::new(Literal::String("foo".to_string()))));
    assert_eq!(literal(r#""\x41\x2D\x5A""#), Ok(PNode::new(Literal::String("A-Z".to_string()))));
    assert_eq!(literal(r#""\u{263A}\u{2639}""#), Ok(PNode::new(Literal::String("☺☹".to_string()))));
    assert_eq!(literal(r#""☺☹""#), Ok(PNode::new(Literal::String("☺☹".to_string()))));
}
