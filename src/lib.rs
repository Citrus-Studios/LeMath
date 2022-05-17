#![allow(unused_macros)]
#![feature(step_trait)]
#![feature(trivial_bounds)]
#![feature(trait_alias)]

pub mod matrix_items;
pub mod summation;
pub mod traitbounds;
pub mod vector_items;

pub use matrix_items::matrix;
pub use vector_items::vectors;
