use std::env;
use std::process;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const MICROS_PER_SEC: u64 = 1_000_000;
const NANOS_PER_MICROS: u64 = 1_000;

fn main() {
    let args: Vec<String> = env::args().collect();

    // decide how many threads to create/spawn
    let nthread = match (&args[1]).parse::<u32>() {
        Ok(nthread) => nthread,
        Err(e) => {
            eprintln!("argument parsing error: {:?}", e);
            process::exit(1);
        }
    };

    // decide how many increments to perform in total
    let sum = match (&args[2]).parse::<u64>() {
        Ok(sum) => sum,
        Err(e) => {
            eprintln!("argument parsing error: {:?}", e);
            process::exit(2);
        }
    };

    let iteration: u64 = match sum.checked_div(nthread.into()) {
        Some(iteration) => iteration,
        None => {
            eprintln!("nthread cannot be zero");
            process::exit(1);
        }
    };

    // Spawn n threads to increment a shared variable (non-atomically), and
    // let the main thread know once all increments are done.
    //
    // Here we're using an Arc to share memory among threads, and the data inside
    // the Arc is protected with a mutex.
    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = channel();
    let now = Instant::now();
    for _ in 0..nthread {
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            // The shared state can only be accessed once the lock is held.
            // Our non-atomic increment is safe because we're the only thread
            // which can access the shared state when the lock is held.
            //
            // We unwrap() the return value to assert that we are not expecting
            // threads to ever fail while holding the lock.
            for _ in 0..iteration {
                let mut value = data.lock().unwrap();
                *value += 1;
                // the lock is unlocked here when `value` goes out of scope.
            }

            tx.send(()).unwrap();
        });
    }

    rx.recv().unwrap();

    let duration_us = now.elapsed().as_micros();
    println!("Execution time: {} microseconds", duration_us);
    println!(
        "Average throughput: {:.1} ops / second",
        (sum * MICROS_PER_SEC) as f64 / (duration_us as f64)
    );
    println!(
        "ns / op: {:.1}",
        (duration_us * NANOS_PER_MICROS as u128) as f64 / (sum as f64)
    );
}
