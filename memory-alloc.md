# Compile and Run

To compile:
```
cargo build --debug --bin mutex-contention
```
or run directly with:
```
cargo run --debug  --bin memory-alloc
```

# Controls

You can control how many allocations to perform, and how much memory to allocate
each time, with commandline options `-n` and `-p`, respectively.

Run `target/debug/memory-alloc -h` for a full help message.

# Source

The source code is in `src/memory-alloc.rs`

# Note

Try invoking the program a few times with the same arguments, and observe the
variance in results. Memory allocation is notoriously unpredictable, and
depending on the total memory pressure of the system, as well as the allocator
used, it is not unusual to see 10x+ difference in how long it takes to allocate
memory of the same size. This is why memory allocation can be an important
contributor to high latency at tail.
