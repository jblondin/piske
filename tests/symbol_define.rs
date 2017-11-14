extern crate piske;

use piske::parse::program;
use piske::visitor::symbol::DefineSymbols;

#[test]
fn test_symbol_define() {
    let prog = r#"
let a = 4;
{ let b = 23.0; }
a = a + 2;
a;
    "#;

    let mut ast = program(prog).unwrap();
    ast.define_symbols().unwrap();
    println!("{:?}", ast);
}
