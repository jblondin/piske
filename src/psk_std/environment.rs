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
    Project
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
            set_image_dims, [("height", "int"), ("width", "int")], PType::Void);
        add_func!(scope, env.func_table, "get_image_height", ExtFuncIdent::GetImageHeight,
            get_image_height, [], PType::Int);
        add_func!(scope, env.func_table, "get_image_width", ExtFuncIdent::GetImageWidth,
            get_image_width, [], PType::Int);
        add_func!(scope, env.func_table, "write", ExtFuncIdent::Write, write, [("file", "string")],
            PType::Void);
        add_func!(scope, env.func_table, "set_pixel_data", ExtFuncIdent::SetPixelData,
            set_pixel_data, [("row", "int"), ("col", "int"), ("value", "float")], PType::Void);
        add_func!(scope, env.func_table, "project", ExtFuncIdent::Project, project,
            [("row", "int"), ("col", "int"), ("center", "complex"), ("size", "complex")],
            PType::Complex);
        env
    }
}

type FuncResult = Result<Value, String>;
type RustFuncInterface = fn(&mut Environment, Vec<Value>) -> FuncResult;
type StdFuncTable = HashMap<ExtFuncIdent, Box<RustFuncInterface>>;

define_func!(set_image_dims, env, [height: usize, width: usize], {
    env.image_data.set_dims(Dims { rows: height, cols: width });
    Ok(Value::Empty)
});
define_func!(get_image_height, env, [], {
    let &Dims { rows: height, .. } = env.image_data.get_dims();
    Ok(Value::Int(height as i64))
});
define_func!(get_image_width, env, [], {
    let &Dims { cols: width, .. } = env.image_data.get_dims();
    Ok(Value::Int(width as i64))
});
define_func!(set_pixel_data, env, [row: usize, col: usize, value: f64], {
    env.image_data.set(Dims::new(row, col), value);
    Ok(Value::Empty)
});
define_func!(write, env, [filename: String], {
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

    Ok(Value::Empty)
});
define_func!(project, env, [row: usize, col: usize, center: (f64, f64), size: (f64, f64)], {
    let &Dims { rows, cols } = env.image_data.get_dims();
    let re = (row as f64 / rows as f64 - 0.5) / size.0 + center.0;
    let im = (col as f64 / cols as f64 - 0.5) / size.1 + center.1;
    Ok(Value::Complex(re, im))
});

