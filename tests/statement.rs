extern crate piske;
extern crate sindra;

use sindra::{Node, Identifier};

use piske::parse::statement;
use piske::ast::ast::*;
use piske::value::Value;

mod test_utils;
use test_utils::*;

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

#[test]
fn test_print() {
    let (mut state, tempfile) = new_state_with_temp_output();
    let prog = r#"
print 5;
    "#;
    expect_prog_with_state(prog, Value::Empty, &mut state);
    test_output(&tempfile, "5\n");

    let (mut state, tempfile) = new_state_with_temp_output();
    let prog = r#"
let hello = "hello, world!";
print hello;
    "#;
    expect_prog_with_state(prog, Value::Empty, &mut state);
    test_output(&tempfile, "hello, world!\n");

    let (mut state, tempfile) = new_state_with_temp_output();
    let prog = r#"
let hello = "hello, world!";
print hello, " hello!";
    "#;
    expect_prog_with_state(prog, Value::Empty, &mut state);
    test_output(&tempfile, "hello, world! hello!\n");


    let (mut state, tempfile) = new_state_with_temp_output();
    let prog = r#"
print "hello, world!";
    "#;
    expect_prog_with_state(prog, Value::Empty, &mut state);
    test_output(&tempfile, "hello, world!\n");

}
