use std::process;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use structopt::StructOpt;

const NANOS_PER_SEC: f64 = 1_000_000_000.0;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Perf Zoo: Mutext Contention",
    about = "Explore the scalability of mutex in a multicore environment.")]
struct Args {
    /// number of concurrent threads
    #[structopt(short = "n", long = "nthread", name = "NUM_OF_THREADS", default_value = "1")]
    nthread: u32,

    /// total increments
    #[structopt(short = "s", long = "sum", name = "SUM_OF_INCREMENTS", default_value = "100000000")]
    sum: u64,
}

fn print_result(sum: u64, duration_sec: f64) {
    println!("Execution time: {:.6} seconds", duration_sec);
    println!(
        "Average throughput: {:.1} ops / second",
        (sum as f64) / duration_sec
    );
    println!(
        "ns / op: {:.1}",
        duration_sec * NANOS_PER_SEC / (sum as f64)
    );
    println!();
}

fn incr_raw(sum: u64) {
    let mut value: u64 = 0;
    let now = Instant::now();

    // Describe the experiment
    println!(
        "====================================================================\n\
          Incrementing the counter without locking, added extra bit operation \
          and addition, so the compiler won't eliminate the loop entirely"
    );

    for _ in 0..sum {
        // use bit operation so compiler won't eliminate the loop entirely
        // use value inside the loop so compiler won't vectorize/unroll it.
        value += 1 + (value & 1);
    }

    assert!(value > sum);

    let duration_ns = now.elapsed().as_nanos() as f64;
    print_result(sum, duration_ns / NANOS_PER_SEC);
}

fn incr_mutex(sum: u64, nthread: u32) {
    let iteration: u64 = match sum.checked_div(nthread.into()) {
        Some(iteration) => iteration,
        None => {
            eprintln!("nthread cannot be zero");
            process::exit(1);
        }
    };

    // Describe the experiment
    println!(
        "====================================================================\n\
          Incrementing the counter using {} threads & mutex for synchronization",
        nthread
    );

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

    let duration_ns = now.elapsed().as_nanos() as f64;
    print_result(sum, duration_ns / NANOS_PER_SEC);
}

fn main() {
    let args = Args::from_args();
    println!(
        "====================================================================\n\
          SUM: {}, THREAD COUNT: {}\n", args.sum, args.nthread);

    incr_raw(args.sum);
    incr_mutex(args.sum, args.nthread);
}
