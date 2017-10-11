extern crate piske;

use piske::compiler_parse::program;
use piske::visitor::symbol::{State, SymbolDefineVisitor};

#[test]
fn test_symbol_define() {
    let prog = r#"
let a = 4;
{ let b = 23.0; }
a = a + 2;
a;
    "#;

    let mut ast = program(prog).unwrap();
    let mut state = State::default();
    ast.define_symbols(&mut state).unwrap();
    println!("{:?}", ast);
}
