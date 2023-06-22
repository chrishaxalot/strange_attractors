use rug::Float;
pub const PRECISION: u32 = 500;
pub const ITERATIONS: u32 = 2_000_000;
pub const PIXELS_PER_UNIT: Float = Float::with_val(PRECISION, 1000);
pub const X_MIN: Float = Float::with_val(PRECISION, -1.2);
pub const X_MAX: Float = Float::with_val(PRECISION, 1.2);
pub const Y_MIN: Float = Float::with_val(PRECISION, -2.0);
pub const Y_MAX: Float = Float::with_val(PRECISION, 2.3);
