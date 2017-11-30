use image;

use psk_std::environment::Environment;
use psk_std::image::Dims;
use psk_std::extrema::Extrema;

/// Set the image dimensions. May invalidate the contents of the image data.
pub fn set_image_dims(env: &mut Environment, height: usize, width: usize) -> Result<(), String> {
    env.image_data.set_dims(Dims { rows: height, cols: width });
    Ok(())
}
/// Get the currently set image height.
pub fn get_image_height(env: &mut Environment) -> Result<usize, String> {
    let &Dims { rows: height, .. } = env.image_data.get_dims();
    Ok(height)
}
/// Get the currently set image width.
pub fn get_image_width(env: &mut Environment) -> Result<usize, String> {
    let &Dims { cols: width, .. } = env.image_data.get_dims();
    Ok(width)
}
/// Set the current pixel data for the specified row and column
pub fn set_pixel_data(env: &mut Environment, row: usize, col: usize, value: f64)
        -> Result<(), String> {
    env.image_data.set(Dims::new(row, col), value);
    Ok(())
}
/// Render the current image data and write it to a file.
pub fn write(env: &mut Environment, filename: String) -> Result<(), String> {
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
/// Project the given pixel onto the underlying axes, using the provided center and size.
pub fn project(env: &mut Environment, row: usize, col: usize, center: (f64, f64), size: (f64, f64))
        -> Result<(f64, f64), String> {
    let &Dims { rows, cols } = env.image_data.get_dims();
    let re = (row as f64 / rows as f64 - 0.5) * size.0 + center.0;
    let im = (col as f64 / cols as f64 - 0.5) * size.1 + center.1;
    Ok((re, im))
}
/// Extract the real component of a complex number.
pub fn re(_: &mut Environment, c: (f64, f64)) -> Result<f64, String> { Ok(c.0) }
/// Extract the imaginary component of a complex number.
pub fn im(_: &mut Environment, c: (f64, f64)) -> Result<f64, String> { Ok(c.1) }
