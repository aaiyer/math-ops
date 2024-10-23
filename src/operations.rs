//! Arithmetic operations for `Vector<T>`.

use crate::vector::Vector;
use num_traits::{Float, Num};
use std::ops::{Add, Div, Mul, Rem, Sub};

/// Trait providing arithmetic operations for `Vector<T>`.
pub trait VectorOps<T> {
  /// Adds another vector to this vector and returns a new vector.
  fn add_vec(&self, other: &Vector<T>) -> Vector<T>;

  /// Subtracts another vector from this vector and returns a new vector.
  fn sub_vec(&self, other: &Vector<T>) -> Vector<T>;

  /// Multiplies this vector with another vector element-wise and returns a new vector.
  fn mul_vec(&self, other: &Vector<T>) -> Vector<T>;

  /// Divides this vector by another vector element-wise and returns a new vector.
  fn div_vec(&self, other: &Vector<T>) -> Vector<T>;

  /// Computes the modulus of this vector by another vector element-wise and returns a new vector.
  fn rem_vec(&self, other: &Vector<T>) -> Vector<T>;

  /// Adds a scalar to each element of the vector and returns a new vector.
  fn add_scalar(&self, scalar: T) -> Vector<T>;

  /// Subtracts a scalar from each element of the vector and returns a new vector.
  fn sub_scalar(&self, scalar: T) -> Vector<T>;

  /// Multiplies each element of the vector by a scalar and returns a new vector.
  fn mul_scalar(&self, scalar: T) -> Vector<T>;

  /// Divides each element of the vector by a scalar and returns a new vector.
  fn div_scalar(&self, scalar: T) -> Vector<T>;

  /// Computes the modulus of each element of the vector by a scalar and returns a new vector.
  fn rem_scalar(&self, scalar: T) -> Vector<T>;
}

impl<T> VectorOps<T> for Vector<T>
where
  T: Num + Copy + PartialOrd + Float,
{
  fn add_vec(&self, other: &Vector<T>) -> Vector<T> {
    assert_eq!(
      self.len(),
      other.len(),
      "Vectors must be of the same length for addition."
    );
    let data = self
      .iter()
      .zip(other.iter())
      .map(|(&a, &b)| a + b)
      .collect();
    Vector::new(data)
  }

  fn sub_vec(&self, other: &Vector<T>) -> Vector<T> {
    assert_eq!(
      self.len(),
      other.len(),
      "Vectors must be of the same length for subtraction."
    );
    let data = self
      .iter()
      .zip(other.iter())
      .map(|(&a, &b)| a - b)
      .collect();
    Vector::new(data)
  }

  fn mul_vec(&self, other: &Vector<T>) -> Vector<T> {
    assert_eq!(
      self.len(),
      other.len(),
      "Vectors must be of the same length for multiplication."
    );
    let data = self
      .iter()
      .zip(other.iter())
      .map(|(&a, &b)| a * b)
      .collect();
    Vector::new(data)
  }

  fn div_vec(&self, other: &Vector<T>) -> Vector<T> {
    assert_eq!(
      self.len(),
      other.len(),
      "Vectors must be of the same length for division."
    );
    let data = self
      .iter()
      .zip(other.iter())
      .map(|(&a, &b)| a / b)
      .collect();
    Vector::new(data)
  }

  fn rem_vec(&self, other: &Vector<T>) -> Vector<T> {
    assert_eq!(
      self.len(),
      other.len(),
      "Vectors must be of the same length for modulus."
    );
    let data = self
      .iter()
      .zip(other.iter())
      .map(|(&a, &b)| a % b)
      .collect();
    Vector::new(data)
  }

  fn add_scalar(&self, scalar: T) -> Vector<T> {
    let data = self.iter().map(|&x| x + scalar).collect();
    Vector::new(data)
  }

  fn sub_scalar(&self, scalar: T) -> Vector<T> {
    let data = self.iter().map(|&x| x - scalar).collect();
    Vector::new(data)
  }

  fn mul_scalar(&self, scalar: T) -> Vector<T> {
    let data = self.iter().map(|&x| x * scalar).collect();
    Vector::new(data)
  }

  fn div_scalar(&self, scalar: T) -> Vector<T> {
    let data = self.iter().map(|&x| x / scalar).collect();
    Vector::new(data)
  }

  fn rem_scalar(&self, scalar: T) -> Vector<T> {
    let data = self.iter().map(|&x| x % scalar).collect();
    Vector::new(data)
  }
}

/// Implement operator overloading for `Vector<T> + Vector<T>`.
impl<T> Add for &Vector<T>
where
  T: Num + Copy + PartialOrd + Float,
{
  type Output = Vector<T>;

  fn add(self, rhs: &Vector<T>) -> Vector<T> {
    self.add_vec(rhs)
  }
}

/// Implement operator overloading for `Vector<T> - Vector<T>`.
impl<T> Sub for &Vector<T>
where
  T: Num + Copy + PartialOrd + Float,
{
  type Output = Vector<T>;

  fn sub(self, rhs: &Vector<T>) -> Vector<T> {
    self.sub_vec(rhs)
  }
}

/// Implement operator overloading for `Vector<T> * Vector<T>`.
impl<T> Mul for &Vector<T>
where
  T: Num + Copy + PartialOrd + Float,
{
  type Output = Vector<T>;

  fn mul(self, rhs: &Vector<T>) -> Vector<T> {
    self.mul_vec(rhs)
  }
}

/// Implement operator overloading for `Vector<T> / Vector<T>`.
impl<T> Div for &Vector<T>
where
  T: Num + Copy + PartialOrd + Float,
{
  type Output = Vector<T>;

  fn div(self, rhs: &Vector<T>) -> Vector<T> {
    self.div_vec(rhs)
  }
}

/// Implement operator overloading for `Vector<T> % Vector<T>`.
impl<T> Rem for &Vector<T>
where
  T: Num + Copy + PartialOrd + Float,
{
  type Output = Vector<T>;

  fn rem(self, rhs: &Vector<T>) -> Vector<T> {
    self.rem_vec(rhs)
  }
}
