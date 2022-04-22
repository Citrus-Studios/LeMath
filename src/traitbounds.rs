use std::{iter::Step, ops::{AddAssign, Sub, SubAssign, Div, Add, Mul}};

macro_rules! impl_trait {
    (impl $trait:ident for $($x:ident)*) => {
        $(impl $trait for $x {})*
    };
}

pub trait Unsigned: PartialEq + Ord + PartialOrd + Step + Sized + AddAssign  + Mul + Add + Div + Sub + SubAssign {}
impl_trait!(impl Unsigned for usize u8 u16 u32 u64 u128);

pub trait Signed: PartialEq + Ord + PartialOrd + Step + Sized + AddAssign + Mul + Add + Div + Sub + SubAssign{}
impl_trait!(impl Signed for isize i8 i16 i32 i64 i128);

pub trait Float: PartialEq + PartialOrd + Sized + AddAssign + Mul + Add + Div + Sub + SubAssign {}
impl_trait!(impl Float for f32 f64);

pub trait Integer: PartialEq + Ord + PartialOrd + Sized + AddAssign + Mul + Add + Div + Sub + SubAssign {}
impl_trait!(impl Integer for usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);

pub trait Real: PartialEq + PartialOrd + Sized + AddAssign + Mul + Add + Div + Sub + SubAssign {}
impl_trait!(impl Real for f32 f64 usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);