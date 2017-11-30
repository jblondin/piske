use std::collections::HashMap;

use sindra::scope::SymbolStore;

use value::Value;
use Symbol;

use psk_std::image::ImageData;
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
type StdFuncTable = HashMap<ExtFuncIdent, Box<RustFuncInterface>>;

/// Piske standard environment
pub struct Environment {
    func_table: StdFuncTable,
    /// Stored ImageData for the current environment
    pub image_data: ImageData<f64>,
    /// Mandelbrot power ( color = (magnifier * escape_value)^power )
    pub power: f64,
    /// Mandelbrot magnifier ( color = (magnifier * escape_value)^power )
    pub magnifier: f64,
}
impl Default for Environment {
    fn default() -> Environment {
        Environment {
            func_table: StdFuncTable::new(),
            image_data: ImageData::<f64>::default(),
            magnifier: 1.0,
            power: 0.8,
        }
    }
}
impl Environment {
    /// Call a standard library function with a vector of arguments
    pub fn call(&mut self, func: ExtFuncIdent, args: Vec<Value>) -> FuncResult {
        self.func_table[&func](self, args)
    }
    /// Create a new environment, and register the standard functions in the scope
    pub fn new<Sc: SymbolStore<Symbol>>(scope: &mut Sc) -> Environment {
        let mut env = Environment::default();
        add_func!(scope, env.func_table, "set_image_dims", ExtFuncIdent::SetImageDims,
            psk_set_image_dims, [("height", "int"), ("width", "int")], PType::Void);
        add_func!(scope, env.func_table, "get_image_height", ExtFuncIdent::GetImageHeight,
            psk_get_image_height, [], PType::Int);
        add_func!(scope, env.func_table, "get_image_width", ExtFuncIdent::GetImageWidth,
            psk_get_image_width, [], PType::Int);
        add_func!(scope, env.func_table, "write", ExtFuncIdent::Write, psk_write,
            [("file", "string")], PType::Void);
        add_func!(scope, env.func_table, "set_pixel_data", ExtFuncIdent::SetPixelData,
            psk_set_pixel_data, [("row", "int"), ("col", "int"), ("value", "float")], PType::Void);
        add_func!(scope, env.func_table, "project", ExtFuncIdent::Project, psk_project,
            [("row", "int"), ("col", "int"), ("center", "complex"), ("size", "complex")],
            PType::Complex);
        add_func!(scope, env.func_table, "re", ExtFuncIdent::Re, psk_re, [("c", "complex")],
            PType::Float);
        add_func!(scope, env.func_table, "im", ExtFuncIdent::Im, psk_im, [("c", "complex")],
            PType::Float);
        env
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
