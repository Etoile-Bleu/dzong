# dzong-manifest

Persistent metadata registry and version management.

## Responsibilities
- **State Tracking**: Maintains the list of all active SSTables and their levels.
- **Atomic Commits**: Records database state changes via append-only `VersionEdit` logs.
- **Consistency**: Ensures the engine always starts with a valid view of the data.

## Core Concepts
- **VersionSet**: The orchestrator of state transitions.
- **Version**: An immutable snapshot of the database files.
- **VersionEdit**: A delta (Add/Remove file) applied to transition between versions.

## Durability
The manifest is persisted to a `MANIFEST` file. Upon recovery, all edits are replayed to reconstruct the latest `Version`.
