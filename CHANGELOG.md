# Changelog

[progress.json](docs/progress.json) is the evidence-backed task ledger. See the [release policy](docs/RELEASE_POLICY.md). This changelog records version-facing changes, not estimated completion.

## [Unreleased]

### Added
- Native Rust MCP stdio and authenticated, loopback-only HTTP transports.
- Provisional Admin and Data API reporting handlers, native OAuth and gated annotation mutation.
- Optional native DuckDB scratchpad and pinned upstream contract fixtures.
- Linux source installation, build smoke and experimental x86-64 release artifact workflow.

### Security
- HTTP body and in-flight limits, timeouts and a 1 MiB bounded stdio frame relay.
- Default read-only MCP surface and redacted write-audit events.

### Outstanding
- Complete 36-tool upstream parity, authorized live Google tests, cross-client conformance, full DuckDB security gates and reproducible/signed distribution.

Additional implementation status, source attribution, alpha eligibility and distribution limits are in the [experimental preview notes](docs/EXPERIMENTAL_RELEASE_NOTES.md). These are not published release notes for a tagged binary.

## [0.0.1] - Development baseline (not a published stable release)

- Initial Rust crate and project identity; no stability guarantees attach to this version string.
