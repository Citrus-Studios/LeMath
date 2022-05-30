use std::ops::{Add, Div, Mul, Sub};

use crate::fractions_num::Fraction;

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

impl Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        return self * rhs.conjugate() / rhs.abs_sq();
    }
}

impl Add<Fraction> for Complex {
    type Output = Complex;

    fn add(self, rhs: Fraction) -> Self::Output {
        return Complex::new(self.real + rhs, self.imaginary);
    }
}

impl Sub<Fraction> for Complex {
    type Output = Complex;

    fn sub(self, rhs: Fraction) -> Self::Output {
        return Complex::new(self.real - rhs, self.imaginary);
    }
}

impl Mul<Fraction> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Fraction) -> Self::Output {
        return Complex::new(self.real * rhs, self.imaginary * rhs);
    }
}

impl Div<Fraction> for Complex {
    type Output = Complex;

    fn div(self, rhs: Fraction) -> Self::Output {
        return Complex::new(self.real / rhs, self.imaginary / rhs);
    }
}
