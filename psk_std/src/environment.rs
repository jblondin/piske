use image::ImageData;


/// Piske standard environment
pub struct Environment {
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
            image_data: ImageData::<f64>::default(),
            magnifier: 1.0,
            power: 0.8,
        }
    }
}
