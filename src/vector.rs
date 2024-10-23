//! Defines the `Vector<T>` struct that wraps `Vec<T>` and provides conversion traits.

use std::ops::{Deref, DerefMut};

/// A wrapper around `Vec<T>` to enable trait implementations.
#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T>(pub Vec<T>);

impl<T> Vector<T> {
  /// Creates a new `Vector<T>` from a `Vec<T>`.
  pub fn new(data: Vec<T>) -> Self {
    Vector(data)
  }

  /// Consumes the `Vector<T>` and returns the inner `Vec<T>`.
  pub fn into_vec(self) -> Vec<T> {
    self.0
  }
}

impl<T> From<Vec<T>> for Vector<T> {
  fn from(vec: Vec<T>) -> Self {
    Vector(vec)
  }
}

impl<T> Into<Vec<T>> for Vector<T> {
  fn into(self) -> Vec<T> {
    self.0
  }
}

impl<T> Deref for Vector<T> {
  type Target = Vec<T>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for Vector<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// Trait to convert `Vec<T>` into `Vector<T>`.
pub trait IntoVector<T> {
  fn into_vector(self) -> Vector<T>;
}

impl<T> IntoVector<T> for Vec<T> {
  fn into_vector(self) -> Vector<T> {
    Vector::new(self)
  }
}

