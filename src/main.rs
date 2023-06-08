use crate::attractor_file::{is_between, scale, CliffordAttractor, Point};
use image::{ImageBuffer, Luma};
use rayon::prelude::*;
use std::collections::BTreeSet;
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

