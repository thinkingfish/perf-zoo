# Compile and Run

To compile:
```
cargo build --release --bin mutex-contention
```
or run directly with:
```
cargo run --release --bin mutex-contention
```

# Controls

You can control how many increments to perform, and how many threads to deploy,
with commandline options `-s` and `-n`, respectively.

Run `target/release/mutex-contention -h` for a full help message.

# Source

The source code is in `src/mutex-contention.rs`
