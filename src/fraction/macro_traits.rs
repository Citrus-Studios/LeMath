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


        )*
    };
}

from_fraction!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);

impl Mul<i32> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: i32) -> Self::Output {
        self * (rhs as f64).into()
    }
}
