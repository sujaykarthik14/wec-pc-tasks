# Task 1: Parallel Array Sum

This is my Rust solution for the Parallel Computing recruitment task. It generates an array of random `u64` numbers and compares three ways of finding their sum.

## How it works

1. **Serial:** One thread adds all the elements.
2. **Interleaved:** Four worker threads share the work. Thread `i` reads indices `i`, `i + 4`, `i + 8`, and so on.
3. **Contiguous:** Each of the four workers reads one continuous part of the array, from `i * N / 4` up to, but not including, `(i + 1) * N / 4`.

Each worker has its own sum. The main thread waits for all four workers and combines their results. The program checks that all three answers match.

## Why parallel summation is useful

For a large array, dividing the work among CPU cores can make the calculation faster. This idea is useful for calculating totals and averages in large datasets and for combining results in image processing and scientific calculations.

The assignment also shows why the way we divide the work matters. Contiguous access reads nearby elements, which can make better use of the CPU cache. Interleaved access reads every fourth element. Comparing both methods helps show how memory access affects performance.

Using more threads does not always save time. For a small array, starting the threads and waiting for them can cost more time than the calculation itself.

## Purpose of this recruitment task

This task gives me practice with creating threads, dividing work, and combining results safely. It also helps me understand why timing a program is necessary before deciding whether a parallel version is better than a serial one.

## How to run

With `main.rs` in the current folder, run these commands on Linux:

rustc -O --edition=2021 main.rs -o task1
./task1

Change this line in `main.rs` to test different array sizes:

```rust
let n: usize = 1_000_000;
```

N must be at least 1024.

## Timing

The output shows the sum and execution time for each method. Generating the array and printing are not included in the timings. The parallel timings include starting the workers, waiting for them, and combining their sums.

The program measures each method once per run. Running it several times helps account for timing variation.
