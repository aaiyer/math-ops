use math_ops::{IntoVector, Normalize, SortOps, Statistics, SummaryOps, Vector, VectorOps};

fn main() {
  // Sample data with NaN values
  let data_f64 = vec![1.0_f64, 2.0, f64::NAN, 4.0, 5.0].into_vector();

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
  let data2 = vec![5.0_f64, 4.0, 3.0, 2.0, 1.0].into_vector();
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
    .into_vector();
  let float_data_f32: Vector<f32> = int_data
    .iter()
    .map(|&x| x as f32)
    .collect::<Vec<_>>()
    .into_vector();
  println!("Converted to f64: {:?}", float_data_f64);
  println!("Converted to f32: {:?}", float_data_f32);

  // Unwrap to Vec
  let original_vec: Vec<f64> = float_data_f64.into_vec();
  println!("Unwrapped to Vec<f64>: {:?}", original_vec);
}
