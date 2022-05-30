use std::fmt::{Debug, Display};

use crate::helper::{GetDecimal, GCD};

#[derive(Clone, Copy, PartialOrd)]
pub struct Fraction {
    pub numerator: i128,
    pub denominator: i128,
}

impl Fraction {
    pub fn new(numerator: i128, denominator: i128) -> Self {
        Self {
            numerator,
            denominator,
        }
        .reduce()
    }
    pub fn from_float(value: f64) -> Self {
        let ten_pow = 10_u128.pow(value.get_decimal().0.len() as u32);
        let numerator = (value.trunc() as i128 * ten_pow as i128)
            + (value.get_decimal().0.parse::<i128>().unwrap());
        let denominator = ten_pow as i128;
        if value.get_decimal().1 {
            return Fraction::new(-numerator, denominator).reduce();
        }
        return Fraction::new(numerator, denominator).reduce();
    }
    pub fn reduce(&self) -> Self {
        let mut numerator = self.numerator;
        let mut denominator = self.denominator;
        let gcd = numerator.gcd(denominator);
        numerator /= gcd;
        denominator /= gcd;
        return Fraction {
            numerator,
            denominator,
        };
    }
    pub fn sqrt(&self) -> f64 {
        ((self.numerator * self.denominator) as f64).sqrt() / self.denominator as f64
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator).unwrap();
        f.write_str("")
    }
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.numerator as f64 / self.denominator as f64).unwrap();
        f.write_str("")
    }
}

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        format!("{self:?}") == format!("{other:?}")
    }
}

#[test]
fn fraction_test() {
    let x = Fraction::new(8291, 10000);
    let y = Fraction::from_float(0.08291);
    assert_eq!(x, y);
}
