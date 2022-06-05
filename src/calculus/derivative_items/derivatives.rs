use pest::iterators::Pairs;

use crate::{equation::Rule, fractions_num::Fraction};

pub struct Derivative<'a> {
    equation: Pairs<'a, Rule>,
}

impl<'a> Derivative<'a> {
    pub fn new(equation: Pairs<'a, Rule>) -> Self {
        Self { equation }
    }
    pub fn build() -> fn(Fraction) -> Fraction {
        todo!()
    }
}
