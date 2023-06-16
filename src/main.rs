use crate::attractor_file::{is_between, scale, CliffordAttractor, Point};
use image::{ImageBuffer, Luma};
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

mod attractor_file;

const X_MIN: f64 = -2.5;
const X_MAX: f64 = 2.5;
const Y_MIN: f64 = -2.5;
const Y_MAX: f64 = 2.5;
const IMG_ROWS: usize = 2000;
const IMG_COLUMNS: usize = 2000;
const ITERATIONS: u64 = 5_000_000_000;
const CHUNK_SIZE: u64 = 5_000_000;

fn points_to_pixels(points: Vec<Point>) -> Vec<u64> {
    let mut pixels: Vec<u64> = vec![0; IMG_ROWS * IMG_COLUMNS];

    for point in points.iter() {
        let scaled_x = scale(X_MIN, X_MAX, 0, (IMG_ROWS - 1) as u32, point.x);
        let scaled_y = scale(Y_MIN, Y_MAX, 0, (IMG_COLUMNS - 1) as u32, point.y);
        if is_between(0, (IMG_ROWS - 1) as u32, scaled_x)
            & is_between(0, (IMG_COLUMNS - 1) as u32, scaled_y)
        {
            let pixel_position = (scaled_x + scaled_y * IMG_COLUMNS as u32) as usize;
            pixels[pixel_position] += 1;
        }
    }

    pixels
}

fn process_pixels(pixels: Vec<u64>) -> Vec<u16> {
    let pixels: Vec<u64> = pixels.iter().map(|x| (*x as f32).log(1.5) as u64).collect();
    let unique_pixels: BTreeSet<u64> = pixels.iter().cloned().collect();

    let pixels: Vec<u64> = pixels
        .iter()
        .map(|x| unique_pixels.iter().position(|y| x == y).unwrap_or(0) as u64)
        .collect();

    let max_pixel = pixels.iter().max().unwrap_or(&0);

    let pixels = pixels
        .iter()
        .map(|x| ((*x as f32 / *max_pixel as f32) * u16::MAX as f32) as u64)
        .map(|x| u16::MAX - (std::cmp::min(u16::MAX as u64, x) as u16))
        .collect();

    pixels
}

fn fill_arc(points_vecs: Arc<Mutex<Vec<Vec<Point>>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let attractor = &mut CliffordAttractor {
            point: Point { x: 1.0, y: 1.0 },
            a: -1.7,
            b: 1.3,
            c: -0.1,
            d: -1.2,
        };

        for _ in 0..(ITERATIONS / CHUNK_SIZE) {
            let sub_points: Vec<Point> = attractor.take(CHUNK_SIZE as usize).collect();
            loop {
                {
                    let mut locked_points_vec = points_vecs.lock().unwrap();
                    if locked_points_vec.len() < 10 {
                        let length = sub_points.len();
                        locked_points_vec.push(sub_points);
                        println!(
                            "I just pushed a list with length {}, Arc len is {}",
                            length,
                            locked_points_vec.len()
                        );
                        break;
                    }
                }
                thread::sleep(Duration::from_millis(100));
            }
        }

        println!("finished point generation");
    })
}

fn pop_points(points_vec: Arc<Mutex<Vec<Vec<Point>>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..10000000 {
            let sublist = points_vec.lock().unwrap().pop();
            match sublist {
                Some(sublist) => {
                    println!(
                        "i just poped a list with length {}, Arc len is {}",
                        sublist.len(),
                        points_vec.lock().unwrap().len()
                    );
                    thread::sleep(Duration::from_millis(1000));
                }
                None => thread::sleep(Duration::from_millis(1000)),
            }
        }
    })
}

fn main() {
    let start = Instant::now();

    let points_vecs: Arc<Mutex<Vec<Vec<Point>>>> = Arc::new(Mutex::new(Vec::new()));

    let mut thread_handles_aq: Vec<thread::JoinHandle<()>> = Vec::new();

    thread_handles_aq.push(fill_arc(points_vecs.clone()));
    thread::sleep(Duration::from_secs(1));
    for _ in 0..10 {
        thread_handles_aq.push(pop_points(points_vecs.clone()));
    }

    thread_handles_aq
        .into_iter()
        .for_each(|th| th.join().expect("can't join thread"));
    /*
    let points = points_vecs.lock().unwrap().pop();

    let pixels = points_to_pixels(points.unwrap());
    let pixels = process_pixels(pixels);

    let gray_image: ImageBuffer<Luma<u16>, Vec<u16>> =
        ImageBuffer::from_raw(IMG_COLUMNS as u32, IMG_ROWS as u32, pixels)
            .expect("Failed to create image buffer");

    gray_image.save("output.png").expect("Failed to save image");
    */
    let end = Instant::now();
    println!("execution time was {:?}", end - start);
}
