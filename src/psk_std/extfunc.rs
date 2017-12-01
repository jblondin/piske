//! External function handling.

use std::collections::HashMap;

use sindra::scope::SymbolStore;

use value::Value;
use Symbol;

use psk_std::environment::Environment;
use psk_std::stdlib::*;

/// External function identifiers for the standard library
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtFuncIdent {
    /// set_image_dims std function
    SetImageDims,
    /// get_image_height std function
    GetImageHeight,
    /// get_image_width std function
    GetImageWidth,
    /// write std function
    Write,
    /// set_pixel_data std function
    SetPixelData,
    /// project std function
    Project,
    /// re std function
    Re,
    /// im std function
    Im
}

type FuncResult = Result<Value, String>;
type RustFuncInterface = fn(&mut Environment, Vec<Value>) -> FuncResult;

/// Standard function lookup for Piske interpreter.
pub struct StdFuncTable {
    func_table: HashMap<ExtFuncIdent, Box<RustFuncInterface>>,
}
impl Default for StdFuncTable {
    fn default() -> StdFuncTable {
        StdFuncTable {
            func_table: HashMap::new(),
        }
    }
}
impl StdFuncTable {
    /// Call a standard library function with a vector of arguments
    pub fn call(&self, env: &mut Environment, func: ExtFuncIdent, args: Vec<Value>) -> FuncResult {
        self.func_table[&func](env, args)
    }
    /// Create a new standard function table, and register the standard functions in the scope
    pub fn new<Sc: SymbolStore<Symbol>>(scope: &mut Sc) -> StdFuncTable {
        let mut tbl = StdFuncTable::default();
        add_func!(scope, tbl.func_table, "set_image_dims", ExtFuncIdent::SetImageDims,
            psk_set_image_dims, [("height", "int"), ("width", "int")], PType::Void);
        add_func!(scope, tbl.func_table, "get_image_height", ExtFuncIdent::GetImageHeight,
            psk_get_image_height, [], PType::Int);
        add_func!(scope, tbl.func_table, "get_image_width", ExtFuncIdent::GetImageWidth,
            psk_get_image_width, [], PType::Int);
        add_func!(scope, tbl.func_table, "write", ExtFuncIdent::Write, psk_write,
            [("file", "string")], PType::Void);
        add_func!(scope, tbl.func_table, "set_pixel_data", ExtFuncIdent::SetPixelData,
            psk_set_pixel_data, [("row", "int"), ("col", "int"), ("value", "float")], PType::Void);
        add_func!(scope, tbl.func_table, "project", ExtFuncIdent::Project, psk_project,
            [("row", "int"), ("col", "int"), ("center", "complex"), ("size", "complex")],
            PType::Complex);
        add_func!(scope, tbl.func_table, "re", ExtFuncIdent::Re, psk_re, [("c", "complex")],
            PType::Float);
        add_func!(scope, tbl.func_table, "im", ExtFuncIdent::Im, psk_im, [("c", "complex")],
            PType::Float);
        tbl
    }
}

add_interpreter_func!(psk_set_image_dims, set_image_dims, [usize, usize], |_| Value::Empty);
add_interpreter_func!(psk_get_image_height, get_image_height, [], |i| Value::Int(i as i64));
add_interpreter_func!(psk_get_image_width, get_image_width, [], |i| Value::Int(i as i64));
add_interpreter_func!(psk_set_pixel_data, set_pixel_data, [usize, usize, f64], |_| Value::Empty);
add_interpreter_func!(psk_write, write, [String], |_| Value::Empty);
add_interpreter_func!(psk_project, project, [usize, usize, (f64, f64), (f64, f64)],
    |(re, im)| Value::Complex(re, im));
add_interpreter_func!(psk_re, re, [(f64, f64)], |f| Value::Float(f));
add_interpreter_func!(psk_im, im, [(f64, f64)], |f| Value::Float(f));
