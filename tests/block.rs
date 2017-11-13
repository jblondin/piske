extern crate piske;
extern crate sindra;

use sindra::{PNode, Identifier};

use piske::parse::{block, program};
use piske::ast::ast::*;

include!("macros.rs");

#[test]
fn test_block() {
    assert_eq!(block("4; 5"), Ok(PNode::new(Block(vec![expr_stmt!(int!(4)), expr_stmt!(int!(5))]))));
    assert_eq!(block("4; 5;"), Ok(PNode::new(Block(vec![expr_stmt!(int!(4)),
        expr_stmt!(int!(5))]))));
    assert_eq!(block("4 5;"), Ok(PNode::new(Block(vec![expr_stmt!(int!(4)), expr_stmt!(int!(5))]))));
    assert_eq!(block("4 5"), Ok(PNode::new(Block(vec![expr_stmt!(int!(4)), expr_stmt!(int!(5))]))));

    assert_eq!(block("4 + 5; let a = 4; 5"), Ok(PNode::new(Block(vec![
        expr_stmt!(add!(int!(4), int!(5))),
        decl_stmt!(ident!(a), int!(4)),
        expr_stmt!(int!(5))
    ]))));

    assert_eq!(block("let a = 5; a = 4; a"), Ok(PNode::new(Block(vec![
        decl_stmt!(ident!(a), int!(5)),
        assign_stmt!(ident!(a), int!(4)),
        expr_stmt!(ident_expr!(ident!(a)))
    ]))));
}

#[test]
fn test_program() {
    assert_eq!(program("let a = 5; a = 4; a"), Ok(PNode::new(Program(PNode::new(Block(vec![
        decl_stmt!(ident!(a), int!(5)),
        assign_stmt!(ident!(a), int!(4)),
        expr_stmt!(ident_expr!(ident!(a)))
    ]))))));
}

#[test]
fn test_multiline() {
    let prog = r#"
let a = 5;
a = 4;
a
    "#;
    assert_eq!(program(prog), Ok(PNode::new(Program(PNode::new(Block(vec![
        decl_stmt!(ident!(a), int!(5)),
        assign_stmt!(ident!(a), int!(4)),
        expr_stmt!(ident_expr!(ident!(a)))
    ]))))));
}
