# 🗃️ Optional DuckDB scratchpad, first vertical slice

Synmetra now contains an **experimental feature-gated native DuckDB** in-memory session manager. DuckDB is compiled from bundled native code through Rust bindings; the MCP executable does not use Python or npm.

Compile with `cargo build --release --features scratchpad`, then opt in using `synmetra serve --scratchpad`. A default read-only Synmetra process does **not** expose scratchpad tools, even when compiled with this feature.

**Thirteen provisional MCP tools:** `scratchpad_open_session`, `scratchpad_close_session`, `scratchpad_list_sessions`, `scratchpad_get_runtime_limits`, `scratchpad_set_runtime_limits`, `scratchpad_list_tables`, `scratchpad_drop_table`, `scratchpad_describe_table`, `scratchpad_summarize_table`, and `scratchpad_export_evidence_bundle`. This slice covers session lifecycle, table enumeration, safe deletion, and bounded read-only schema/row-count inspection. The two remaining scratchpad tools are release regression diagnostics and landing-parameter shift diagnostics.

Sessions use caller-specified logical IDs (1–128 ASCII letters, digits, underscores or hyphens), which can be reopened to refresh their 30-minute expiry. They default to a quota of four concurrent sessions, with a hard cap of eight. These IDs are not authentication secrets; for now scratchpad operation is restricted to a single-client stdio process. DuckDB is configured to disallow external file access and automatic extension installation/loading. Drop-table names are constrained to bounded ASCII identifiers to block SQL injection; there is no exposed SQL execution tool yet.

**Security limitations:** this is single-user local state with partial runtime-limit compatibility. Quotas for stored rows and table size still require implementation before ingestion can be enabled. Do not treat this as full SednaLabs scratchpad parity or enable it on publicly reachable HTTP services. No live GA4 data is used in tests.

**Isolation gate:** Synmetra refuses `serve --scratchpad --http-loopback`, even with an inbound HTTP bearer token. The current DuckDB session state is shared within one process; isolated per-client HTTP sessions must be implemented and security-tested before network transports can use it. Stdio remains the only accepted scratchpad transport.

The table-description and summary MCP tools are currently **provisional**, with direct in-memory DuckDB tests. Their pinned upstream schemas also expose pagination, output modes, clipping and summary-only controls; these are not yet implemented, so argument-level or Contract V1 output parity must not be claimed. No arbitrary client-provided SQL is accepted by either tool.

The optional evidence-export tool produces an in-memory JSON+Markdown bundle without writing to disk. Its initial implementation caps exports to eight validated tables, 20 sample rows per table, 2 KiB per sample row, and 256 KiB total serialized output. It does not call Google or support multiuser HTTP. Sample cells are currently emitted as nullable **bounded text**, including numeric cells, instead of preserving original SQL scalar types. This prevents reliance on DuckDB's disabled JSON extension. Sample-order determinism, native type fidelity, and full upstream evidence-contract parity remain open acceptance gates.

### 📥 Bounded GA4 response ingestion (experimental)

The optional `scratchpad_ingest_report` and `scratchpad_ingest_realtime_report` tools now call the read-only GA4 Data API and load **up to 1,000 returned rows per request** into a memory-only DuckDB table. They preserve metrics as VARCHAR to avoid silently losing precision. Upstream property authorization and native OAuth still apply. Before a network request, the server checks that the local scratchpad session exists.

Ingestion validates 1–16 distinct safe column names, the Data API dimension/metric headers and row lengths, 2,048-byte individual cells, a 2 MiB response size, and the 20,000-row per-session cap. Database values use bound parameters and writes are transactional. It rejects accidental table replacement; `append=true` is required for a matching existing schema. These are deliberately tighter quotas than upstream and do not yet establish complete pagination, output-format or GA behavior parity. Stdio-only remains the accepted transport; never expose scratchpad over HTTP.

### 🔎 Strict read-only SQL subset

`scratchpad_query` is now provisionally implemented and separately capability-gated. It **does not forward arbitrary client SQL**: a bounded ASCII grammar is parsed and safe, quoted column/table identifiers are reconstructed into new SQL. The current accepted syntax is `SELECT col1, col2 FROM table [ORDER BY col ASC|DESC] [LIMIT n] [OFFSET n]`, `SELECT * FROM table`, or `SELECT COUNT(*) FROM table`. There is no WHERE, JOIN, UNION, CTE, file scanner, external function, DDL, DML, extension, or multiple-statement support; these requests are rejected.

Rows default to 100 (maximum 1,000), cell rendering defaults to 1,024 characters (maximum 2,048), OFFSET cannot exceed 20,000, and serialized output cannot exceed 256 KiB. It supports `summary_only`; non-`rows` output modes and pagination cursors currently return explicit unsupported errors rather than silently changing meaning. This is a **safe subset**, not full SednaLabs read-only SQL feature parity; additional audited AST validation and analytical SQL support are still outstanding.
