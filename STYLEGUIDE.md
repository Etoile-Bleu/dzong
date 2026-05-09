# Engineering Style Guide

All code in Dzong must adhere to these strict engineering standards.

## Code Constraints

### 1. File Size Limits
- **Preferred**: < 100 lines.
- **Hard Limit**: 150 lines.
- If a file exceeds 150 lines, it **must** be split into submodules or refactored into smaller traits/builders.

### 2. Function Size Limits
- **Target**: <= 25 lines.
- **Absolute Max**: 40 lines.
- Focus on single-purpose, composable functions.

### 3. Module Organization
- Each module has a single responsibility.
- `mod.rs` files are reserved **only** for module declarations (`mod x;`) and public re-exports (`pub use x::*;`). **No business logic in `mod.rs`.**

## Rust Idioms

### 1. No Panics
- **Forbidden**: `unwrap()`, `expect()`, `panic!()`, `todo!()`, `unimplemented!()`.
- Use `Result<T, E>` with `thiserror` for all error handling.
- Propagation must be explicit with `?`.

### 2. Unsafe Code
- Forbidden by default.
- Only allowed if a measurable performance bottleneck is proven via benchmarking.
- Must include:
    - Safety invariants documentation.
    - Memory and aliasing guarantees.
    - Associated unit tests.

### 3. Ownership & Allocation
- Minimize `clone()`, `Arc`, and heap allocations.
- Prefer borrowing, slices, and `Cow`.
- Design for zero-copy reads.

## Testing Policy
- Every module requires unit tests, edge case tests, and failure tests.
- Integration tests belong in `tests/`.
- Benchmarks must use `Criterion`.
- Target: High coverage and deterministic results.

## Commits
- Use **Conventional Commits**.
- Commits must be atomic and logically scoped.
