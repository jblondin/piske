use std::collections::HashMap;

use sindra::scope::SymbolStore;

use image;
use value::Value;
use Symbol;
use psk_std::image::{ImageData, Dims};
use psk_std::extrema::Extrema;

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

/// Piske standard environment
pub struct Environment {
    func_table: StdFuncTable,
    image_data: ImageData<f64>,
    power: f64,
    magnifier: f64,
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

type FuncResult = Result<Value, String>;
type RustFuncInterface = fn(&mut Environment, Vec<Value>) -> FuncResult;
type StdFuncTable = HashMap<ExtFuncIdent, Box<RustFuncInterface>>;

fn set_image_dims(env: &mut Environment, height: usize, width: usize) -> Result<(), String> {
    env.image_data.set_dims(Dims { rows: height, cols: width });
    Ok(())
}
add_interpreter_func!(psk_set_image_dims, set_image_dims, [usize, usize], |_| Value::Empty);

fn get_image_height(env: &mut Environment) -> Result<usize, String> {
    let &Dims { rows: height, .. } = env.image_data.get_dims();
    Ok(height)
}
add_interpreter_func!(psk_get_image_height, get_image_height, [], |i| Value::Int(i as i64));

fn get_image_width(env: &mut Environment) -> Result<usize, String> {
    let &Dims { cols: width, .. } = env.image_data.get_dims();
    Ok(width)
}
add_interpreter_func!(psk_get_image_width, get_image_width, [], |i| Value::Int(i as i64));

fn set_pixel_data(env: &mut Environment, row: usize, col: usize, value: f64) -> Result<(), String> {
    env.image_data.set(Dims::new(row, col), value);
    Ok(())
}
add_interpreter_func!(psk_set_pixel_data, set_pixel_data, [usize, usize, f64], |_| Value::Empty);

fn write(env: &mut Environment, filename: String) -> Result<(), String> {
    use std::fs::File;

    let &Dims { rows, cols } = env.image_data.get_dims();
    let mut img_buf = image::ImageBuffer::new(rows as u32, cols as u32);
    let extrema = env.image_data.extrema();
    let range = extrema.range();

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let value = env.image_data.get(Dims::new(x as usize, y as usize));

        let alpha = (env.magnifier * value / range).powf(env.power);
        let alpha = if alpha > 1.0 { 255u8 }
            else if alpha < 0.0 { 0u8 }
            else { (alpha*255.0) as u8 };
        *pixel = image::Luma([alpha]);
    }

    let mut file = File::create(filename).map_err(|e| format!("{}", e))?;
    image::ImageLuma8(img_buf).save(&mut file, image::PNG).map_err(|e| format!("{}", e))?;

    Ok(())
}
add_interpreter_func!(psk_write, write, [String], |_| Value::Empty);

fn project(env: &mut Environment, row: usize, col: usize, center: (f64, f64), size: (f64, f64))
        -> Result<(f64, f64), String> {
    let &Dims { rows, cols } = env.image_data.get_dims();
    let re = (row as f64 / rows as f64 - 0.5) * size.0 + center.0;
    let im = (col as f64 / cols as f64 - 0.5) * size.1 + center.1;
    Ok((re, im))
}
add_interpreter_func!(psk_project, project, [usize, usize, (f64, f64), (f64, f64)],
    |(re, im)| Value::Complex(re, im));

fn re(_: &mut Environment, c: (f64, f64)) -> Result<f64, String> { Ok(c.0) }
add_interpreter_func!(psk_re, re, [(f64, f64)], |f| Value::Float(f));

fn im(_: &mut Environment, c: (f64, f64)) -> Result<f64, String> { Ok(c.1) }
add_interpreter_func!(psk_im, im, [(f64, f64)], |f| Value::Float(f));
