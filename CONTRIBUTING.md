# Contributing to Dzong

Thank you for your interest in contributing to Dzong. We maintain high engineering standards to ensure the project remains production-grade.

## Workflow

1. **Fork and Branch**: Create a feature branch for your changes.
2. **Implement**: Follow the [Style Guide](STYLEGUIDE.md).
3. **Test**: Ensure all tests pass (`cargo test --workspace`).
4. **Format & Lint**:
    - `cargo fmt --all`
    - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
5. **Commit**: Use [Conventional Commits](https://www.conventionalcommits.org/).

## Coding Standards

- **Explicitness**: Prefer explicit code over "clever" abstractions.
- **Observability**: Add tracing spans and events for significant operations.
- **Documentation**: All public APIs must have rustdoc comments and examples.

## Pull Request Requirements

- All CI checks must pass (no warnings, no formatting errors).
- New features must include unit and integration tests.
- Large changes should be preceded by an Architecture Decision Record (ADR) in the `ADRs/` directory.
