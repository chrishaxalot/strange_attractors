#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct CliffordAttractor {
    pub point: Point,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
}

pub fn scale(a_min: f64, a_max: f64, b_min: u32, b_max: u32, x: f64) -> u32 {
    let x_in_percent = (x - a_min) / (a_max - a_min);
    b_min + (((b_max - b_min) as f64) * x_in_percent) as u32
}

pub fn is_between(min: u32, max: u32, x: u32) -> bool {
    (x >= min) & (x <= max)
}

impl Iterator for CliffordAttractor {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.point;

        self.point.x = (self.a * current.y).sin() + self.c * (self.a * current.x).cos();
        self.point.y = (self.b * current.x).sin() + self.d * (self.b * current.y).cos();

        Some(current)
    }
}
