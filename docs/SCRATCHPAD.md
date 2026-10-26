# 🗃️ Optional DuckDB scratchpad, first vertical slice

Synmetra now contains an **experimental feature-gated native DuckDB** in-memory session manager. DuckDB is compiled from bundled native code through Rust bindings; the MCP executable does not use Python or npm.

Compile with `cargo build --release --features scratchpad`, then opt in using `synmetra serve --scratchpad`. A default read-only Synmetra process does **not** expose scratchpad tools, even when compiled with this feature.

**Fifteen provisional MCP tools:** `scratchpad_open_session`, `scratchpad_close_session`, `scratchpad_list_sessions`, `scratchpad_get_runtime_limits`, `scratchpad_set_runtime_limits`, `scratchpad_list_tables`, `scratchpad_drop_table`, `scratchpad_describe_table`, `scratchpad_summarize_table`, and `scratchpad_export_evidence_bundle`. This slice covers session lifecycle, table enumeration, safe deletion, and bounded read-only schema/row-count inspection. The two remaining scratchpad tools are release regression diagnostics and landing-parameter shift diagnostics.

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

### 📉 Release-window instrumentation diagnostics

`scratchpad_release_regression_report` compares a named anchor event against a distinct comparison event in baseline and post-release windows. It uses validated DuckDB identifiers and bounded, generated read-only projections. Defaults are 7 pre-release days, 1 transition day and 7 post-release days. Optional metric weighting must be a finite nonnegative numeric value. Results report anchor/baseline ratios, relative change and conservative flags (`insufficient_baseline`, `low_sample`, `possible_instrumentation_regression`, or `no_large_ratio_drop_detected`). The tool **cannot establish causation** and does not by itself prove an instrumentation failure.

The initial version supports up to 20,000 rows per local table, a bounded date window, and explicitly rejects pagination cursors or non-`rows` output modes. Confidence and output-contract equivalence with the pinned upstream remain unaccepted.

### 🧭 Landing query-parameter shift diagnostics

`scratchpad_landing_param_shift_report` is implemented provisionally. It aggregates the **presence of query-parameter keys**, not their private values, across bounded pre/post release windows. Optional channel and source/medium columns create separate comparison groups. Defaults: 7 pre-release days, 1 transition day and 7 post-release days; up to 200 ranked shifts, 2,000 unique groups, and 20,000 local input rows. Its SQL projections are generated from validated local columns. It never fetches URLs or sends data to an external service.

The first implementation preserves simple ASCII parameter keys only (no percent-decoding), omits unsupported cursor/output-mode controls, and ranks absolute changes in occurrences. It is descriptive and cannot establish causal campaign attribution. Full source-algorithm equivalence and real-property acceptance remain open.

**Tool-name coverage:** all 35 pinned SednaLabs MCP names now have Rust handlers; Google's `create_property_annotation` makes the 36-tool union. Some implementations remain intentionally narrow, and no tool should be described as fully parity-accepted until its individual contract and live/synthetic acceptance gates pass.


## 🦀 Bundled native-library boundary, license and portability review

Synmetra's default Cargo profile has **no DuckDB dependency activated** (default features empty). The optional scratchpad feature selects DuckDB Rust bindings 1.4.2 with default features disabled and the bundled feature enabled; this builds the native DuckDB engine from source through libduckdb-sys and its C API. The MCP hosting process and safety wrappers are Rust, but the embedded SQL engine is **native C/C++**, not pure Rust. No Python, JavaScript package manager, or external database server is required.

DuckDB core and the duckdb-rs bindings identify as **MIT licensed** ([DuckDB core FAQ](https://duckdb.org/faq), [duckdb-rs repository](https://github.com/duckdb/duckdb-rs)). Distribution must retain the applicable notices. Licensing of **all transitive bundled components** still needs a frozen-lockfile dependency/license audit before binary release approval. The top-level MIT label in Synmetra's Cargo manifest is not sufficient by itself.

**Native build and portability:** x86-64 Linux/Arch is the primary target. The bundled feature compiles native C/C++ at build time; compiler, disk, CPU, linker, and ABI requirements differ from the default Rust-only feature set. Passing CI feature tests is not proof of an installable Linux binary, ARM64 portability, or a reproducible stripped/static release. Those M9 acceptance gates remain open.

**Engine and data boundaries:**

- Sessions use an in-memory DuckDB connection with a 30-minute idle TTL, default four-session/eight-table quotas (hard maxima eight/sixteen), and an enforced 20,000-row total per session. Input cells and evidence exports have separate byte/count ceilings. Sessions are held within one process rather than scoped to authenticated clients. The scratchpad **must remain stdio-only** until per-identity HTTP isolation (T143) passes.
- Current source sets external access false, disables automatic extension installation/loading, sets the DuckDB buffer-manager memory limit to 256 MB, caps temporary-directory storage at 64 MB, then locks configuration. See [DuckDB security](https://duckdb.org/docs/lts/operations_manual/securing_duckdb/overview) and [DuckDB memory limits](https://duckdb.org/docs/current/operations_manual/limits). Feature-enabled CI acceptance is required before calling these verified. The memory limit applies to the buffer manager, **not a hard process RSS ceiling**; internal temporary spill limits do not guarantee that no bytes ever touch disk.
- The scratchpad_query tool does not submit caller-provided raw SQL verbatim. Its bounded parser reconstructs a limited SELECT from catalog-validated identifiers; direct engine external-file checks and immutable configuration checks are separate defense-in-depth gates. This remains a narrow language subset, not general SQL compatibility.
- Evidence exports return bounded JSON/Markdown in the MCP response and accept no destination file path. This does **not** establish cross-client confidentiality: exported analytics samples are intentionally caller-visible, and any CI diagnostics must remain synthetic. No user analytics records, credentials, private conversations or access tokens belong in this public repo.

**Open release and security gates:** T142/T145/T147/T149 cover resource bounds, external access, deterministic evidence, and hostile SQL fixtures; T143 covers authenticated client identity separation; T146 unexpected termination; T148 export privacy/permissions; T150 native dependency/licensing review; T177 independent dependency/license/security scanning; M9 portable release acceptance. Documentation or mocks alone must never make these 100%; record exact CI evidence in docs/progress.json.
