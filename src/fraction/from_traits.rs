use super::fractions::Fraction;

impl From<f64> for Fraction {
    fn from(from: f64) -> Self {
        Fraction::from_float(from)
    }
}
impl From<f32> for Fraction {
    fn from(from: f32) -> Self {
        Fraction::from_float(from as f64)
    }
}

impl From<Fraction> for f64 {
    fn from(from: Fraction) -> Self {
        from.numerator as f64 / from.denominator as f64
    }
}

impl From<Fraction> for f32 {
    fn from(from: Fraction) -> Self {
        from.numerator as f32 / from.denominator as f32
    }
}
