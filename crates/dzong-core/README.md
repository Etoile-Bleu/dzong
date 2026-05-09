# dzong-core

The central coordination layer and primary API of the Dzong engine.

## Responsibilities
- **Orchestration**: Manages the interaction between MemTable, WAL, SSTables, and Manifest.
- **MemTable Management**: Handles active in-memory writes and schedules flushes.
- **Read Path**: Executes lookups across memory and disk hierarchy.
- **Recovery**: Replays WAL and Manifest to restore state after a restart.

## Main Components
- **`DzongEngine`**: The main entry point for `put`, `get`, and `delete` operations.
- **`Options`**: Configurable parameters for the engine (data directory, memtable size, etc.).

## Lifecycle
1. **Write**: Appended to WAL, then inserted into MemTable.
2. **Flush**: When MemTable is full, it's converted to a Level 0 SSTable.
3. **Recovery**: Replays Manifest to find SSTables, then replays WAL for non-flushed data.
