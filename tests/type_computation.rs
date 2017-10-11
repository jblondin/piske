extern crate piske;

use piske::compiler_parse::program;
use piske::visitor::symbol::{self, SymbolDefineVisitor};
use piske::visitor::type_visitor::compute_types;

#[test]
fn test_compute_types() {
    let prog = r#"
let a = 4.0;
{ let b = 23.0;
b = 4; }
a = a + 2;
a;
    "#;

    let mut ast = program(prog).unwrap();
    let mut sym_state = symbol::State::new();
    ast.define_symbols(&mut sym_state).unwrap();
    compute_types(&mut ast).unwrap();
}
