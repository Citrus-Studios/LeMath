/// Calculus Integral
/// Find the area underneath a curve\
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Integral {
    integration: IntegrationType,
    equation: fn(f64) -> f64,
    from: f64,
    to: f64,
    sampling_rate: i64,
}

/// Integration type for Integrals
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum IntegrationType {
    ReimannSum,
    ReimannTrapozoidalSum,
}

impl Integral {
    pub fn new(from: f64, to: f64, equation: fn(f64) -> f64) -> Self {
        Self {
            integration: IntegrationType::ReimannSum,
            equation,
            from,
            to,
            sampling_rate: 32,
        }
    }
    pub fn set_sampling_rate(mut self, sampling_rate: i64) -> Self {
        self.sampling_rate = sampling_rate;
        self
    }
    pub fn reimann_sum(&self) -> f64 {
        let dx = (self.to - self.from) / self.sampling_rate as f64;
        println!("{dx}");
        let mut res = 0.0;
        for x in 1..=self.sampling_rate {
            println!("{x} {res}");
            res += dx * self.equation.call((dx * x as f64,));
        }
        return res;
    }
    pub fn build(&self) -> f64 {
        return match self.integration {
            IntegrationType::ReimannSum => self.reimann_sum(),
            IntegrationType::ReimannTrapozoidalSum => todo!(),
        };
    }
}

#[test]
fn integral_test() {
    let x = Integral::new(0.0, 3.0, |x| -> f64 {
        return x;
    })
    .build();
    assert_eq!(4.5, x);
}
