// use std::marker::PhantomData;

// use crate::{fraction::fractions::Fraction, summation::sum_extra, traitbounds::Real};

// /// Calculus Integral
// /// Find the area underneath a curve\
// #[derive(Clone, Copy, PartialEq, PartialOrd)]
// pub struct Integral {
//     integration: IntegrationType,
//     equation: fn(f64) -> f64,
//     from: f64,
//     to: f64,
//     sampling_rate: i64,
// }

// /// Integration type for Integrals
// #[derive(Clone, Copy, PartialEq, PartialOrd)]
// pub enum IntegrationType {
//     ReimannSum,
//     ReimannTrapozoidalSum,
// }

// impl Integral {
//     pub fn new(from: f64, to: f64, equation: fn(f64) -> f64) -> Self {
//         Self {
//             integration: IntegrationType::ReimannSum,
//             equation,
//             from,
//             to,
//             sampling_rate: 32,
//         }
//     }
//     pub fn set_sampling_rate(mut self, sampling_rate: i64) -> Self {
//         self.sampling_rate = sampling_rate;
//         self
//     }
//     pub fn reimann_sum(&self) -> f64 {
//         let dx = Fraction::new((self.to - self.from) as i128, self.sampling_rate as i128);
//         println!("{dx}");
//         let mut res = Fraction::from(0.0);
//         for x in 1..=self.sampling_rate {
//             println!("{x} {res}");
//             res += dx * Fraction::from(self.equation.call((f64::from(dx) * x as f64,)));
//         }
//         return f64::from(res);
//     }
//     pub fn build(&self) -> f64 {
//         return match self.integration {
//             IntegrationType::ReimannSum => self.reimann_sum(),
//             IntegrationType::ReimannTrapozoidalSum => todo!(),
//         };
//     }
// }

// #[test]
// fn integral_test() {
//     let x = Integral::new(0.0, 3.0, |x| -> f64 {
//         return x;
//     })
//     .build();
//     assert_eq!(4.5, x);
// }
