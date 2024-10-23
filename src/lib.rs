//! # MathOps
//!
//! `MathOps` is a Rust library providing mathematical and statistical operations on vectors.
//! It supports both `f32` and `f64` types, and includes methods for common statistical measures,
//! normalization, sorting, and vector arithmetic.

pub mod conversion;
pub mod normalize;
pub mod operations;
pub mod sort;
pub mod statistics;
pub mod summary;
pub mod vector;

// Re-exporting for easy access
pub use conversion::*;
pub use normalize::*;
pub use operations::*;
pub use sort::*;
pub use statistics::*;
pub use summary::*;
pub use vector::*;
