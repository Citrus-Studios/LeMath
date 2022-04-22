use std::iter::Step;

macro_rules! impl_trait {
    (impl $trait:ident for $($x:ident)*) => {
        $(impl $trait for $x {})*
    };
}

pub trait Unsigned: PartialEq + Ord + PartialOrd + Step + Sized {
    fn floor(&self) -> Self {
        self.floor()
    }
}
impl_trait!(impl Unsigned for usize u8 u16 u32 u64 u128);

pub trait Signed: PartialEq + Ord + PartialOrd + Step + Sized {
    fn floor(&self) -> Self {
        self.floor()
    }
}
impl_trait!(impl Signed for isize i8 i16 i32 i64 i128);

pub trait Float: PartialEq + PartialOrd + Sized {
    fn floor(&self) -> Self {
        self.floor()
    }
}
impl_trait!(impl Float for f32 f64);

pub trait Integer: PartialEq + Ord + PartialOrd + Sized {
    fn floor(&self) -> Self {
        self.floor()
    }
}
impl_trait!(impl Integer for usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);

pub trait Real: PartialEq + PartialOrd + Sized {
    fn floor(&self) -> Self {
        self.floor()
    }
}
impl_trait!(impl Real for f32 f64 usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);

macro_rules! impl_from_trait {
    (trait $trait:ident: $($x:ident)*) => {
        pub trait $trait: $(From<$x> + )* {}
    };
}

impl_from_trait!(trait FromUnsigned: usize u8 u16 u32 u64 u128);
impl_from_trait!(trait FromSigned: isize i8 i16 i32 i64 i128);
impl_from_trait!(trait FromFloat: f32 f64);
impl_from_trait!(trait FromInteger: usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);
impl_from_trait!(trait FromReal: f32 f64 usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128);
