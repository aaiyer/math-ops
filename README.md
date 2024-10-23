# MathOps

`MathOps` is a Rust library providing mathematical and statistical operations on vectors. It supports both `f32` and `f64` types and includes methods for common statistical measures, normalization, sorting, and vector arithmetic.

## Features

- **Statistical Methods**: `mean`, `median`, `variance`, `standard deviation`, `quantile`, `interquartile range (IQR)` and `cumulative sum`.
- **Normalization**: Min-max normalization and standardization (mean 0, standard deviation 1).
- **Sorting Methods**: `sorted` and `sort_in_place`.
- **Vector Arithmetic**: Addition, subtraction, multiplication, division, and modulus with both vectors and scalars using explicit methods.
- **Operator Overloading**: Supports `Vector<T> + Vector<T>`, `Vector<T> - Vector<T>`, etc., using operator overloading.
- **Type Conversion**: Convert between integer types and `f32`/`f64`, and between `f32` and `f64`.
- **Summary Statistics**: Generate a neatly formatted summary of key statistical measures.
- **Seamless Conversion**: Easily wrap and unwrap between `Vec<T>` and `Vector<T>`.

## Why Wrap `Vec<T>` into `Vector<T>`

Wrapping `Vec<T>` into a local `Vector<T>` struct allows us to implement custom traits and methods without violating Rust's orphan rules. This approach is a **zero-cost abstraction**:

- **No Overhead**: The `Vector<T>` struct is a simple wrapper around `Vec<T>`, introducing no runtime overhead.
- **Seamless Integration**: By implementing `Deref` and `DerefMut`, `Vector<T>` can be used just like a `Vec<T>`, supporting indexing, iteration, and other common operations.
- **Enhanced Functionality**: Enables the implementation of custom traits for mathematical and statistical operations directly on `Vector<T>`.

## Usage

```rust
use math_ops::{
  Normalize,
  SortOps,
  Statistics,
  SummaryOps,
  UnwrapToVec,
  Vector,
  VectorOps,
  WrapAsVector,
};

fn main() {
  // Sample data with NaN values
  let data_f64 = vec![1.0_f64, 2.0, f64::NAN, 4.0, 5.0].wrap_as_vector();

  // Statistical operations
  println!("=== Statistical Operations ===");
  println!("Mean (f64): {:?}", data_f64.mean());
  println!("Median (f64): {:?}", data_f64.median());
  println!("Variance (f64): {:?}", data_f64.var());
  println!("Standard Deviation (f64): {:?}", data_f64.stddev());
  println!("IQR (f64): {:?}", data_f64.iqr());
  println!("Quantile(25%) (f64): {:?}", data_f64.quantile(0.25));
  println!("Quantile(95%) (f64): {:?}", data_f64.quantile(0.95));

  // Cumulative Sum
  println!("Cumulative Sum (f64): {:?}", data_f64.cumsum());

  // Summary
  println!("\n=== Summary ===");
  let summary = data_f64.summary();
  println!("{}", summary);

  // Normalization
  println!("\n=== Normalization ===");
  let normalized = data_f64.min_max_normalize();
  println!("Min-Max Normalized: {:?}", normalized);
  let standardized = data_f64.standardize();
  println!("Standardized: {:?}", standardized);

  // Sorting
  println!("\n=== Sorting ===");
  let sorted = data_f64.sorted();
  println!("Sorted: {:?}", sorted);

  // Arithmetic Operations with Vectors
  println!("\n=== Arithmetic Operations with Vectors ===");
  let data2 = vec![5.0_f64, 4.0, 3.0, 2.0, 1.0].wrap_as_vector();
  let sum_vec = &data_f64 + &data2;
  println!("Vector Addition: {:?}", sum_vec);
  let sub_vec = &data_f64 - &data2;
  println!("Vector Subtraction: {:?}", sub_vec);
  let mul_vec = &data_f64 * &data2;
  println!("Vector Multiplication: {:?}", mul_vec);
  let div_vec = &data_f64 / &data2;
  println!("Vector Division: {:?}", div_vec);
  let rem_vec = &data_f64 % &data2;
  println!("Vector Modulus: {:?}", rem_vec);

  // Arithmetic Operations with Scalars
  println!("\n=== Arithmetic Operations with Scalars ===");
  let scalar = 2.0_f64;
  let added = data_f64.add_scalar(scalar);
  println!("Added Scalar: {:?}", added);
  let subtracted = data_f64.sub_scalar(scalar);
  println!("Subtracted Scalar: {:?}", subtracted);
  let multiplied = data_f64.mul_scalar(scalar);
  println!("Multiplied by Scalar: {:?}", multiplied);
  let divided = data_f64.div_scalar(scalar);
  println!("Divided by Scalar: {:?}", divided);
  let modulus = data_f64.rem_scalar(scalar);
  println!("Modulus with Scalar: {:?}", modulus);

  // Type Conversion
  println!("\n=== Type Conversion ===");
  let int_data = vec![1, 2, 3, 4, 5];
  let float_data_f64: Vector<f64> = int_data
    .iter()
    .map(|&x| x as f64)
    .collect::<Vec<_>>()
    .wrap_as_vector();
  let float_data_f32: Vector<f32> = int_data
    .iter()
    .map(|&x| x as f32)
    .collect::<Vec<_>>()
    .wrap_as_vector();
  println!("Converted to f64: {:?}", float_data_f64);
  println!("Converted to f32: {:?}", float_data_f32);

  // Unwrap to Vec
  let original_vec: Vec<f64> = float_data_f64.unwrap_to_vec();
  println!("Unwrapped to Vec<f64>: {:?}", original_vec);
}
```
