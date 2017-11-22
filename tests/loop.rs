extern crate piske;

use piske::parse::program;
use piske::visitor::symbol::DefineSymbols;
use piske::visitor::type_visitor::ComputeTypes;
use piske::visitor::eval::Evaluate;
use piske::value::Value;

fn expect_prog(prog: &str, val: Value) {
    let ast = program(prog).unwrap();
    ast.define_symbols().unwrap();
    ast.compute_types().unwrap();
    let evaluated = ast.eval();
    assert_eq!(evaluated, Ok(val));
}

#[test]
fn test_interval_integer() {
    let prog = r#"
let a = 0;
iterate over [0, 10) {
    a = a + 1;
}
a
    "#;

    expect_prog(prog, Value::Int(10));

    let prog = r#"
let a = 0;
iterate over [0, 10] {
    a = a + 1;
}
a
    "#;

    expect_prog(prog, Value::Int(11));


    let prog = r#"
let a = 0;
iterate i = [1, 11) {
    a = a + i;
}
a
    "#;

    expect_prog(prog, Value::Int(10 * 11 / 2));

    let prog = r#"
let a = 0;
iterate i = [1, 10] {
    a = a + i;
}
a
    "#;

    expect_prog(prog, Value::Int(10 * 11 / 2));

}

#[test]
fn test_break() {

    let prog = r#"
let a = 0;
let b = iterate i = [1, 10) {
    a = a + i
    i
}
b
    "#;

    expect_prog(prog, Value::Int(9));

    let prog = r#"
let a = iterate i = [1, 100) {
    if i > 50 {
        break 101;
    }
    i
}
a
    "#;

    expect_prog(prog, Value::Int(101));

    let prog = r#"
let a = iterate i = [1, 50] {
    if i > 50 {
        break 101;
    }
    i
}
a
    "#;

    expect_prog(prog, Value::Int(50));

}
