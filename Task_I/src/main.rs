use std::fs::File;
use std::io::{BufReader, Read};
use std::thread;
use std::time::Instant;

fn serial_sum(array: &[u64]) -> u64 {
    let mut sum = 0;
    for &value in array {
        sum += value;
    }
    sum
}

fn parallel_sum(array: &[u64], interleaved: bool) -> u64 {
    thread::scope(|scope| {
        let mut workers = Vec::new();

        for i in 0..4 {
            workers.push(scope.spawn(move || {
                let mut sum = 0;

                if interleaved {
                    for j in (i..array.len()).step_by(4) {
                        sum += array[j];
                    }
                } else {
                    let start = i * array.len() / 4;
                    let end = (i + 1) * array.len() / 4;
                    for j in start..end {
                        sum += array[j];
                    }
                }

                sum
            }));
        }

        let mut total = 0;
        for worker in workers {
            total += worker.join().unwrap();
        }
        total
    })
}

fn main() {
    let n: usize = 1_000_000; // Change this to test other sizes (N >= 1024).
    assert!(n >= 1024 && (n as u128) * 99 <= u64::MAX as u128);
    let mut array: Vec<u64> = Vec::new();
    let mut random = BufReader::new(File::open("/dev/urandom").unwrap());

    for _ in 0..n {
        let mut bytes = [0_u8; 8];
        random.read_exact(&mut bytes).unwrap();
        array.push(u64::from_ne_bytes(bytes) % 100);
    }

    let start = Instant::now();
    let serial = serial_sum(&array);
    let serial_time = start.elapsed();

    let start = Instant::now();
    let interleaved = parallel_sum(&array, true);
    let interleaved_time = start.elapsed();

    let start = Instant::now();
    let contiguous = parallel_sum(&array, false);
    let contiguous_time = start.elapsed();

    assert_eq!(serial, interleaved);
    assert_eq!(serial, contiguous);

    println!("N = {n}; all three sums match.");
    println!("Serial:      sum = {serial}, time = {serial_time:?}");
    println!("Interleaved: sum = {interleaved}, time = {interleaved_time:?}");
    println!("Contiguous:  sum = {contiguous}, time = {contiguous_time:?}");
}
