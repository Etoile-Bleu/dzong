# Dzong Architecture

Dzong is a pure Rust embedded storage engine based on a Log-Structured Merge Tree (LSM-Tree) architecture.

The project is designed around:

* deterministic behavior
* explicit ownership
* modularity
* observability
* low read amplification
* predictable write performance

The name "Dzong" references the fortified monasteries of Bhutan historically used as administrative and storage centers.

The lore remains purely aesthetic and must never influence engineering decisions.

---

# Design Principles

Dzong follows several core engineering principles:

* correctness before optimization
* explicit over implicit
* immutable storage structures
* sequential disk I/O whenever possible
* small composable modules
* zero-copy oriented APIs where practical
* strict ownership semantics
* no hidden synchronization

The architecture intentionally favors maintainability and debuggability over premature complexity.

---

# Workspace Layout

Dzong uses Cargo Workspaces to isolate responsibilities and reduce coupling.

```text
crates/
├── dzong-core/
├── dzong-cli/
├── dzong-wal/
├── dzong-sstable/
├── dzong-bloom/
├── dzong-manifest/
├── dzong-common/
└── dzong-testing/
```

## Crate Responsibilities

### dzong-core

High-level database engine:

* MemTable orchestration
* read/write coordination
* flush scheduling
* compaction management
* snapshots
* iterators

### dzong-cli

Administrative tooling and debugging interface.

### dzong-wal

Crash recovery and sequential durability layer.

### dzong-sstable

Immutable sorted table implementation.

### dzong-bloom

Bloom filter implementation and membership checks.

### dzong-manifest

Persistent metadata registry describing database state.

### dzong-common

Shared primitives:

* errors
* traits
* utilities
* shared types

### dzong-testing

Reusable testing and benchmarking infrastructure.

---

# Core Architectural Components

## 1. MemTable

The MemTable is the active in-memory write buffer.

Implementation goals:

* sorted key ordering
* concurrent inserts
* predictable iteration
* low allocation overhead

The primary implementation uses a concurrent SkipList.

Responsibilities:

* absorb incoming writes
* maintain sorted ordering
* provide fast point lookups
* expose sequential iterators

### MemTable Invariants

* keys remain sorted
* active MemTable is mutable
* immutable MemTables are read-only
* immutable MemTables are flushed exactly once

---

## 2. Write-Ahead Log (WAL)

Every write operation must be persisted to the WAL before acknowledgement.

The WAL provides:

* crash recovery
* durability guarantees
* sequential append performance

### WAL Guarantees

* append-only writes
* ordered durability
* checksum validation
* corruption detection

### WAL Invariants

* fsync completes before acknowledgement
* records are replayable in insertion order
* partial writes are detectable

---

# WAL Record Layout

```text
+-----------+------------+-------------+---------+
| Checksum  | Key Length | Value Length| Payload |
+-----------+------------+-------------+---------+
```

Payload:

```text
[key bytes][value bytes]
```

Future versions may support:

* compression
* batching
* record fragmentation

---

## 3. SSTables

SSTables are immutable sorted files stored on disk.

Once created, SSTables are never modified.

This property simplifies:

* concurrency
* caching
* iteration
* crash recovery

---

# SSTable Layout

```text
+-------------------+
| Data Blocks       |
+-------------------+
| Block Index       |
+-------------------+
| Bloom Filter      |
+-------------------+
| Metadata Block    |
+-------------------+
| Footer            |
+-------------------+
```

---

## Data Blocks

Data blocks contain:

* sorted key-value pairs
* restart points
* optional compression

Properties:

* sequentially readable
* binary searchable
* cache-friendly

---

## Index Block

The index maps:

* key ranges
* block offsets
* block sizes

This enables logarithmic block lookup.

---

## Bloom Filters

Bloom filters reduce unnecessary disk reads.

Properties:

* probabilistic membership checks
* configurable false-positive rate
* no false negatives

Bloom filters are checked before disk access.

---

## Footer

The footer contains:

* metadata offsets
* format version
* magic number
* integrity validation

