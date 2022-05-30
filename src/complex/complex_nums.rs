use crate::fractions_num::Fraction;

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub(crate) real: Fraction,
    pub(crate) imaginary: Fraction,
}

impl Complex {
    pub fn new(real: Fraction, imaginary: Fraction) -> Self {
        Self { real, imaginary }
    }
    pub fn conjugate(&mut self) -> Self {
        self.imaginary = -self.imaginary;
        self.clone()
    }
}
