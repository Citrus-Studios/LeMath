use crate::fractions_num::Fraction;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Complex {
    pub(crate) real: Fraction,
    pub(crate) imaginary: Fraction,
}

impl Complex {
    pub fn new(real: Fraction, imaginary: Fraction) -> Self {
        Self { real, imaginary }
    }
    pub fn conjugate(self) -> Self {
        let mut self_clone = self.clone();
        self_clone.imaginary = -self_clone.imaginary;
        self_clone
    }
    pub fn abs_sq(self) -> Fraction {
        self.real * self.real + self.imaginary * self.imaginary
    }
    pub fn abs(self) -> f64 {
        self.abs_sq().sqrt()
    }
}
