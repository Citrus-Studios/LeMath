use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use super::fractions::Fraction;

impl AddAssign for Fraction {
    /// This adds two fractions together and then assigns the result to the original fraction
    ///
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    ///
    /// let mut x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    ///
    /// x += y;
    /// assert_eq!(x, Fraction::new(7, 6));
    /// ```
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for Fraction {
    /// This subtracts two fractions and then assigns the result to the original fraction
    ///
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    ///
    /// let mut x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    ///
    /// x -= y;
    /// assert_eq!(x, Fraction::new(-1, 6));
    /// ```
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl MulAssign for Fraction {
    /// This multiplies two fractions and then assigns the result to the original fraction
    ///
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    ///     
    /// let mut x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    ///
    /// x *= y;
    /// assert_eq!(x, Fraction::new(1, 3));
    /// ```
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl DivAssign for Fraction {
    /// This divides two fractions and then assigns the result to the original fraction
    ///
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    ///
    /// let mut x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    ///
    /// x /= y;
    /// assert_eq!(x, Fraction::new(3, 4));
    /// ```
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl Add for Fraction {
    type Output = Self;

    /// This adds two fractions together
    ///
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    ///
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    ///
    /// assert_eq!(x + y, Fraction::new(7, 6));
    /// ```
    fn add(self, other: Self) -> Self {
        return self.add_number(other);
    }
}

impl Sub for Fraction {
    type Output = Self;

    /// This subtracts two fractions
    ///
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    ///
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    ///
    /// assert_eq!(x - y, Fraction::new(-1, 6));
    /// ```
    fn sub(self, other: Self) -> Self {
        return self.sub_number(other);
    }
}

impl Mul for Fraction {
    type Output = Self;

    /// This multiplies two fractions together
    ///
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    ///
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    ///
    /// assert_eq!(x * y, Fraction::new(1, 3));
    /// ```
    fn mul(self, other: Self) -> Self {
        return self.mul_number(other);
    }
}

impl Div for Fraction {
    type Output = Self;

    /// This divides two fractions
    ///
    /// # Examples
    /// ```
    /// use lemonmath::fraction::Fraction;
    ///
    /// let x = Fraction::new(1, 2);
    /// let y = Fraction::new(2, 3);
    ///
    /// assert_eq!(x / y, Fraction::new(3, 4));
    /// ```
    fn div(self, other: Self) -> Self {
        return self.div_number(other);
    }
}
