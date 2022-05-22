use crate::helper::GCD;

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
