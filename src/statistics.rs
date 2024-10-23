//! Statistical methods for `Vector<T>`.

use crate::vector::Vector;
use num_traits::{Float, ToPrimitive};
use crate::IntoVector;

/// Trait definition for Statistics, generic over type T.
/// T is expected to be a floating-point type like f32 or f64.
pub trait Statistics<T> {
  /// Computes the mean (average) of the data.
  /// Returns an Option<T> where None represents an empty dataset.
  fn mean(&self) -> Option<T>;

  /// Computes the variance of the data.
  /// Returns an Option<T>, where None represents an empty dataset.
  /// Variance is the average of the squared deviations from the mean.
  fn var(&self) -> Option<T>;

  /// Computes the standard deviation of the data.
  /// Returns an Option<T>, where None represents an empty dataset.
  /// Standard deviation is the square root of the variance.
  fn stddev(&self) -> Option<T>;

  /// Computes the median of the data.
  /// Returns an Option<T>, where None represents an empty dataset.
  /// The median is the value separating the higher half from the lower half.
  fn median(&self) -> Option<T>;

  /// Computes the quantile for the given fraction `q`.
  /// `q` is expected to be a floating-point value between 0 and 1.
  /// Returns an Option<T> where None represents an empty dataset.
  /// For example, q = 0.5 gives the median, q = 0.25 gives the 25th percentile.
  fn quantile(&self, q: T) -> Option<T>;

  /// Computes the interquartile range (IQR) of the data.
  /// Returns an Option<T>, where None represents an empty dataset.
  /// IQR is the range between the 25th percentile and 75th percentile.
  fn iqr(&self) -> Option<T>;

  /// Returns the minimum value in the dataset, ignoring NaN values.
  /// Returns an Option<T>, where None represents an empty dataset.
  fn min(&self) -> Option<T>;

  /// Returns the maximum value in the dataset, ignoring NaN values.
  /// Returns an Option<T>, where None represents an empty dataset.
  fn max(&self) -> Option<T>;

  /// Computes the cumulative sum of the data.
  /// Returns a `Vector<T>`, where each element is the cumulative sum up to that index.
  /// NaN values are ignored in the summation.
  fn cumsum(&self) -> Vector<T>;
}

impl<T> Statistics<T> for Vector<T>
where
  T: Float + ToPrimitive + Copy + PartialOrd,
{
  fn mean(&self) -> Option<T> {
    let mut sum = T::zero();
    let mut count = 0;
    for &x in self.iter() {
      if !x.is_nan() {
        sum = sum + x;
        count += 1;
      }
    }
    if count == 0 {
      None
    } else {
      Some(sum / T::from(count).unwrap())
    }
  }

  fn var(&self) -> Option<T> {
    let mean = self.mean()?;
    let mut sum_sq_diff = T::zero();
    let mut count = 0;
    for &x in self.iter() {
      if !x.is_nan() {
        sum_sq_diff = sum_sq_diff + (x - mean) * (x - mean);
        count += 1;
      }
    }
    if count < 2 {
      None
    } else {
      Some(sum_sq_diff / T::from(count).unwrap())
    }
  }

  fn stddev(&self) -> Option<T> {
    self.var().map(|v| v.sqrt())
  }

  fn median(&self) -> Option<T> {
    let mut non_nan_values: Vec<T> = self.iter().cloned().filter(|x| !x.is_nan()).collect();
    let n = non_nan_values.len();
    if n == 0 {
      return None;
    }
    non_nan_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = n / 2;
    if n % 2 == 0 {
      Some((non_nan_values[mid - 1] + non_nan_values[mid]) / T::from(2.0).unwrap())
    } else {
      Some(non_nan_values[mid])
    }
  }

  fn quantile(&self, q: T) -> Option<T> {
    if q < T::zero() || q > T::one() {
      return None;
    }
    let mut non_nan_values: Vec<T> = self.iter().cloned().filter(|x| !x.is_nan()).collect();
    let n = non_nan_values.len();
    if n == 0 {
      return None;
    }
    non_nan_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pos = q * T::from(n - 1).unwrap();
    let pos_floor = pos.floor();
    let pos_ceil = pos.ceil();
    let weight = pos - pos_floor;
    let idx_floor = pos_floor.to_usize()?;
    let idx_ceil = pos_ceil.to_usize()?;
    if idx_floor == idx_ceil {
      Some(non_nan_values[idx_floor])
    } else {
      Some(
        non_nan_values[idx_floor]
          + (non_nan_values[idx_ceil] - non_nan_values[idx_floor]) * weight,
      )
    }
  }

  fn iqr(&self) -> Option<T> {
    let q75 = self.quantile(T::from(0.75).unwrap())?;
    let q25 = self.quantile(T::from(0.25).unwrap())?;
    Some(q75 - q25)
  }

  fn min(&self) -> Option<T> {
    self.iter()
      .cloned()
      .filter(|x| !x.is_nan())
      .min_by(|a, b| a.partial_cmp(b).unwrap())
  }

  fn max(&self) -> Option<T> {
    self.iter()
      .cloned()
      .filter(|x| !x.is_nan())
      .max_by(|a, b| a.partial_cmp(b).unwrap())
  }

  fn cumsum(&self) -> Vector<T> {
    let mut cum_sum = T::zero();
    let mut result = Vec::with_capacity(self.len());
    for &x in self.iter() {
      if !x.is_nan() {
        cum_sum = cum_sum + x;
      }
      result.push(cum_sum);
    }
    result.into_vector()
  }
}
