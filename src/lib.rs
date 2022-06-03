#![allow(unused_macros)]
#![feature(step_trait)]
#![feature(trivial_bounds)]
#![feature(trait_alias)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]

pub mod helper;
pub mod summation;
pub mod traitbounds;

pub mod any_length_num;
pub mod calculus;
pub mod complex;
pub mod fraction;
pub mod linear_algebra;

pub use crate::fraction::fractions as fractions_num;
