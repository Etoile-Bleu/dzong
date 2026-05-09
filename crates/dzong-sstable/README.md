# dzong-sstable

Immutable Sorted String Table (SSTable) implementation.

## Responsibilities
- **Persistence**: Efficiently stores sorted key-value pairs on disk.
- **Indexing**: Provides sparse indexes for fast logarithmic lookups.
- **Iteration**: Supports sequential scans over key ranges.

## Physical Layout
- **Data Blocks**: Chunks of sorted records (default 4KB).
- **Index Block**: Maps the first key of each block to its file offset.
- **Footer**: Fixed-size tail containing pointers to the index and metadata.

## Tombstone Handling
SSTables explicitly store the operation type (`Put` or `Delete`). A `Delete` record acts as a tombstone, hiding keys in older SSTables during read operations.
