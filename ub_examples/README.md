# UB Examples

A collection of Rust examples demonstrating undefined behavior (UB) in various scenarios. These examples are designed for educational purposes to explore unsafe code practices and their potential pitfalls.

## Workspace Overview

This package contains multiple examples located in the `examples/` folder. 
Each example can be run independently to observe specific types of undefined behavior.

## Running Examples

To run an example, use:

```bash
cargo run -p ub_examples --example <example_name>
```

Replace `<example_name>` with the name of the `.rs` file in the `examples/` folder (without the `.rs` extension), e.g.:

```bash
cargo run -p ub_examples --example dangling_pointer
```

To build all examples without running them:

```bash
cargo build -p ub_examples --examples
```

## Examples Available

- `null_pointer`: Demonstrates dereferencing a null pointer.
- `dangling_pointer`: Shows usage of a dangling pointer.
- `type_invariant`: Highlights type invariant violations.
- `uninitialized_memory`: Explores reading uninitialized memory.

---

> **Note:** These examples intentionally contain unsafe code and are not suitable for use in production environments. Handle with care and use for learning purposes only.