use image::{ImageBuffer, Luma};
use indicatif::ProgressBar;
use std::time::Instant;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

struct CliffordAttractor {
    point: Point,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

fn scale(a_min: f64, a_max: f64, b_min: u32, b_max: u32, x: f64) -> u32 {
    let x_in_percent = (x - a_min) / (a_max - a_min);
    b_min + (((b_max - b_min) as f64) * x_in_percent) as u32
}

fn is_between(min: u32, max: u32, x: u32) -> bool {
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

fn main() {
    let start = Instant::now();
    let attractor = CliffordAttractor {
        point: Point { x: 1.0, y: 1.0 },
        a: -1.7,
        b: 1.3,
        c: -0.1,
        d: -1.2,
    };

    let iterations = 5_000_000_000;

    let img_rows = 4000;
    let img_columns = 4000;
    let x_min = -2.5;
    let x_max = 2.5;
    let y_min = -2.5;
    let y_max = 2.5;

    let mut pixels: Vec<u64> = vec![0; img_rows * img_columns];
    let update_intervall = 1000;

    let progress_bar = ProgressBar::new(iterations);
    for (i, point) in attractor.into_iter().take(iterations as usize).enumerate() {
        let scaled_x = scale(x_min, x_max, 0, img_rows as u32 - 1, point.x);
        let scaled_y = scale(y_min, y_max, 0, img_columns as u32 - 1, point.y);
        if is_between(0, (img_rows - 1) as u32, scaled_x)
            & is_between(0, (img_columns - 1) as u32, scaled_y)
        {
            pixels[(scaled_x + scaled_y * img_columns as u32) as usize] += 1;
        }

        // updating every iteration is too slow as IO becomes bottleneck
        if i % update_intervall == 0 {
            progress_bar.inc(update_intervall as u64);
        }
    }
    progress_bar.finish();

    // take log

    pixels = pixels
        .iter()
        .map(|x| (*x as f32).log(1.01) as u64)
        .collect();

    // scale median_pixel_value to factor*u16::MAX
    let mut pixels_copy = pixels.clone();
    pixels_copy.sort_unstable();

    pixels_copy = pixels_copy
        .into_iter()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let max_pixel = pixels
        .iter()
        .map(|x| {
            pixels_copy
                .iter()
                .position(|y| x == y)
                .expect("value not found")
        })
        .max()
        .expect("list empty");

    let gray_image: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(
        img_columns as u32,
        img_rows as u32,
        pixels
            .iter()
            .map(|x| ((*x as f32 / max_pixel as f32) * u16::MAX as f32) as u64)
            .map(|x| u16::MAX - (std::cmp::min(u16::MAX as u64, x) as u16))
            .collect(),
    )
    .unwrap();
    gray_image.save("output.png").expect("Failed to save image");
    let end = Instant::now();
    println!("execution time was {:?}", end - start)
}
