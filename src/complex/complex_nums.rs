use crate::fractions_num::Fraction;

pub struct Complex {
    real: Fraction,
    imaginary: Fraction,
}

impl Complex {
    pub fn new(real: Fraction, imaginary: Fraction) -> Self {
        Self { real, imaginary }
    }
}
