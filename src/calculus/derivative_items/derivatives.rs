use pest::iterators::Pairs;

use crate::equation::Rule;

pub struct Derivative<'a> {
    equation: Pairs<'a, Rule>,
}

impl<'a> Derivative<'a> {
    pub fn new(equation: Pairs<'a, Rule>) -> Self {
        Self { equation }
    }
}
