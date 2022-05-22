use crate::helper::{GetDecimal, GCD};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
        let ten_pow = 10_u32.pow(format!("{}", value.get_decimal()).len() as u32);
        let numerator = (value.trunc() as i128 * ten_pow as i128) + (value.get_decimal() as i128);
        let denominator = ten_pow as i128;
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
}

#[test]
fn fraction_test() {
    let x = Fraction::new(8291, 10000);
    let y = Fraction::from_float(0.08291);
    assert_eq!(x, y);
}
