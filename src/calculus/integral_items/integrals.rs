use std::marker::PhantomData;

/// Calculus Integral
/// Find the area underneath a curve
pub struct Integral<A, F: Fn<A>> {
    integration: IntegrationType,
    equation: F,
    _marker: PhantomData<A>,
}

/// Integration type for Integrals
pub enum IntegrationType {
    ReimannSum,
    ReimannTrapozoidalSum,
}

impl<A, F: Fn<A>> Integral<A, F> {
    pub fn new(equation: F) -> Self {
        Self {
            integration: IntegrationType::ReimannSum,
            equation,
            _marker: PhantomData,
        }
    }
}

#[test]
fn integral_test() {
    use crate::equation::equation;

    let x = Integral::new(equation!(10x + 3));
}
