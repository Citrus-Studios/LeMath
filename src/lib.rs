#![allow(unused_macros)]
#![feature(step_trait)]
#![feature(trivial_bounds)]
#![feature(trait_alias)]

pub mod summation;
pub mod traitbounds;

#[cfg(feature = "linear-algebra")]
pub mod linear_algebra;
