//! Sorting methods for `Vector<T>`.

use num_traits::{Float, ToPrimitive};
use crate::vector::Vector;

/// Trait providing sorting methods for `Vector<T>`.
pub trait SortOps<T> {
  /// Returns a new sorted vector without modifying the original.
  fn sorted(&self) -> Vector<T>;

  /// Sorts the vector in place.
  fn sort_in_place(&mut self);
}

impl<T> SortOps<T> for Vector<T>
where
  T: Float + ToPrimitive + Copy + PartialOrd,
{
  fn sorted(&self) -> Vector<T> {
    let mut sorted = self.clone();
    sorted.sort_in_place();
    sorted
  }

  fn sort_in_place(&mut self) {
    self.0.sort_by(|a, b| {
      match (a.partial_cmp(b), a.is_nan(), b.is_nan()) {
        (Some(ordering), _, _) => ordering,
        (None, false, true) => std::cmp::Ordering::Less,  // Keep `a` before `b` when `b` is NaN
        (None, true, false) => std::cmp::Ordering::Greater, // Move `a` after `b` when `a` is NaN
        _ => std::cmp::Ordering::Equal,  // Both are NaN, keep equal
      }
    });
  }
}
