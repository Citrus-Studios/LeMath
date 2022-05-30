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

#[test]
fn complex_math_test() {
    let x = Complex::new(3.into(), 7.into());
    let y = Complex::new(11.into(), 37.into());
    assert_eq!(Complex::new(14.into(), 44.into()), x + y);
    assert_eq!(Complex::new((-8).into(), (-30).into()), x - y);
    assert_eq!(Complex::new((-226).into(), 188.into()), x * y);
    assert_eq!(
        Complex::new(0.1959731543624161.into(), (-0.022818791946308724).into()),
        x / y
    );
}
