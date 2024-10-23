//! Provides the `Summary` struct and its display implementation.

use crate::statistics::Statistics;
use crate::vector::Vector;
use comfy_table::{Cell, Table};
use num_traits::Float;
use std::fmt;

/// Struct containing summary statistics of a vector.
pub struct Summary<T> {
  /// Number of elements.
  pub count: usize,
  /// Mean of the vector.
  pub mean: Option<T>,
  /// Standard deviation of the vector.
  pub stddev: Option<T>,
  /// Minimum value.
  pub min: Option<T>,
  /// 25th percentile.
  pub q25: Option<T>,
  /// Median value.
  pub median: Option<T>,
  /// 75th percentile.
  pub q75: Option<T>,
  /// Maximum value.
  pub max: Option<T>,
}

impl<T> fmt::Display for Summary<T>
where
  T: Float + fmt::Display,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut table = Table::new();
    table.set_header(vec!["Statistic", "Value"]);

    table.add_row(vec![Cell::new("Count"), Cell::new(self.count)]);
    table.add_row(vec![
      Cell::new("Mean"),
      Cell::new(format!("{:.4}", self.mean.unwrap_or(T::nan()))),
    ]);
    table.add_row(vec![
      Cell::new("Std Dev"),
      Cell::new(format!("{:.4}", self.stddev.unwrap_or(T::nan()))),
    ]);
    table.add_row(vec![
      Cell::new("Min"),
      Cell::new(format!("{:.4}", self.min.unwrap_or(T::nan()))),
    ]);
    table.add_row(vec![
      Cell::new("25%"),
      Cell::new(format!("{:.4}", self.q25.unwrap_or(T::nan()))),
    ]);
    table.add_row(vec![
      Cell::new("Median"),
      Cell::new(format!("{:.4}", self.median.unwrap_or(T::nan()))),
    ]);
    table.add_row(vec![
      Cell::new("75%"),
      Cell::new(format!("{:.4}", self.q75.unwrap_or(T::nan()))),
    ]);
    table.add_row(vec![
      Cell::new("Max"),
      Cell::new(format!("{:.4}", self.max.unwrap_or(T::nan()))),
    ]);

    write!(f, "{}", table)
  }
}

/// Trait providing a `summary` method for `Vector<T>`.
pub trait SummaryOps<T> {
  fn summary(&self) -> Summary<T>;
}

impl<T> SummaryOps<T> for Vector<T>
where
  T: Float + Copy + PartialOrd + fmt::Display,
{
  fn summary(&self) -> Summary<T> {
    Summary {
      count: self.len(),
      mean: self.mean(),
      stddev: self.stddev(),
      min: self.min(),
      q25: self.quantile(T::from(0.25).unwrap()),
      median: self.median(),
      q75: self.quantile(T::from(0.75).unwrap()),
      max: self.max(),
    }
  }
}
