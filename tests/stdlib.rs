extern crate tempfile;
extern crate piske;

use std::fs::File;

use piske::parse::program;
use piske::visitor::{State, SymbolDefineVisitor, TypeComputationVisitor, EvaluateVisitor};
use piske::value::Value;

fn expect_prog_with_state(prog: &str, val: Value, mut state: &mut State) {
    let ast = program(prog).unwrap();
    SymbolDefineVisitor::visit(&ast, &mut state).unwrap();
    TypeComputationVisitor::visit(&ast, &mut state).unwrap();
    let evaluated = EvaluateVisitor::visit(&ast, &mut state);
    assert_eq!(evaluated, Ok(val));
}

fn expect_prog(prog: &str, val: Value) {
    let mut state = State::default();
    expect_prog_with_state(prog, val, &mut state);
}

#[test]
fn test_image_dims() {

    let prog = r#"
set_image_dims(50, 60);
get_image_height()
    "#;
    expect_prog(prog, Value::Int(50));

    let prog = r#"
set_image_dims(50, 60);
get_image_width()
    "#;
    expect_prog(prog, Value::Int(60));
}

fn test_output(mut file: &File, expected: &str) {
    use std::io::{Read, Seek, SeekFrom};

    let mut buffer = String::new();
    file.seek(SeekFrom::Start(0)).unwrap();
    file.read_to_string(&mut buffer).unwrap();
    assert_eq!(&buffer, expected);
}

#[test]
fn test_print() {
    let mut state = State::default();
    let tempfile = tempfile::tempfile().unwrap();
    let temp_clone = tempfile.try_clone().unwrap();
    state.std_env.set_stdout(tempfile);
    let prog = r#"
print_int(5);
    "#;
    expect_prog_with_state(prog, Value::Empty, &mut state);
    test_output(&temp_clone, "5\n");

    let tempfile = tempfile::tempfile().unwrap();
    let temp_clone = tempfile.try_clone().unwrap();
    state.std_env.set_stdout(tempfile);
    let prog = r#"
let hello = "hello, world!";
print_string(hello);
    "#;
    expect_prog_with_state(prog, Value::Empty, &mut state);
    test_output(&temp_clone, "hello, world!\n");
}
