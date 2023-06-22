// Standard library imports
use std::time::Instant;

// External crate imports
use image::{ImageBuffer, Luma};
use indicatif::ProgressBar;
use rug::Float;

// Local module imports
mod clifford_attractor;
mod constants;
mod point;
mod utils;

// Local item uses
use clifford_attractor::CliffordAttractor;
use constants::*;
use point::Point;
use utils::scale;

fn main() {
    let attractor = CliffordAttractor {
        point: Point {
            x: Float::with_val(PRECISION, 1.0),
            y: Float::with_val(PRECISION, 1.0),
        },
        a: Float::with_val(PRECISION, -1.7),
        b: Float::with_val(PRECISION, 1.3),
        c: Float::with_val(PRECISION, -0.1),
        d: Float::with_val(PRECISION, -1.2),
    };

    let iterations = 2_000_000;
    let pixels_per_unit = Float::with_val(PRECISION, 1000);
    let x_min = Float::with_val(PRECISION, -1.2);
    let x_max = Float::with_val(PRECISION, 1.2);
    let y_min = Float::with_val(PRECISION, -2.0);
    let y_max = Float::with_val(PRECISION, 2.3);

    let x_diff = Float::with_val(PRECISION, &x_max - &x_min);
    let img_width = (x_diff * &pixels_per_unit)
        .to_integer()
        .unwrap()
        .to_usize()
        .unwrap();

    // start calculating the pixels
    let start = Instant::now();
    let y_diff = Float::with_val(PRECISION, &y_max - &y_min);
    let img_height = (y_diff * &pixels_per_unit)
        .to_integer()
        .unwrap()
        .to_usize()
        .unwrap();
    println!("{} {}", img_height, img_width);

    let mut pixels: Vec<u64> = vec![0; img_height * img_width];
    let update_intervall = 1_000;

    let progress_bar = ProgressBar::new(iterations);
    for (i, point) in attractor.take(iterations as usize).enumerate() {
        let scaled_x = scale(&x_min, &x_max, 0, img_width as u32 - 1, &point.x);
        let scaled_y = scale(
            &Float::with_val(PRECISION, -&y_max),
            &Float::with_val(PRECISION, -&y_min),
            0,
            img_height as u32 - 1,
            &point.y,
        );
        if (0..img_width as i32).contains(&scaled_x) & (0..img_height as i32).contains(&scaled_y) {
            pixels[scaled_y as usize * img_width + scaled_x as usize] += 1;
        }

        // updating every iteration is too slow as IO becomes bottleneck
        if i % update_intervall == 0 {
            progress_bar.inc(update_intervall as u64);
        }
    }
    progress_bar.finish();

    // start progressing the pixels
    pixels = pixels.iter().map(|x| (*x as f32).log(1.5) as u64).collect();

    // scale median_pixel_value to factor*u16::MAX
    let mut unique_pixels: Vec<u64> = pixels
        .iter()
        .cloned()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    unique_pixels.sort_unstable();

    let pixels: Vec<u64> = pixels
        .iter()
        .map(|x| {
            unique_pixels
                .iter()
                .position(|y| x == y)
                .expect("value not found") as u64
        })
        .collect();

    let max_pixel = *pixels.iter().max().expect("no max found");

    let gray_image: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(
        img_width as u32,
        img_height as u32,
        pixels
            .iter()
            .map(|x| ((*x as f32 / max_pixel as f32) * u16::MAX as f32) as u64)
            .map(|x| u16::MAX - (std::cmp::min(u16::MAX as u64, x) as u16))
            .collect(),
    )
    .unwrap();

    gray_image
        .save(format!("img/clifford_{iterations}_{PRECISION}.png"))
        .expect("Failed to save image");
    gray_image
        .save("img/output.png")
        .expect("Failed to save image");
    let end = Instant::now();
    println!("execution time was {:?}", end - start)
}
