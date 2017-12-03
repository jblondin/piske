//! Complex numbers
use std::ops::{Add, Sub, Mul, Div, Neg};

/// Complex number
#[derive(Clone, Copy, Debug, PartialEq)]
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

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}
impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex {
        Complex::new(self.re - rhs.re, self.im - rhs.im)
    }
}
impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        let (a, b, c, d) = (self.re, self.im, rhs.re, rhs.im);
        Complex::new(
            a * c - b * d,
            b * c + a * d
        )
    }
}
impl Div for Complex {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Complex {
        let (a, b, c, d) = (self.re, self.im, rhs.re, rhs.im);
        let denom = c * c + d * d;
        Complex::new(
            (a * c + b * d) / denom,
            (b * c - a * d) / denom
        )
    }
}
impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        Complex::new(-self.re, -self.im)
    }
}
