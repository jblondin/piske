//! Complex numbers

/// Complex number
pub struct Complex {
    /// Real component of complex number
    pub re: f64,
    /// Imaginary component of complex number
    pub im: f64
}
impl Complex {
    /// Create a new complex number
    pub fn new(re: f64, im: f64) -> Complex {
        Complex { re: re, im: im }
    }
    /// Compute the complex conjugate of a number
    pub fn conj(self) -> Complex {
        Complex { re: self.re, im: -self.im }
    }
}
