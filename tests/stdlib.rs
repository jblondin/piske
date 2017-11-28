extern crate tempfile;
extern crate piske;

use piske::value::Value;

mod test_utils;
use test_utils::*;

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

#[test]
fn test_project() {
    expect_prog(r#"project(1024, 512, 0+0i, 2+2i)"#, Value::Complex(1.0, 0.0));
    expect_prog(r#"project(512 + 256, 512 + 256, 0+0i, 2+2i)"#, Value::Complex(0.5, 0.5));
    expect_prog(r#"project(256, 512 + 256, 0+0i, 2+2i)"#, Value::Complex(-0.5, 0.5));
    expect_prog(r#"project(512 + 256, 256, 0+0i, 2+2i)"#, Value::Complex(0.5, -0.5));

    let prog = r#"
let z = 0+0i;
let c = project(0, 1024, 0+0i, 2+2i);
z = z * z + c;
z
    "#;
    expect_prog(prog, Value::Complex(-1.0, 1.0));

}

#[test]
fn test_complex_extraction() {
    expect_prog(r"re(1+0i)", Value::Float(1.0));
    expect_prog(r"im(1+0i)", Value::Float(0.0));
}
