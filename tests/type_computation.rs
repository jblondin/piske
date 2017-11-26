extern crate piske;

use piske::parse::program;
use piske::visitor::{State, SymbolDefineVisitor, TypeComputationVisitor};

#[test]
fn test_compute_types() {
    let prog = r#"
let a = 4.0;
{ let b = 23.0;
b = 4; }
a = a + 2;
a;
    "#;

    let ast = program(prog).unwrap();
    let mut state = State::default();
    SymbolDefineVisitor::visit(&ast, &mut state).unwrap();
    TypeComputationVisitor::visit(&ast, &mut state).unwrap();
}
