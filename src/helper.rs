/// A trait to losslessly get the decimal part of a float
///
/// # Examples
/// ```
/// use lemonmath::helper::GetDecimal;
///
/// let x = 1.12;
///
/// assert_eq!(x.get_decimal(), 12);
/// ```
pub trait GetDecimal {
    fn get_decimal(&self) -> u128;
}

macro_rules! impl_get_decimal {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
            fn get_decimal(&self) -> u128 {
                if self.floor().to_string() != self.to_string() {
                    let decimalstring = &self.to_string()[(self.floor()).to_string().len()+1..];
                    let decimalnumber = decimalstring.parse::<u128>().unwrap();
                    return decimalnumber;
                } else {
                    return 0u128;
                }
            }
        }
    )*)
}

impl_get_decimal!(GetDecimal for f32 f64);

/// This is a helper trait to find gcd of two numbers
///
/// # Examples
/// ```
/// use lemonmath::helper::GCD;
///
/// let x = (2, 4);
///
/// assert_eq!(x.0.gcd(x.1), 2);
/// ```
pub trait GCD {
    fn gcd(self, other: Self) -> Self;
}

macro_rules! impl_gcd {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
            // Copyright 2014-2016 The Rust Project Developers. See the COPYRIGHT
            // file at the top-level directory of this distribution and at
            // http://rust-lang.org/COPYRIGHT.
            //
            // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
            // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
            // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
            // option. This file may not be copied, modified, or distributed
            // except according to those terms.
            fn gcd(self, other: Self) -> Self {
                // Use Stein's algorithm
                let mut m = self;
                let mut n = other;
                if m == 0 || n == 0 {
                    return (m | n).abs();
                }

                // find common factors of 2
                let shift = (m | n).trailing_zeros();

                // The algorithm needs positive numbers, but the minimum value
                // can't be represented as a positive one.
                // It's also a power of two, so the gcd can be
                // calculated by bitshifting in that case

                // Assuming two's complement, the number created by the shift
                // is positive for all numbers except gcd = abs(min value)
                // The call to .abs() causes a panic in debug mode
                if m == Self::min_value() || n == Self::min_value() {
                    return ((1 << shift) as $t).abs();
                }

                // guaranteed to be positive now, rest like unsigned algorithm
                m = m.abs();
                n = n.abs();

                // divide n and m by 2 until odd
                m >>= m.trailing_zeros();
                n >>= n.trailing_zeros();

                while m != n {
                    if m > n {
                        m -= n;
                        m >>= m.trailing_zeros();
                    } else {
                        n -= m;
                        n >>= n.trailing_zeros();
                    }
                }
                m << shift
            }
        }
    )*)
}

impl_gcd!(GCD for i8 i16 i32 i64 i128 isize);
