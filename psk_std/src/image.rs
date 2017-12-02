pub struct Dims {
    pub rows: i64,
    pub cols: i64,
}
impl Default for Dims {
    fn default() -> Dims {
        Dims {
            rows: 1024,
            cols: 1024,
        }
    }
}
impl Dims {
    pub fn new(r: i64, c: i64) -> Dims {
        Dims {
            rows: r,
            cols: c,
        }
    }
}

// pub enum PixelType {
//     RGBA,
//     RGB,
//     GrayscaleAlpha,
//     Grayscale,
// }
// pub enum Pixel {
//     RGBA {
//         r: u8,
//         g: u8,
//         b: u8,
//         a: u8
//     },
//     RGB {
//         r: u8,
//         g: u8,
//         b: u8
//     },
//     GrayscaleAlpha {
//         v: u8,
//         a: u8,
//     },
//     Grayscale {
//         v: u8,
//     },
// }

// pub trait IntoPixel: Sized {
//     fn into_pixel(self, pixel_type: PixelType) -> Pixel {
//         match pixel_type {
//             PixelType::RGBA => self.into_rgba(),
//             PixelType::RGB => self.into_rgb(),
//             PixelType::GrayscaleAlpha => self.into_grayscale_alpha(),
//             PixelType::Grayscale => self.into_grayscale(),
//         }
//     }
//     fn into_rgba(self) -> Pixel;
//     fn into_rgb(self) -> Pixel;
//     fn into_grayscale_alpha(self) -> Pixel;
//     fn into_grayscale(self) -> Pixel;
// }
// pub trait IntoStorage<T> {
//     fn into(&self) -> T;
// }

// impl IntoPixel for u64 {
//     fn into_rgba(self) -> Pixel {
//         let mask = !0u8 as u64;
//         Pixel::RGBA {
//             r: (self | ((mask << 24) >> 24)) as u8,
//             g: (self | ((mask << 16) >> 16)) as u8,
//             b: (self | ((mask << 8) >> 8)) as u8,
//             a: (self | mask) as u8,
//         }
//     }
//     fn into_rgb(self) -> Pixel {
//         let mask = !0u8 as u64;
//         Pixel::RGB {
//             r: (self | ((mask << 16) >> 16)) as u8,
//             g: (self | ((mask << 8) >> 8)) as u8,
//             b: (self | mask) as u8,
//         }
//     }
//     fn into_grayscale_alpha(self) -> Pixel {
//         let mask = !0u8 as u64;
//         Pixel::GrayscaleAlpha {
//             v: (self | ((mask << 8) >> 8)) as u8,
//             a: (self | mask) as u8,
//         }
//     }
//     fn into_grayscale(self) -> Pixel {
//         Pixel::Grayscale {
//             v: (self | (!0u8 as u64)) as u8,
//         }
//     }
// }
// impl From<Pixel> for u64 {
//     fn from(p: Pixel) -> u64 {
//         match p {
//             Pixel::RGBA { r, g, b, a } => {
//                 (r as u64) << 24 | (g as u64) << 16 | (b as u64) << 8 | (a as u64)
//             }
//             Pixel::RGB { r, g, b } => {
//                 (r as u64) << 16 | (g as u64) << 8 | (b as u64)
//             },
//             Pixel::GrayscaleAlpha { v, a } => {
//                 (v as u64) << 8 | (a as u64)
//             },
//             Pixel::Grayscale { v } => {
//                 v as u64
//             }
//         }
//     }
// }


pub struct ImageData<T> {
    pub dims: Dims,
    pub values: Vec<T>,
}
impl<T: Clone + Default> Default for ImageData<T> {
    fn default() -> ImageData<T> {
        let dims = Dims::default();
        let (r, c) = (dims.rows, dims.cols);
        ImageData {
            dims: dims,
            values: vec![T::default(); (r * c) as usize]
        }
    }
}
impl<T: Copy> ImageData<T> {
    pub fn get(&self, loc: Dims) -> T {
        self.values[(loc.cols * self.dims.rows + loc.rows) as usize]
    }
    pub fn set(&mut self, loc: Dims, value: T) {
        self.values[(loc.cols * self.dims.rows + loc.rows) as usize] = value;
    }
    pub fn get_dims(&self) -> &Dims { &self.dims }
    pub fn set_dims(&mut self, dims: Dims) { self.dims = dims }
}

// impl Image for ImageData<u64> {
//     fn set_pixel(&mut self, loc: Dims, pixel: Pixel) {
//         self.set(loc, pixel.into())
//     }
//     fn get_pixel(&mut self, loc: Dims, pixel_type: PixelType) -> Pixel {
//         self.get(loc).into_pixel(pixel_type)
//     }
// }

// pub trait Image {
//     fn set_pixel(&mut self, loc: Dims, pixel: Pixel);
//     fn get_pixel(&mut self, loc: Dims, pixel_type: PixelType) -> Pixel;
// }
