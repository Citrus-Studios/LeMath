use std::{iter::Step, ops::{AddAssign, Sub, SubAssign, Div, Add, Mul, MulAssign}};

macro_rules! impl_trait {
    (impl $trait:ident for $($x:ident)*) => {
        $(impl $trait for $x {})*
    };
}

pub trait NumberTraitBounds = PartialEq + PartialOrd + Sized + AddAssign + Mul + Add + Div + Sub + SubAssign + MulAssign;

pub trait Unsigned: NumberTraitBounds + Ord {}
impl_trait!(impl Unsigned for usize u8 u16 u32 u64 u128);

pub trait Signed: NumberTraitBounds + Ord {}
impl_trait!(impl Signed for isize i8 i16 i32 i64 i128);

pub trait Float: NumberTraitBounds {}
impl_trait!(impl Float for f32 f64);

pub trait Integer: NumberTraitBounds + Ord {}
impl_trait!(impl Integer for usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);

pub trait Real: NumberTraitBounds {}
impl_trait!(impl Real for f32 f64 usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);