use crate::fractions_num::Fraction;

pub struct Derivative {
    equation: String,
}

impl Derivative {
    pub fn new(equation: String) -> Self {
        Self { equation }
    }
    pub fn build() -> fn(Fraction) -> Fraction {
        todo!()
    }
}
