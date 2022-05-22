use std::ops::{Add, Div, Mul, Sub};

use crate::helper::GCD;

use super::fractions::Fraction;

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut numerator = self.numerator * rhs.numerator;
        let mut denominator = self.denominator * rhs.denominator;
        let gcd = numerator.gcd(denominator);
        numerator /= gcd;
        denominator /= gcd;
        return Fraction::new(numerator, denominator);
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * Fraction::new(rhs.denominator, rhs.numerator)
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut numerator = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        let mut denominator = self.denominator * rhs.denominator;
        let gcd = numerator.gcd(denominator);
        numerator /= gcd;
        denominator /= gcd;
        return Fraction::new(numerator, denominator);
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self * Fraction::new(-rhs.numerator, rhs.numerator)
    }
}
