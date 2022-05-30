use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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

impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        (self * Fraction::new(rhs.denominator, rhs.numerator)).reduce()
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut numerator = (self.numerator * rhs.denominator) + (rhs.numerator * self.denominator);
        let mut denominator = self.denominator * rhs.denominator;
        let gcd = numerator.gcd(denominator);
        numerator /= gcd;
        denominator /= gcd;
        return Fraction::new(numerator, denominator);
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self + Fraction::new(-rhs.numerator, rhs.denominator)).reduce()
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Neg for Fraction {
    type Output = Fraction;

    fn neg(self) -> Self::Output {
        Fraction::new(-self.numerator, self.denominator)
    }
}

#[test]
fn fraction_math_test() {
    let x = Fraction::from(10.20);
    let y = Fraction::from(2.340);
    assert_eq!(x * y, 23.868.into());
    assert_eq!(x / y, Fraction::new(170, 39));
    assert_eq!(x + y, 12.54.into());
    assert_eq!(x - y, 7.86.into());
}
