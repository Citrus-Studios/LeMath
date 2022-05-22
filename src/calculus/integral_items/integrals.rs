use equation::equation;

/// Calculus Integral
/// Find the area underneath a curve
pub struct Integral<F> {
    integration: IntegrationType,
    equation: F,
}

/// Integration type for Integrals
pub enum IntegrationType {
    ReimannSum,
    ReimannTrapozoidalSum,
}

impl<F> Integral<F> {
    pub fn new(equation: F) -> Self {
        Self {
            integration: ReimannSum,
            equation,
        }
    }
}

#[test]
fn integral_test() {
    let x = Integral::new(equation!(10x + 3));
}
