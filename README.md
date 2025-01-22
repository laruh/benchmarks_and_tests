# Project Cleanliness Commands

Use the following `cargo` commands to keep the project clean and well-maintained:

### Code Formatting

Format the entire codebase with:

```bash
cargo fmt
```

### Linting

Run Clippy with strict settings to catch issues:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

- **`--workspace`**: Lints all workspace crates.
- **`--all-targets`**: Lints library, binary, test, and benchmark targets.
- **`-D warnings`**: Treats all warnings as errors.