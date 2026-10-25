# 🗃️ Optional DuckDB scratchpad, first vertical slice

Synmetra now contains an **experimental feature-gated native DuckDB** in-memory session manager. DuckDB is compiled from bundled native code through Rust bindings; the MCP executable does not use Python or npm.

Compile with `cargo build --release --features scratchpad`, then opt in using `synmetra serve --scratchpad`. A default read-only Synmetra process does **not** expose scratchpad tools, even when compiled with this feature.

**Ten provisional MCP tools:** `scratchpad_open_session`, `scratchpad_close_session`, `scratchpad_list_sessions`, `scratchpad_get_runtime_limits`, `scratchpad_set_runtime_limits`, `scratchpad_list_tables`, `scratchpad_drop_table`, `scratchpad_describe_table`, `scratchpad_summarize_table`, and `scratchpad_export_evidence_bundle`. This slice covers session lifecycle, table enumeration, safe deletion, and bounded read-only schema/row-count inspection. The other five scratchpad tools are not yet implemented: report/realtime ingestion, SQL query, release regression, and landing shift.

Sessions use caller-specified logical IDs (1–128 ASCII letters, digits, underscores or hyphens), which can be reopened to refresh their 30-minute expiry. They default to a quota of four concurrent sessions, with a hard cap of eight. These IDs are not authentication secrets; for now scratchpad operation is restricted to a single-client stdio process. DuckDB is configured to disallow external file access and automatic extension installation/loading. Drop-table names are constrained to bounded ASCII identifiers to block SQL injection; there is no exposed SQL execution tool yet.

**Security limitations:** this is single-user local state with partial runtime-limit compatibility. Quotas for stored rows and table size still require implementation before ingestion can be enabled. Do not treat this as full SednaLabs scratchpad parity or enable it on publicly reachable HTTP services. No live GA4 data is used in tests.

**Isolation gate:** Synmetra refuses `serve --scratchpad --http-loopback`, even with an inbound HTTP bearer token. The current DuckDB session state is shared within one process; isolated per-client HTTP sessions must be implemented and security-tested before network transports can use it. Stdio remains the only accepted scratchpad transport.

The table-description and summary MCP tools are currently **provisional**, with direct in-memory DuckDB tests. Their pinned upstream schemas also expose pagination, output modes, clipping and summary-only controls; these are not yet implemented, so argument-level or Contract V1 output parity must not be claimed. No arbitrary client-provided SQL is accepted by either tool.

The optional evidence-export tool produces an in-memory JSON+Markdown bundle without writing to disk. Its initial implementation caps exports to eight validated tables, 20 sample rows per table, 2 KiB per sample row, and 256 KiB total serialized output. It does not call Google or support multiuser HTTP. Sample-order determinism and full upstream evidence-contract parity remain open acceptance gates.
