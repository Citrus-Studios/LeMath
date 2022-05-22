use std::marker::PhantomData;

use crate::fraction::fractions::Fraction;

/// Calculus Integral
/// Find the area underneath a curve\
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Integral {
    integration: IntegrationType,
    equation: fn(Fraction) -> Fraction,
    from: Fraction,
    to: Fraction,
    sampling_rate: i64,
}

/// Integration type for Integrals
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum IntegrationType {
    ReimannSum,
    ReimannTrapozoidalSum,
}

impl Integral {
    pub fn new(from: f64, to: f64, equation: fn(Fraction) -> Fraction) -> Self {
        Self {
            integration: IntegrationType::ReimannSum,
            equation,
            from: from.into(),
            to: to.into(),
            sampling_rate: 32,
        }
    }
    pub fn set_sampling_rate(mut self, sampling_rate: i64) -> Self {
        self.sampling_rate = sampling_rate;
        self
    }
    pub fn reimann_sum(&self) -> Fraction {
        let dx = (self.to - self.from) / self.sampling_rate.into();

        todo!();
    }
    pub fn build(&self) -> Fraction {
        return match self.integration {
            IntegrationType::ReimannSum => self.reimann_sum(),
            IntegrationType::ReimannTrapozoidalSum => todo!(),
        };
    }
}

#[test]
fn integral_test() {
    let x = Integral::new(0.0, 3.0, |x| {
        return x;
    })
    .set_sampling_rate(4)
    .build();
    assert_eq!(4.5, x.into());
}
