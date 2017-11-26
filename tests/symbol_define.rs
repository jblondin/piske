extern crate piske;

use piske::parse::program;
use piske::visitor::{State, SymbolDefineVisitor};

#[test]
fn test_symbol_define() {
    let prog = r#"
let a = 4;
{ let b = 23.0; }
a = a + 2;
a;
    "#;

    let ast = program(prog).unwrap();
    let mut state = State::default();
    ast.visit(&mut state).unwrap();
    println!("{:?}", ast);
}
