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
   - **`cargo build -p slice_vs_iter --release`**: This command builds the `slice_vs_iter` package in release mode, optimizing the code for performance.

2. **Run the Manual Benchmark**:
    ```bash
    ./target/release/slice_vs_iter
    ```
   - **`./target/release/slice_vs_iter`**: This runs the built executable for `slice_vs_iter`, which contains the manual benchmarking code.

## Running Criterion Benchmarks
To run the benchmarks using Criterion, you can choose between the following options:

1. **Run All Benchmarks in the `slice_vs_iter` Package**:
    ```bash
    cargo bench -p slice_vs_iter
    ```
   - **`cargo bench -p slice_vs_iter`**: This command runs all the benchmarks defined in the `slice_vs_iter` package.

2. **Run a Specific Benchmark File**:
    ```bash
    cargo bench --bench bench_slice_vs_iter
    ```
   - **`cargo bench --bench bench_slice_vs_iter`**: This command runs only the benchmarks defined in the `bench_slice_vs_iter.rs` file.

3. **Run a Specific Benchmark Function**:
    ```bash
    cargo bench --bench bench_slice_vs_iter -- <bench_function_name_filter>
    ```
   - **`cargo bench --bench bench_slice_vs_iter -- <bench_function_name_filter>`**: This command runs a specific benchmark function from the `bench_slice_vs_iter.rs` file. Replace `<bench_function_name_filter>` with a part of the benchmark function name to filter and run only that specific benchmark.

This will execute the benchmarks and provide detailed timing analysis for both approaches.