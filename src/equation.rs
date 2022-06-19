pub struct EquationBuilder {}

impl EquationBuilder {
    pub fn new(equation: &str) {}
}

#[test]
fn equation_enum_test() {
    let equation = EquationBuilder::new("10*x + 3");
    println!("{:#?}", equation);
}
