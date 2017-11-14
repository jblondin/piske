extern crate piske;
extern crate sindra;

use sindra::{Node, Identifier};

use piske::parse::statement;
use piske::ast::ast::*;


include!("macros.rs");

#[test]
fn test_expr_stmt() {
    assert_eq!(statement("2 + 3"), Ok(Node::new(Statement::Expression(add!(int!(2), int!(3))))));
    assert_eq!(statement("2 + 3;"), Ok(Node::new(Statement::Expression(add!(int!(2), int!(3))))));
    assert_eq!(statement("2 + 3 ;"), Ok(Node::new(Statement::Expression(add!(int!(2), int!(3))))));
    assert_eq!(statement("2 + 3 ; "), Ok(Node::new(Statement::Expression(add!(int!(2), int!(3))))));
    assert_eq!(statement("3;"), Ok(Node::new(Statement::Expression(int!(3)))));
    assert_eq!(statement("3 ;"), Ok(Node::new(Statement::Expression(int!(3)))));
    assert_eq!(statement("3 ; "), Ok(Node::new(Statement::Expression(int!(3)))));
    assert_eq!(statement("3"), Ok(Node::new(Statement::Expression(int!(3)))));
}

#[test]
fn test_decl_stmt() {
    assert_eq!(statement("let a = 4;"), Ok(Node::new(Statement::Declare(ident!(a), int!(4)))));
}

#[test]
fn test_assign_stmt() {
    assert_eq!(statement("a = 3"), Ok(Node::new(Statement::Assign(ident!(a), int!(3)))));
}
