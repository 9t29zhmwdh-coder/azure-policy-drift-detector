# Contributing

Contributions are welcome. Please follow these guidelines:

## Before submitting a PR

- `cargo clippy --workspace -- -D warnings` must pass
- `cargo test --workspace` must pass
- No credentials, subscription IDs, or resource data in any committed file
- New Azure API calls must be read-only

## Commit style

Use prefix format: `[feat]`, `[fix]`, `[docs]`, `[refactor]`, `[test]`

Example: `[feat] Add Management Group scope support`

## Adding a new drift detection type

1. Add the new `DriftType` variant in `apdd-core/src/models.rs`
2. Implement detection logic in `apdd-core/src/analyzer.rs` with unit tests
3. Add the Azure API call in `apdd-azure/src/` if needed
4. Wire the result into `apdd-core/src/report.rs`
5. Update `ROADMAP.md` and `CHANGELOG.md`
