extern crate piske;

use piske::parse::program;
use piske::visitor::symbol::DefineSymbols;
use piske::visitor::type_visitor::ComputeTypes;

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
    ast.borrow_mut().define_symbols().unwrap();
    ast.borrow_mut().compute_types().unwrap();
}
