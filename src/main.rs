use image::{ImageBuffer, Luma};
use indicatif::ProgressBar;
use std::collections::BTreeSet;
use std::time::Instant;

mod attractor_file;

const X_MIN: f64 = -2.5;
const X_MAX: f64 = 2.5;
const Y_MIN: f64 = -2.5;
const Y_MAX: f64 = 2.5;
const IMG_ROWS: usize = 2000;
const IMG_COLUMNS: usize = 2000;
const ITERATIONS: u64 = 50_000_000;
const UPDATE_INTERVAL: u64 = ITERATIONS / 100;

fn main() {
    let start = Instant::now();

    let attractor = attractor_file::CliffordAttractor {
        point: attractor_file::Point { x: 1.0, y: 1.0 },
        a: -1.7,
        b: 1.3,
        c: -0.1,
        d: -1.2,
    };

    let mut pixels: Vec<u64> = vec![0; IMG_ROWS * IMG_COLUMNS];
    let progress_bar = ProgressBar::new(ITERATIONS);

    for (i, point) in attractor.into_iter().take(ITERATIONS as usize).enumerate() {
        let scaled_x = attractor_file::scale(X_MIN, X_MAX, 0, (IMG_ROWS - 1) as u32, point.x);
        let scaled_y = attractor_file::scale(Y_MIN, Y_MAX, 0, (IMG_COLUMNS - 1) as u32, point.y);
        if attractor_file::is_between(0, (IMG_ROWS - 1) as u32, scaled_x)
            & attractor_file::is_between(0, (IMG_COLUMNS - 1) as u32, scaled_y)
        {
            pixels[(scaled_x + scaled_y * IMG_COLUMNS as u32) as usize] += 1;
        }

        if i % UPDATE_INTERVAL as usize == 0 {
            progress_bar.inc(UPDATE_INTERVAL);
        }
    }
    progress_bar.finish();

    pixels = pixels.iter().map(|x| (*x as f32).log(1.5) as u64).collect();
    let unique_pixels: BTreeSet<u64> = pixels.iter().cloned().collect();

    let pixels: Vec<u64> = pixels
        .iter()
        .map(|x| unique_pixels.iter().position(|y| x == y).unwrap_or(0) as u64)
        .collect();

    let max_pixel = pixels.iter().max().unwrap_or(&0);

    let gray_image: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(
        IMG_COLUMNS as u32,
        IMG_ROWS as u32,
        pixels
            .iter()
            .map(|x| ((*x as f32 / *max_pixel as f32) * u16::MAX as f32) as u64)
            .map(|x| u16::MAX - (std::cmp::min(u16::MAX as u64, x) as u16))
            .collect(),
    )
    .expect("Failed to create image buffer");

    gray_image.save("output.png").expect("Failed to save image");

    let end = Instant::now();
    println!("execution time was {:?}", end - start);
}
