use std::{thread, time};
use std::sync::mpsc;

extern crate num_cpus;

const N: u32 = 2_000_000_000;

fn main() {
    let start = time::Instant::now();
    println!("{}, duration = {}ms", parallel(), start.elapsed().as_millis());
}

fn parallel() -> f64 {
    let chunks_count = num_cpus::get() as u32;
    let chunk_size = N / chunks_count;
    let (tx, rx) = mpsc::channel();
    for chunk in 0..chunks_count {
        let tx = tx.clone();
        let offset = chunk * chunk_size;
        thread::spawn(move || {
            let mut sum = 0.0;
            for k in offset..offset + chunk_size {
                sum += term(k as f64);
            }
            tx.send(sum).unwrap();
        });
    }
    let mut sum = 0.0;
    for _ in 0..chunks_count {
        sum += rx.recv().unwrap()
    }
    sum + 3.0
}

fn term(k: f64) -> f64 {
    4.0 * (-1.0_f64).powf(k) / ((2.0 * k + 2.0) * (2.0 * k + 3.0) * (2.0 * k + 4.0))
}