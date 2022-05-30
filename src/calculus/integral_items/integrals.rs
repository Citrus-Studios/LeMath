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
    ReimannSumLeft,
    ReimannSumRight,
    ReimannSumMid,
    ReimannTrapezoidalSum,
}

impl Integral {
    pub fn new(from: f64, to: f64, equation: fn(Fraction) -> Fraction) -> Self {
        Self {
            integration: IntegrationType::ReimannTrapezoidalSum,
            equation,
            from: from.into(),
            to: to.into(),
            sampling_rate: 10000,
        }
    }
    pub fn set_sampling_rate(mut self, sampling_rate: i64) -> Self {
        self.sampling_rate = sampling_rate;
        self
    }
    pub fn set_integration_type(mut self, integration_type: IntegrationType) -> Self {
        self.integration = integration_type;
        self
    }
    /// Reimann sum for computing integral
    pub fn reimann_sum_left(&self) -> Fraction {
        let dx = (self.to - self.from) / self.sampling_rate.into();
        let mut res = Fraction::from(0);
        for x in 0..self.sampling_rate {
            res += dx * (self.equation)(self.from + (x * dx));
            // println!(
            //     "{x} {} - {} {res:?} {dx:?}",
            //     self.from + (x * dx),
            //     (self.equation)(self.from + (x * dx))
            // );
        }
        return res;
    }
    /// Reimann sum for computing integral
    pub fn reimann_trapezoidal_sum(&self) -> Fraction {
        let dx = (self.to - self.from) / self.sampling_rate.into();
        let mut res = Fraction::from(0);
        for x in 0..self.sampling_rate {
            res += dx * (self.equation)(self.from + (x * dx));
            // println!(
            //     "{x} {} - {} {res:?} {dx:?}",
            //     self.from + (x * dx),
            //     (self.equation)(self.from + (x * dx))
            // );
        }
        return res;
    }
    pub fn build(&self) -> Fraction {
        return match self.integration {
            IntegrationType::ReimannSumLeft => self.reimann_sum_left(),
            IntegrationType::ReimannTrapezoidalSum => todo!(),
            IntegrationType::ReimannSumRight => todo!(),
            IntegrationType::ReimannSumMid => todo!(),
        };
    }
}

#[test]
fn integral_test() {
    let x = Integral::new(0.0, 3.0, |x| (10 * x * 10 * x) + 3).build();
    let x = f64::from(x);
    assert_eq!(909f64, x.round());
}
