//! Module for type conversions between integers and floating-point types,
//! and between `f32` and `f64`.

/// Trait to convert integer types to floating-point types.
pub trait IntToFloat {
  fn to_f32(&self) -> f32;
  fn to_f64(&self) -> f64;
}

/// Trait to convert floating-point types to integer types.
pub trait FloatToInt {
  fn to_i8(&self) -> i8;
  fn to_i16(&self) -> i16;
  fn to_i32(&self) -> i32;
  fn to_i64(&self) -> i64;
  fn to_u8(&self) -> u8;
  fn to_u16(&self) -> u16;
  fn to_u32(&self) -> u32;
  fn to_u64(&self) -> u64;
}

/// Implement `IntToFloat` for all integer types.
macro_rules! impl_int_to_float {
  ($($t:ty)*) => ($(
    impl IntToFloat for $t {
      fn to_f32(&self) -> f32 {
        *self as f32
      }
      fn to_f64(&self) -> f64 {
        *self as f64
      }
    }
  )*)
}

impl_int_to_float!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize);

/// Implement `FloatToInt` for `f32` and `f64`.
impl FloatToInt for f32 {
  fn to_i8(&self) -> i8 {
    *self as i8
  }

  fn to_i16(&self) -> i16 {
    *self as i16
  }

  fn to_i32(&self) -> i32 {
    *self as i32
  }

  fn to_i64(&self) -> i64 {
    *self as i64
  }

  fn to_u8(&self) -> u8 {
    *self as u8
  }

  fn to_u16(&self) -> u16 {
    *self as u16
  }

  fn to_u32(&self) -> u32 {
    *self as u32
  }

  fn to_u64(&self) -> u64 {
    *self as u64
  }
}

impl FloatToInt for f64 {
  fn to_i8(&self) -> i8 {
    *self as i8
  }

  fn to_i16(&self) -> i16 {
    *self as i16
  }

  fn to_i32(&self) -> i32 {
    *self as i32
  }

  fn to_i64(&self) -> i64 {
    *self as i64
  }

  fn to_u8(&self) -> u8 {
    *self as u8
  }

  fn to_u16(&self) -> u16 {
    *self as u16
  }

  fn to_u32(&self) -> u32 {
    *self as u32
  }

  fn to_u64(&self) -> u64 {
    *self as u64
  }
}

/// Trait to convert between `f32` and `f64`.
pub trait FloatCast {
  fn to_f32_cast(&self) -> f32;
  fn to_f64_cast(&self) -> f64;
}

impl FloatCast for f32 {
  fn to_f32_cast(&self) -> f32 {
    *self
  }

  fn to_f64_cast(&self) -> f64 {
    *self as f64
  }
}

impl FloatCast for f64 {
  fn to_f32_cast(&self) -> f32 {
    *self as f32
  }

  fn to_f64_cast(&self) -> f64 {
    *self
  }
}
