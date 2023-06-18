use rug::Float;

pub const PRECISION: u32 = 256;

#[derive(Clone)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}

pub struct CliffordAttractor {
    pub point: Point,
    pub a: Float,
    pub b: Float,
    pub c: Float,
    pub d: Float,
}

pub fn scale(a_min: &Float, a_max: &Float, b_min: u32, b_max: u32, x: &Float) -> i32 {
    let a_diff = Float::with_val(PRECISION, a_max - a_min);
    let x_in_percent = Float::with_val(PRECISION, x - a_min) / &a_diff;
    let x_as_float = Float::with_val(PRECISION, b_max as i32 - b_min as i32) * x_in_percent;
    b_min as i32 + x_as_float.to_integer().unwrap().to_i32().unwrap()
}

pub fn is_between(min: i32, max: i32, x: i32) -> bool {
    (x >= min) & (x <= max)
}

impl Iterator for CliffordAttractor {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.point.clone();

        self.point.x = (self.a.clone() * current.y.clone()).sin()
            + self.c.clone() * (self.a.clone() * current.x.clone()).cos();
        self.point.y = (self.b.clone() * current.x.clone()).sin()
            + self.d.clone() * (self.b.clone() * current.y.clone()).cos();

        Some(current)
    }
}
