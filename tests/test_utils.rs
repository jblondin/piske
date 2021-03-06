#![allow(dead_code)]

extern crate tempfile;
extern crate piske;

use std::fs::File;

use piske::value::Value;
use piske::parse::program;
use piske::visitor::{State, SymbolDefineVisitor, TypeComputationVisitor, EvaluateVisitor,
    TranspileVisitor};

pub fn expect_prog_with_state(prog: &str, val: Value, mut state: &mut State) {
    let ast = program(prog).unwrap();
    SymbolDefineVisitor::visit(&ast, &mut state).unwrap();
    TypeComputationVisitor::visit(&ast, &mut state).unwrap();
    let evaluated = EvaluateVisitor::visit(&ast, &mut state);
    assert_eq!(evaluated, Ok(val));
}

pub fn expect_prog(prog: &str, val: Value) {
    let mut state = State::default();
    expect_prog_with_state(prog, val, &mut state);
}

pub fn examine_translated_source(prog: &str) {
    let mut state = State::default();
    let ast = program(prog).unwrap();
    SymbolDefineVisitor::visit(&ast, &mut state).unwrap();
    TypeComputationVisitor::visit(&ast, &mut state).unwrap();
    let translated = TranspileVisitor::visit(&ast, &mut state).unwrap();
    println!("{}", translated.as_str());
}

pub fn test_output(mut file: &File, expected: &str) {
    use std::io::{Read, Seek, SeekFrom};

    let mut buffer = String::new();
    file.seek(SeekFrom::Start(0)).unwrap();
    file.read_to_string(&mut buffer).unwrap();
    assert_eq!(&buffer, expected);
}

pub fn new_state_with_temp_output() -> (State, File) {
    let mut state = State::default();
    let tempfile = tempfile::tempfile().unwrap();
    let temp_clone = tempfile.try_clone().unwrap();
    state.io.set_stdout(tempfile);
    (state, temp_clone)
}
