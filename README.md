# Dzong

[![Crates.io](https://img.shields.io/crates/v/dzong.svg)](https://crates.io/crates/dzong)
[![Documentation](https://docs.rs/dzong/badge.svg)](https://docs.rs/dzong)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/dzong-db/dzong)

**Dzong** is a high-performance, embedded key-value database engine written in pure Rust, based on a Log-Structured Merge Tree (LSM-Tree) architecture.

Named after the massive fortified monasteries of Bhutan—centers of administration, storage, and protection—Dzong is designed to be a resilient and efficient guardian for your data.

## Features

- **LSM-Tree Architecture**: Optimized for high write throughput and efficient storage.
- **WAL-based Recovery**: Ensuring data integrity across crashes.
- **Concurrent MemTable**: High-concurrency writes using a lock-free or low-lock skip list.
- **Immutable SSTables**: Structured for efficient reads and background compaction.
- **Level-based Compaction**: Automatic background data organization.
- **Zero-copy Orientation**: Minimizing overhead for high-performance systems.

## Engineering Standards

Dzong is built with a focus on:
- **Predictability**: Boring code, explicit ownership, and deterministic behavior.
- **Observability**: First-class support for tracing and metrics.
- **Correctness**: Zero panics policy, strict error handling, and high test coverage.

## Getting Started

Add Dzong to your `Cargo.toml`:

```toml
[dependencies]
dzong = "0.1"
```

## Documentation

- [Architecture](ARCHITECTURE.md)
- [Style Guide](STYLEGUIDE.md)
- [Contributing](CONTRIBUTING.md)

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
