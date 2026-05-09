# Dzong

[![Crates.io](https://img.shields.io/crates/v/dzong.svg)](https://crates.io/crates/dzong)
[![Documentation](https://docs.rs/dzong/badge.svg)](https://docs.rs/dzong)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/dzong-db/dzong)

**Dzong** is a high-performance, embedded key-value database engine written in pure Rust, based on a Log-Structured Merge Tree (LSM-Tree) architecture.

Named after the massive fortified monasteries of Bhutan—centers of administration, storage, and protection—Dzong is designed to be a resilient and efficient guardian for your data.

> [!IMPORTANT]
> Dzong is currently in **v0.1.x (Stable Beta)**. The core LSM engine (WAL, MemTable, SSTables, Manifest, Compaction) is functional and verified by a comprehensive integration test suite.

## Features

- **LSM-Tree Architecture**: Optimized for high write throughput and efficient storage.
- **WAL-based Recovery**: Ensuring data integrity across crashes with sequential durability.
- **Atomic Manifest**: Persistent metadata registry tracking database state via `VersionEdit` logs.
* **Block-based SSTables**: High-performance immutable storage with sparse indexing.
- **Level-based Compaction**: Automatic background data organization (L0 -> L1+).
- **Tombstone Semantics**: First-class support for deletes via explicit operation types.
- **Pure Rust**: Zero unsafe code in the core engine unless strictly necessary and documented.

## Usage

Add Dzong to your `Cargo.toml`:

```toml
[dependencies]
dzong = "0.1"
```

### Basic Example

```rust
use dzong::{DzongEngine, Options, Key, Value};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Configure the engine
    let path = Path::new("./data");
    let options = Options::new(path);
    
    // 2. Open the engine (recovers state from WAL/Manifest if exists)
    let mut engine = DzongEngine::open(options)?;
    
    // 3. Put/Get/Delete operations
    let key = Key::new(b"user:123");
    let val = Value::new(b"Matheo");
    
    engine.put(key.clone(), val.clone())?;
    
    if let Some(record) = engine.get(&key)? {
        println!("Found: {:?}", record);
    }
    
    engine.delete(key)?;
    
    Ok(())
}
```

## Documentation

- [Architecture](ARCHITECTURE.md) - Deep dive into internals.
- [Style Guide](STYLEGUIDE.md) - Coding standards.
- [Contributing](CONTRIBUTING.md) - How to help.

## Testing

Dzong maintains a high bar for correctness:

```bash
# Run unit tests
cargo test --workspace

# Run integration tests (aggressive workloads & recovery)
cargo test -p dzong-testing
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
