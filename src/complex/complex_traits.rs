use std::ops::Add;

use super::complex_nums::Complex;

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.real + rhs.real, self.imaginary + rhs.imaginary)
    }
}
