#![allow(unused_macros)]
#![feature(step_trait)]
#![feature(trivial_bounds)]
#![feature(trait_alias)]

pub mod summation;
pub mod traitbounds;

#[cfg(feature = "linear-algebra")]
pub mod matrix_items;
#[cfg(feature = "linear-algebra")]
pub mod vector_items;

#[cfg(feature = "linear-algebra")]
pub use matrix_items::matrix as matrices;
#[cfg(feature = "linear-algebra")]
pub use vector_items::vectors;
