# dzong-compaction

Background data reorganization and space reclamation.

## Responsibilities
- **Read Optimization**: Merges multiple SSTables into fewer, non-overlapping files.
- **Garbage Collection**: Physically removes keys marked with tombstones (`Delete`).
- **LSN Resolution**: Resolves conflicts by picking the record with the highest LSN.

## Components
- **`CompactionPicker`**: Decides when and which files to compact based on level thresholds.
- **`MergeIterator`**: An N-way merge iterator that resolves duplicates across multiple inputs.
- **`CompactionWorker`**: Executes the physical merge and produces new SSTables.

## Strategy
Currently implements a basic Leveled Compaction strategy (L0 -> L1).
