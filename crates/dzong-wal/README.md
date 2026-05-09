# dzong-wal

Write-Ahead Log (WAL) implementation for crash-safe durability.

## Responsibilities
- **Durability**: Ensures every write is persisted to disk before being acknowledged.
- **Sequential I/O**: Optimized for append-only performance.
- **Recovery**: Provides `WalReader` to replay operations during engine startup.

## Binary Format
Each record follows the structure:
- `Checksum` (u32)
- `OpType` (u8)
- `LSN` (u64)
- `Key Length` (u32)
- `Value Length` (u32)
- `Payload` (Key + Value bytes)

## Invariants
- `fsync` is called on every write (configurable).
- Partial or corrupt records are detected via checksums.
