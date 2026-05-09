# Architecture Decision Records & Design

This document outlines the architectural principles and design of Dzong.

## Core Components

### 1. MemTable (`dzong-core`)
The "Great Hall" where all writes first arrive. It uses a concurrent SkipList to provide sorted access and high-concurrency inserts.

### 2. Write-Ahead Log (WAL) (`dzong-wal`)
The "Inscription": every operation is recorded sequentially for crash recovery before being acknowledged.

### 3. SSTables (`dzong-sstable`)
The "Vaults": immutable files stored on disk in levels. Each table includes:
- **Data Blocks**: Key-value pairs stored in sorted order.
- **Index Blocks**: Binary search index for block locations.
- **Bloom Filters**: Fast membership testing to skip unnecessary disk I/O.

### 4. Compaction (`dzong-core`)
The "Librarians": background workers that merge SSTables from higher levels to lower levels, reclaiming space and maintaining read performance.

### 5. Manifest (`dzong-core`)
The "Registry": maintains the current state of the database, including which SSTables belong to which levels.

## Data Flow

### Write Path
1. Write arrives at the Engine.
2. Appended to the current WAL.
3. Inserted into the active MemTable.
4. If MemTable exceeds size limit, it becomes immutable and is flushed to Level 0 as an SSTable.

### Read Path
1. Search the active MemTable.
2. Search immutable MemTables (in-flight flushes).
3. Check Level 0 SSTables (ordered by time).
4. Search Level 1 and below (using index and Bloom filters).

## Concurrency Model
- Shared-nothing where possible.
- Explicit synchronization using `parking_lot` and `crossbeam`.
- Background workers handle I/O-intensive tasks (flushing, compaction).
