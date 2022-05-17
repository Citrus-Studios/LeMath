#[cfg(feature = "linear-algebra")]
pub mod matrix_items;
#[cfg(feature = "linear-algebra")]
pub mod vector_items;

#[cfg(feature = "linear-algebra")]
pub use matrix_items::matrices;
#[cfg(feature = "linear-algebra")]
pub use vector_items::vectors;

// The Linear Algebra Macros
#[cfg(feature = "linear-algebra")]
pub use matrix_items::matrices::matrix;
#[cfg(feature = "linear-algebra")]
pub use vector_items::vectors::vector;
