pub struct EquationBuilder {}

impl EquationBuilder {
    pub fn new(equation: &str) {
        let equation = equation.to_string();
        
    }
}

#[test]
fn equation_enum_test() {
    let equation = EquationBuilder::new("10*x + 3");
    println!("{:#?}", equation);
}
