use std::ops::Mul;

use super::fractions::Fraction;

macro_rules! from_fraction {
    ($($x:ty)*) => {
        $(
            impl From<$x> for Fraction {
                fn from(from: $x) -> Self {
                    Fraction::from_float(from as f64)
                }
            }

            impl From<Fraction> for $x {
                fn from(from: Fraction) -> Self {
                    from.numerator as $x / from.denominator as $x
                }
            }


            impl Mul<$x> for Fraction {
                type Output = Fraction;

                fn mul(self, rhs: $x) -> Self::Output {
                    self * Fraction::from(rhs as f64)
                }
            }

            impl Mul<Fraction> for $x {
                type Output = Fraction;

                fn mul(self, rhs: Fraction) -> Self::Output {
                    Fraction::from(self as f64) * rhs
                }
            }
        )*
    };
}

from_fraction!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);
