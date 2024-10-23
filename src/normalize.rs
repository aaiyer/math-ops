//! Normalization methods for `Vector<T>`.

use crate::statistics::Statistics;
use crate::vector::Vector;
use num_traits::{Float, FromPrimitive};

/// Trait providing normalization methods for `Vector<T>`.
pub trait Normalize<T> {
  fn min_max_normalize(&self) -> Vector<T>;
  fn standardize(&self) -> Vector<T>;
}

impl<T> Normalize<T> for Vector<T>
where
  T: Float + FromPrimitive + Copy + PartialOrd,
{
  fn min_max_normalize(&self) -> Vector<T> {
    let non_nan_values: Vec<&T> = self.iter().filter(|&&x| !x.is_nan()).collect();
    if non_nan_values.is_empty() {
      return Vector::new(vec![T::zero(); self.len()]);
    }
    let min = **non_nan_values.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max = **non_nan_values.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let range = max - min;
    if range == T::zero() {
      return Vector::new(vec![T::zero(); self.len()]);
    }
    let normalized = self
      .iter()
      .map(|&x| {
        if x.is_nan() {
          T::zero()
        } else {
          (x - min) / range
        }
      })
      .collect();
    Vector::new(normalized)
  }

  fn standardize(&self) -> Vector<T> {
    let mean = self.mean().unwrap_or(T::zero());
    let stddev = self.stddev().unwrap_or(T::one());
    if stddev == T::zero() {
      return Vector::new(vec![T::zero(); self.len()]);
    }
    let standardized = self
      .iter()
      .map(|&x| {
        if x.is_nan() {
          T::zero()
        } else {
          (x - mean) / stddev
        }
      })
      .collect();
    Vector::new(standardized)
  }
}
