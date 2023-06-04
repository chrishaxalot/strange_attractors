use image::{ImageBuffer, Luma};
use indicatif::ProgressBar;
use std::time::Instant;

#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

struct CliffordAttractor {
    point: Point,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

fn scale(a_min: f32, a_max: f32, b_min: u32, b_max: u32, x: f32) -> u32 {
    let x_in_percent = (x - a_min) / (a_max - a_min);
    b_min + (((b_max - b_min) as f32) * x_in_percent) as u32
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

    let iterations = 50_000_000_000;

    let img_rows = 2000;
    let img_columns = 2000;
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

        if i % update_intervall == 0 {
            progress_bar.inc(update_intervall as u64);
        }
    }
    progress_bar.finish();

    let max_pixel_value = pixels.iter().max().unwrap();
    pixels = pixels
        .iter()
        .map(|x| (*x as f32) as u64)
        //.map(|x| ((*x as f32) / (*max_pixel_value as f32) * (u16::MAX as f32)) as u64)
        .collect();

    let gray_image: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(
        img_columns as u32,
        img_rows as u32,
        pixels
            .iter()
            .map(|x| *x as u16)
            //.map(|x| (std::cmp::min(u16::MAX as u64, *x) as u16))
            .collect(),
    )
    .unwrap();
    gray_image.save("output.png").expect("Failed to save image");
    let end = Instant::now();
    println!("execution time was {:?}", end - start)
}
