

use image::{ImageBuffer, Luma};
use indicatif::ProgressBar;
use rug::Float;
use std::time::Instant;
mod attractor_file;

fn main() {
    let start = Instant::now();
    let attractor = attractor_file::CliffordAttractor {
        point: attractor_file::Point {
            x: Float::with_val(attractor_file::PRECISION, 1.0),
            y: Float::with_val(attractor_file::PRECISION, 1.0),
        },
        a: Float::with_val(attractor_file::PRECISION, -1.7),
        b: Float::with_val(attractor_file::PRECISION, 1.3),
        c: Float::with_val(attractor_file::PRECISION, -0.1),
        d: Float::with_val(attractor_file::PRECISION, -1.2),
    };

    let iterations = 100_000_000;
    let pixels_per_unit = Float::with_val(attractor_file::PRECISION, 4000);
    let x_min = Float::with_val(attractor_file::PRECISION, -2.5);
    let x_max = Float::with_val(attractor_file::PRECISION, 2.5);
    let y_min = Float::with_val(attractor_file::PRECISION, -2.5);
    let y_max = Float::with_val(attractor_file::PRECISION, 2.5);

    let x_diff =  Float::with_val(attractor_file::PRECISION, &x_max - &x_min);
    let img_columns = (x_diff * &pixels_per_unit)
        .to_integer()
        .unwrap()
        .to_usize()
        .unwrap();
    let y_diff = Float::with_val(attractor_file::PRECISION, &y_max - &y_min);
    let img_rows = (y_diff * &pixels_per_unit)
        .to_integer()
        .unwrap()
        .to_usize()
        .unwrap();
    println!("{} {}", img_rows, img_columns);

    let mut pixels: Vec<u64> = vec![0; img_rows * img_columns];
    let update_intervall = 10_000;

    let progress_bar = ProgressBar::new(iterations);
    for (i, point) in attractor.take(iterations as usize).enumerate() {
        let scaled_x = attractor_file::scale(
            &x_min,
            &x_max,
            0,
            img_columns as u32 - 1,
            &point.x,
        );
        let scaled_y = attractor_file::scale(
            &Float::with_val(attractor_file::PRECISION, -&y_max),
            &Float::with_val(attractor_file::PRECISION, -&y_min),
            0,
            img_rows as u32 - 1,
            &point.y,
        );
        if attractor_file::is_between(0, (img_columns - 1) as i32, scaled_x)
            & attractor_file::is_between(0, (img_rows - 1) as i32, scaled_y)
        {
            pixels[scaled_y as usize * img_columns + scaled_x as usize] += 1;
        }

        // updating every iteration is too slow as IO becomes bottleneck
        if i % update_intervall == 0 {
            progress_bar.inc(update_intervall as u64);
        }
    }
    progress_bar.finish();

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
