use std::ops::{Add, Mul, Sub};

use super::complex_nums::Complex;

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.real + rhs.real, self.imaginary + rhs.imaginary)
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex::new(self.real - rhs.real, self.imaginary - rhs.imaginary)
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex::new(
            (self.real * rhs.real) - (self.imaginary * rhs.imaginary),
            (self.real * rhs.imaginary) + (self.imaginary * rhs.real),
        )
    }
}
