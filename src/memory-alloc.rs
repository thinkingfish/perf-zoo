use clap::Parser;
use std::alloc::{alloc, Layout};
use std::time::Instant;

const NANOS_PER_SEC: f64 = 1_000_000_000.0;

#[derive(Debug, Parser)]
#[clap(
    name = "Perf Zoo: Memory Allocation",
    about = "Explore the cost of memory allocation."
)]
struct Args {
    /// number of allocations to perform
    #[clap(
        short = 'n',
        long = "nalloc",
        name = "NUM_OF_ALLOCATIONS",
        default_value = "10000"
    )]
    nalloc: u64,

    /// allocation size in bytes
    #[clap(
        short = 'p',
        long = "power",
        name = "POWER_OF_2_SIZE",
        default_value = "6"
    )]
    power: u32,
}

fn print_result(nalloc: u64, duration_sec: f64) {
    println!("Execution time: {:.6} seconds", duration_sec);
    println!(
        "Average throughput: {:.1} alloc / second",
        (nalloc as f64) / duration_sec
    );
    println!(
        "ns / alloc: {:.1}",
        duration_sec * NANOS_PER_SEC / (nalloc as f64)
    );
    println!();
}

fn alloc_by_size(nalloc: u64, size: usize) {
    let mut v: Vec<*mut u8> = Vec::with_capacity(nalloc as usize);
    let now;

    // Describe the experiment
    println!(
        "====================================================================\n\
         Repeatedly allocating memory blocks of identical size"
    );
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, size);
        now = Instant::now();
        for _ in 0..nalloc {
            v.push(alloc(layout));
        }
    }
    let duration_ns = now.elapsed().as_nanos() as f64;
    print_result(nalloc, duration_ns / NANOS_PER_SEC);
}

fn main() {
    let args = Args::parse();
    let size = 2_usize.pow(args.power);
    println!(
        "====================================================================\n\
         NALLOC: {}, SIZE: {}, total memory: {}\n",
        args.nalloc, size, args.nalloc as usize * size
    );

    alloc_by_size(args.nalloc, size);
}
