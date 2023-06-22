use super::point::Point;
use rug::Float;

pub struct CliffordAttractor {
    pub point: Point,
    pub a: Float,
    pub b: Float,
    pub c: Float,
    pub d: Float,
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
