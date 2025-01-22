# Benchmark extend_from_slice vs extend_with_chain

This project benchmarks two different approaches for extending a vector in Rust and calculates their SHA-256 hashes to ensure they produce the same output.

## Functions Being Benchmarked
- **extend_from_slice**: Extends a vector using the `extend_from_slice` method.
- **extend_with_chain**: Extends a vector using chained iterators.

## SHA-256 Verification
Both approaches are verified to produce the same output by calculating and comparing their SHA-256 hashes.

## Building and Running Manual Benchmark from `main` Function

To build and run the manual benchmark from `main.rs`, follow these steps:

1. **Build the Project in Release Mode**:
    ```bash
    cargo build -p slice_vs_iter --release
    ```
   This command builds the `slice_vs_iter` package in release mode, optimizing the code for performance.

2. **Run the Manual Benchmark**:
    ```bash
    ./target/release/slice_vs_iter
    ```
   This runs the built executable for `slice_vs_iter`, which contains the manual benchmarking code.

## Running Criterion Benchmarks
To run the benchmarks using Criterion, you can run all benchmarks in the `slice_vs_iter` package:

   ```bash
    cargo bench -p slice_vs_iter
   ```
    