# 🗃️ Optional native DuckDB scratchpad

**Status:** experimental, single-process, single-stdio-client feature. This is not a multi-user database service, approved production analytics export, or proof of full SednaLabs parity.

## Build and start (EndeavourOS / Arch)

The source installation in [INSTALL.md](INSTALL.md) builds the default server **without** scratchpad. With the pinned Rust 1.99.1 toolchain and an available native C/C++ compiler and linker:

```fish
cargo +1.99.1 build --locked --features scratchpad --bin synmetra
target/debug/synmetra serve --scratchpad
```

The `--scratchpad` argument is rejected when the binary was built without the `scratchpad` feature. The server deliberately **rejects** `--scratchpad --http-loopback`: the scratchpad state is shared within a process and not partitioned by authenticated HTTP caller. Do not expose it through a proxy, shared daemon, or public port. The default Google API tools remain read-only; local scratchpad ingestion is an independent local write capability.

## Native dependency and portability boundary

`Cargo.toml` selects optional `duckdb = 1.4.2` with `default-features = false` and `bundled`. Rust `duckdb` bindings build/link the DuckDB engine's **native C/C++** implementation. Bundling is not equivalent to a pure-Rust executable. The default build has no DuckDB dependency; the feature-enabled build needs an appropriate C/C++ toolchain and native linker. A bundled engine does not guarantee that all system C/C++ runtime libraries are absent. Inspect native linkage using `ldd target/debug/synmetra` and `cargo tree --locked --features scratchpad` for a specific Linux distribution/architecture. Do not promise a universally portable binary without testing it.

Synmetra remains MIT licensed; DuckDB and transitive native/build dependencies retain their own license obligations. Review the pinned lockfile and dependency license inventory before redistributing feature-enabled release binaries (T013, T177, T216). The x86-64 Linux feature CI exercises bundled native compilation and tests, but a clean EndeavourOS install, ARM64 build, independent reproducibility, SBOM and signed release are separate unaccepted gates (M9).

## Implemented local safety controls

- Each 1–128-character validated session ID maps to its own **in-memory** DuckDB connection. Sessions expire after 30 minutes; default limit four concurrent sessions (hard maximum eight) and eight tables per session (hard maximum sixteen).
- Initialization disables DuckDB external access and extension auto-install/auto-load, locks configuration, sets a 256 MB engine memory limit and a 64 MB maximum temporary directory size. These are engine configuration limits, not OS-enforced sandboxing or a complete process RSS guarantee.
- Ingestion enforces at most 20,000 rows across a session and transactional rejection of overflowing appends. API responses and local table/query inputs are bounded.
- The user-provided SQL string is **not directly executed**. Only a small `SELECT columns FROM table [ORDER BY column] [LIMIT n] [OFFSET n]` grammar is parsed; the code reconstructs its own identifier-validated query. No joins, file readers, arbitrary expressions, extension commands, DML or DDL are exposed.
- Query output allows at most 1,000 rows and 2,048 characters per cell, and enforces a 256 KiB serialized response ceiling. Evidence export permits at most eight tables, twenty rows per table and 2,048 bytes per sample row, with a 256 KiB aggregate ceiling. Sample order and query hashes are deterministic.
- Evidence export returns bounded JSON plus a Markdown **string** in the MCP response; it does not ask the server to write an arbitrary output path. Downstream clients are responsible for protecting files they create from that response.

Tests in `src/scratchpad.rs` cover session quotas, overflow rollback, deterministic sampling, aggregate output bounds, hostile SQL and direct DuckDB external-access denial. The native feature test must actually pass on a given commit before its security acceptance gates are credited.

## Unaccepted boundaries and operator precautions

- Per-identity isolation across multiple MCP clients (T143) is **not** implemented. IDs are not secrets. Scratchpad-over-HTTP is intentionally rejected, and stdio should be launched under one trusted OS user/client.
- Session data are in memory, but spill/temp cleanup after an unexpected process kill is not independently accepted (T146). DuckDB's 64 MB temp ceiling permits potential temporary storage; do not assume zero disk footprint. Avoid sensitive sessions until cleanup and private-directory behavior are verified.
- Exported analytics can contain sensitive property data even when keys/tokens are excluded. Consumers control retention, local file modes and onward disclosure; export permission and redaction acceptance remains open (T148).
- A synthetic CI suite cannot prove complete SQL sandboxing against every DuckDB engine vulnerability, extension, network path or future dependency revision. Keep the embedded engine patched; additional fuzzing and hostile-path QA are tracked by T149.
- No authorized live Google Analytics ingestion or clean native release/install has been certified. Do not claim full upstream behavioral parity from fixture tests.

See [SECURITY.md](SECURITY.md), [LOCAL_EXECUTION_THREAT_MODEL.md](LOCAL_EXECUTION_THREAT_MODEL.md), and the [acceptance ledger](progress.json).
