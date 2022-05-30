use crate::fractions_num::Fraction;

pub struct Complex {
    pub(crate) real: Fraction,
    pub(crate) imaginary: Fraction,
}

impl Complex {
    pub fn new(real: Fraction, imaginary: Fraction) -> Self {
        Self { real, imaginary }
    }
}
