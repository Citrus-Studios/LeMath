use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

pub struct EquationBuilder {}

impl EquationBuilder {
    pub fn new(equation: &str) -> Pairs<Rule> {
        let mut pairs =
            EquationParser::parse(Rule::equation, equation).unwrap_or_else(|e| panic!("{}", e));
        pairs
    }
}

#[derive(Parser)]
#[grammar = "../equation.pest"]
struct EquationParser;

#[test]
fn equation_enum_test() {
    let equation = EquationBuilder::new("10*x + 3");
    println!("{:#?}", equation);
}
