//! Standard library functions for piske.

use std::collections::HashMap;
use std::io::{self, Stdin, Stdout, Stderr, Write};

use sindra::scope::SymbolStore;

use sindra::value::Extract;
use Symbol;
use value::Value;

#[macro_use] mod macros;

/// External function identifiers for the standard library
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtFuncIdent {
    /// set_image_dims std function
    SetImageDims,
    /// get_image_height std function
    GetImageHeight,
    /// get_image_width std function
    GetImageWidth,
    /// print std function
    Print
}

struct Dims {
    height: u64,
    width: u64,
}
impl Default for Dims {
    fn default() -> Dims {
        Dims {
            height: 1024,
            width: 1024,
        }
    }
}

/// Piske standard environment
pub struct Environment {
    func_table: StdFuncTable,
    image_dims: Dims,
    #[allow(dead_code)]
    stdin: Stdin,
    #[allow(dead_code)]
    stdout: Stdout,
    #[allow(dead_code)]
    stderr: Stderr,
}
impl Default for Environment {
    fn default() -> Environment {
        Environment {
            func_table: StdFuncTable::new(),
            image_dims: Dims::default(),
            stdin: io::stdin(),
            stdout: io::stdout(),
            stderr: io::stderr(),
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
            set_image_dims, [("height", "int"), ("width", "int")]);
        add_func!(scope, env.func_table, "get_image_height", ExtFuncIdent::GetImageHeight,
            get_image_height, []);
        add_func!(scope, env.func_table, "get_image_width", ExtFuncIdent::GetImageWidth,
            get_image_width, []);
        add_func!(scope, env.func_table, "print_int", ExtFuncIdent::Print, print_int,
            [("message", "int")]);
        env
    }
}

type FuncResult = Result<Value, String>;
type RustFuncInterface = fn(&mut Environment, Vec<Value>) -> FuncResult;
type StdFuncTable = HashMap<ExtFuncIdent, Box<RustFuncInterface>>;

define_func!(set_image_dims, env, [height: u64, width: u64], {
    env.image_dims = Dims { height: height, width: width };
    Ok(Value::Empty)
});
define_func!(get_image_height, env, [], {
    Ok(Value::Int(env.image_dims.height as i64))
});
define_func!(get_image_width, env, [], {
    Ok(Value::Int(env.image_dims.width as i64))
});
define_func!(print_int, env, [message: i64], {
    writeln!(&mut env.stdout, "{}", message).unwrap();
    Ok(Value::Empty)
});
