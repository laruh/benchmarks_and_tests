# Benchmarks and Tests

This project benchmarks two different approaches for extending a vector in Rust and calculates their SHA-256 hashes to ensure they produce the same output.

## Building and Running Manual Benchmark from `main` function

To build and run the manual benchmark from `main.rs`, follow these steps:

1. **Build the Project in Release Mode**:
    ```bash
    cargo build --release
    ```
2. **Run the Benchmarks:**
    ```bash
   ./target/release/benchmarks_and_tests
   ```
   
## Running Criterion Benchmarks
To run the benchmarks using Criterion, follow these steps:

1. **Run the Benchmarks:**
    ```bash
    cargo bench
    ```

This will execute the benchmarks and provide detailed timing analysis for both approaches.

## Functions Being Benchmarked
- extend_from_slice: Extends a vector using the `extend_from_slice` method.
- extend_with_chain: Extends a vector using chained iterators.

## SHA-256 Verification
Both approaches are verified to produce the same output by calculating and comparing their SHA-256 hashes.