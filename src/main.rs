

use image::{ImageBuffer, Luma};
use indicatif::ProgressBar;
use rug::Float;
use std::time::Instant;
mod attractor_file;

const X_MIN: f64 = -2.5;
const X_MAX: f64 = 2.5;
const Y_MIN: f64 = -2.5;
const Y_MAX: f64 = 2.5;
const IMG_ROWS: usize = 2000;
const IMG_COLUMNS: usize = 2000;
const ITERATIONS: u64 = 500_000_000;

fn point_to_vec(points: &Vec<Point>) -> Vec<u64> {
    let mut pixels: Vec<u64> = vec![0; IMG_ROWS * IMG_COLUMNS];

    points.iter().for_each(|point| {
        let scaled_x = scale(X_MIN, X_MAX, 0, (IMG_ROWS - 1) as u32, point.x);
        let scaled_y = scale(Y_MIN, Y_MAX, 0, (IMG_COLUMNS - 1) as u32, point.y);
        if is_between(0, (IMG_ROWS - 1) as u32, scaled_x)
            & is_between(0, (IMG_COLUMNS - 1) as u32, scaled_y)
        {
            let pixel_position = (scaled_x + scaled_y * IMG_COLUMNS as u32) as usize;
            pixels[pixel_position] += 1;
        }
    });

    pixels
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

    let mut points: Vec<Point> = Vec::new();
    attractor
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
        .take(ITERATIONS as usize)
        .for_each(|point| points.push(point));

    let pixel_slices: Vec<Vec<Point>> = points
        .chunks(points.len() / 16)
        .map(|chunk| chunk.to_vec())
        .collect();

    let pixels: Vec<Vec<u64>> = pixel_slices
        .par_iter()
        .map(|points| point_to_vec(points))
        .collect();

    let pixels: Vec<u64> = pixels
        .iter()
        .fold(vec![0; pixels[0].len()], |mut acc, vec| {
            acc.iter_mut().zip(vec.iter()).for_each(|(a, &b)| *a += b);
            acc
        });

    let pixels: Vec<u64> = pixels.iter().map(|x| (*x as f32).log(1.5) as u64).collect();
    let unique_pixels: BTreeSet<u64> = pixels.iter().cloned().collect();

    let pixels: Vec<u64> = pixels
        .iter()
        .map(|x| unique_pixels.iter().position(|y| x == y).unwrap_or(0) as u64)
        .collect();

    let max_pixel = pixels.iter().max().unwrap_or(&0);
    let max_pixel = *pixels.iter().max().expect("no max found");

    let gray_image: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::from_raw(
        IMG_COLUMNS as u32,
        IMG_ROWS as u32,
        pixels
            .iter()
            .map(|x| ((*x as f32 / max_pixel as f32) * u16::MAX as f32) as u64)
            .map(|x| u16::MAX - (std::cmp::min(u16::MAX as u64, x) as u16))
            .collect(),
    )
    .expect("Failed to create image buffer");

    gray_image.save("output.png").expect("Failed to save image");

    let end = Instant::now();
    println!("execution time was {:?}", end - start);
}

