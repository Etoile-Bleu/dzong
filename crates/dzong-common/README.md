# dzong-common

Foundational primitives, shared types, and error handling for the Dzong database engine.

## Responsibilities
- **Error Handling**: Defines `DzongError` and `Result<T>` used across all crates.
- **Shared Types**: Core abstractions like `Key` and `Value`.
- **Traits**: Common interfaces for components.
- **Utilities**: Byte manipulation and generic helpers.

## Design
This crate aims for zero dependencies on other `dzong-*` crates to serve as the base of the dependency graph.
