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
