extern crate piske;

use piske::parse::program;
use piske::visitor::{State, SymbolDefineVisitor, TypeComputationVisitor, EvaluateVisitor};
use piske::value::Value;

fn eval_prog(prog: &str) -> Result<Value, String> {
    let ast = program(prog).unwrap();
    let mut state = State::default();
    SymbolDefineVisitor::visit(&ast, &mut state).unwrap();
    TypeComputationVisitor::visit(&ast, &mut state).unwrap();
    EvaluateVisitor::visit(&ast, &mut state)
}

fn expect_prog(prog: &str, val: Value) {
    assert_eq!(eval_prog(prog), Ok(val));
}

fn expect_prog_approx_complex(prog: &str, re: f64, im: f64) {
    let val = eval_prog(prog);
    if let Ok(Value::Complex(r, i)) = val {
        assert!((r - re).abs() < 1e-6);
        assert!((i - im).abs() < 1e-6);
    } else {
        panic!("Expected complex number, found {:?}", val);
    }
}

#[test]
fn test_imaginary() {
    let prog = r#"
let a = 1 + 2i;
a
    "#;

    expect_prog(prog, Value::Complex(1.0, 2.0));

    expect_prog(r#"1 + 2i"#, Value::Complex(1.0, 2.0));
    expect_prog(r#"1.0 + 2i"#, Value::Complex(1.0, 2.0));
    expect_prog(r#"1 + 2.0i"#, Value::Complex(1.0, 2.0));
    expect_prog(r#"1.0 + 2.0i"#, Value::Complex(1.0, 2.0));

    expect_prog(r#"2i"#, Value::Complex(0.0, 2.0));
    expect_prog(r#"2.0i"#, Value::Complex(0.0, 2.0));

    expect_prog(r#"0i"#, Value::Complex(0.0, 0.0));
    expect_prog(r#"0.0i"#, Value::Complex(0.0, 0.0));

    expect_prog(r#"2 + 0i"#, Value::Complex(2.0, 0.0));
    expect_prog(r#"2 + 0.0i"#, Value::Complex(2.0, 0.0));
    expect_prog(r#"2.0 + 0i"#, Value::Complex(2.0, 0.0));
    expect_prog(r#"2.0 + 0.0i"#, Value::Complex(2.0, 0.0));

}

#[test]
fn test_complex_arith() {
    expect_prog(r#"1 + 2i + 2 + 3i"#, Value::Complex(3.0, 5.0));
    expect_prog(r#"(1 + 2i) + 2 + 3i"#, Value::Complex(3.0, 5.0));
    expect_prog(r#"1 + 2i + (2 + 3i)"#, Value::Complex(3.0, 5.0));
    expect_prog(r#"(1 + 2i) + (2 + 3i)"#, Value::Complex(3.0, 5.0));

    expect_prog(r#"1 + 2i - 2 + 3i"#, Value::Complex(-1.0, 5.0));
    expect_prog(r#"(1 + 2i) - 2 + 3i"#, Value::Complex(-1.0, 5.0));
    expect_prog(r#"1 + 2i - (2 + 3i)"#, Value::Complex(-1.0, -1.0));
    expect_prog(r#"(1 + 2i) - (2 + 3i)"#, Value::Complex(-1.0, -1.0));

    expect_prog(r#"1 + 2i * 2 + 3i"#, Value::Complex(1.0, 7.0));
    expect_prog(r#"(1 + 2i) * 2 + 3i"#, Value::Complex(2.0, 7.0));
    expect_prog(r#"1 + 2i * (2 + 3i)"#, Value::Complex(-5.0, 4.0));
    expect_prog(r#"(1 + 2i) * (2 + 3i)"#, Value::Complex(-4.0, 7.0));

    expect_prog_approx_complex(r#"1 + 2i / 2 + 3i"#, 1.0, 4.0);
    expect_prog_approx_complex(r#"(1 + 2i) / 2 + 3i"#, 0.5, 4.0);
    expect_prog_approx_complex(r#"1 + 2i / (2 + 3i)"#, 1.4615384615384617, 0.3076923076923077);
    expect_prog_approx_complex(r#"(1 + 2i) / (2 + 3i)"#, 0.6153846153846154, 0.07692307692307691);
}

#[test]
fn test_conjugate() {
    expect_prog(r#"(1 + 2i)`"#, Value::Complex(1.0, -2.0));
    expect_prog(r#"1 + (2i)`"#, Value::Complex(1.0, -2.0));
}
