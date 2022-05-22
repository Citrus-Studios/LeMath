use std::marker::PhantomData;

pub trait IntFuncGen<I, R> = Fn<I, Output = R>;

/// Calculus Integral
/// Find the area underneath a curve
pub struct Integral<I, R, F: IntFuncGen<I, R>> {
    integration: IntegrationType,
    equation: F,
    from: f64,
    to: f64,
    sampling_rate: i64,
    _marker: PhantomData<R>,
    _marker2: PhantomData<I>,
}

/// Integration type for Integrals
pub enum IntegrationType {
    ReimannSum,
    ReimannTrapozoidalSum,
}

impl<I, R, F: IntFuncGen<I, R>> Integral<I, R, F> {
    pub fn new(from: f64, to: f64, equation: F) -> Self {
        Self {
            integration: IntegrationType::ReimannSum,
            equation,
            from,
            to,
            sampling_rate: 32,
            _marker: PhantomData,
            _marker2: PhantomData,
        }
    }
    pub fn set_sampling_rate(mut self, sampling_rate: i64) -> Self {
        self.sampling_rate = sampling_rate;
        self
    }
    pub fn reimann_sum(&self) -> R {
        todo!();
    }
    pub fn build(&self) -> R {
        return match self.integration {
            IntegrationType::ReimannSum => self.reimann_sum(),
            IntegrationType::ReimannTrapozoidalSum => todo!(),
        };
    }
}

#[test]
fn integral_test() {
    use crate::equation::equation;

    let x = Integral::new(0.0, 3.0, equation!(x));
}