---

# Storage Invariants

The storage engine guarantees:

* SSTables are immutable
* keys inside SSTables remain sorted
* Levels >= 1 contain non-overlapping key ranges
* WAL durability precedes MemTable acknowledgement
* Manifest updates are atomic

Violating these invariants is considered corruption.

---

# Write Path

The write path is optimized for:

* sequential I/O
* batching
* low latency

---

## Write Flow

```text
Client Write
    ↓
WAL Append
    ↓
fsync
    ↓
MemTable Insert
    ↓
Acknowledgement
```

---

## Flush Flow

When the active MemTable exceeds its configured threshold:

```text
Active MemTable
    ↓
Immutable MemTable
    ↓
Background Flush
    ↓
Level 0 SSTable
```

Flushes occur asynchronously.

---

# Read Path

Reads prioritize:

* memory access
* newest data
* minimal disk I/O

---

## Read Flow

```text
Active MemTable
    ↓
Immutable MemTables
    ↓
Level 0 SSTables
    ↓
Level 1+
```

---

## Lookup Strategy

### Active MemTable

Fastest lookup path.

### Immutable MemTables

Recent writes waiting for flush completion.

### Level 0 SSTables

May contain overlapping key ranges.

Searched newest-first.

### Levels >= 1

Properties:

* sorted ranges
* non-overlapping files
* binary-searchable

Bloom filters are checked before disk access.

---

# Compaction

Compaction reorganizes SSTables to:

* reduce read amplification
* reclaim storage
* remove obsolete keys
* collapse tombstones

---

# Compaction Strategy

Dzong uses leveled compaction inspired by RocksDB.

### Level 0

Allows overlapping ranges.

Optimized for:

* fast flushes
* high write throughput

### Levels >= 1

Maintain non-overlapping sorted ranges.

Optimized for:

* predictable reads
* binary search efficiency

---

# Compaction Invariants

Compaction must preserve:

* newest sequence numbers
* sorted ordering
* snapshot visibility
* atomic manifest updates

---

# Manifest

The Manifest is the persistent metadata registry.

It tracks:

* SSTable locations
* level assignments
* compaction state
* sequence numbers

The Manifest enables:

* startup recovery
* consistent database reconstruction

---

# Manifest Guarantees

* atomic updates
* monotonic sequence progression
* crash-safe recovery

---

# Concurrency Model

Dzong uses explicit synchronization.

Preferred primitives:

* Arc
* atomics
* crossbeam
* parking_lot

---

# Concurrency Guarantees

* SSTables are immutable and thread-safe
* reads should remain lock-free whenever practical
* WAL appends are serialized
* MemTable writes are concurrent
* background tasks are isolated

---

# Background Workers

Dedicated workers handle:

* MemTable flushes
* compaction
* cleanup
* manifest persistence

Workers communicate through bounded channels.

---

# Memory Ownership Model

Ownership rules remain explicit.

### SSTables

Shared using Arc.

### MemTables

Single mutable owner during active writes.

### Iterators

Borrow immutable views whenever possible.

### Buffers

Reused aggressively to reduce allocations.

---

# Error Handling

Production code must never:

* panic
* unwrap
* expect

All recoverable failures use:

* Result<T, E>
* explicit propagation
* structured error types

Corruption must never crash the process.

---

# Observability

Dzong uses structured tracing.

Critical operations emit spans for:

* WAL latency
* flush duration
* compaction duration
* read amplification
* SSTable generation

The observability stack uses:

* tracing
* tracing-subscriber

---

# Testing Philosophy

Testing is mandatory.

Every subsystem requires:

* unit tests
* integration tests
* corruption tests
* recovery tests
* concurrency tests

Future roadmap:

* property testing
* fuzzing
* crash simulation
* fault injection

---

# Future Extensions

Planned future features include:

* block cache
* snapshots
* prefix iterators
* compression
* transactional batches
* async APIs
* pluggable compaction policies
* adaptive Bloom filters

The architecture must remain extensible without major rewrites.
