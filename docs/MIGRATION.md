# 🔀 Migrating from the pinned Google or SednaLabs GA4 MCP servers

**Purpose:** plan a controlled, non-destructive migration to the experimental standalone Rust `synmetra` binary. This is **not** a drop-in parity guarantee. No authorized live Google Analytics property QA or production release is certified.

## Pinned source baselines and native inventory

- Google's [official Analytics MCP reference](https://github.com/googleanalytics/google-analytics-mcp/tree/ce84226bf4cedea7f2910f5afb07a6b9138f6553) is Python-based with **10** named tools.
- [SednaLabs GA4 MCP reference](https://github.com/sednalabs/ga4-mcp/tree/039fd4620064fc4b76dc6bca641945aba52c14bc) is Rust with **35** named tools.
- Nine names overlap. Synmetra targets the **36-name union**: default read-only stdio exposes 20, the optional `--scratchpad` profile exposes 15 local operations in addition, and the optional `--allow-write` profile adds the one Google-only annotation write.
- The native tool-name and JSON input-schema snapshot is [spec/native/tool-schema.v1.json](../spec/native/tool-schema.v1.json), captured from Rust CI; source details are in [PROVENANCE.md](PROVENANCE.md). A matching name is not necessarily a matching JSON output or successfully authorized Google request.

## Current input-field differences to account for

A direct **property-name-only** comparison of the 35 pinned SednaLabs schemas to the reviewed native schema lock (with table metadata projection now added; native feature CI required) has **2 tools with differing property sets**, **0 SednaLabs inputs missing from the reviewed native schema**, and **2 native-only properties**. This is a narrower and newer assessment than the historical [SCHEMA_PARITY_AUDIT.md](SCHEMA_PARITY_AUDIT.md), which measured 75 missing fields before subsequent implementation. It does **not** compare nested types/defaults or prove behavioral equivalence.

| Difference | Remaining fields |
| --- | --- |
| Upstream input names missing | **None in the reviewed native schema lock**. Native ingestion explicitly rejects cursors and summary_only, and currently supports only rows mode. Historical ingest collects at most 20 bounded pages atomically; realtime remains one page. |

`preview_report_request` and `run_report` additionally advertise a native `keep_empty_rows` option. The **10 official extracted Google function signature parameter names have no missing native names** in this capture, but Google's fixture does not include full dynamic MCP type/default/output schemas. In particular, the official Python realtime tool declares `offset`, while Google's v1beta realtime API does not; Synmetra rejects nonzero realtime offsets rather than silently pretending to page.

**Do not silently forward removed/unsupported controls.** Update caller expectations explicitly and keep existing servers available during parallel non-production checks. Table description and summary now have bounded column metadata projections with cursor/session/schema fingerprints. The local release and landing diagnostics now accept bounded max_rows/max_cell_chars projections. Historical and realtime ingest tools advertise every pinned input name; bounded historical multi-page ingestion is implemented, while realtime one-page and non-rows output limits remain. All 36 **input names** are expressible in the schema; T164–T165 and live acceptance remain open for full semantic parity. This does not imply all upstream output semantics match.

## Tool output and error behavior

Synmetra returns `ok/data/meta` for tool successes and `ok/error/meta` with categories and optional provider HTTP status for tool failures, both in MCP structured content and legacy JSON text. Default Google report `data` remains provider-shaped; opt-in `output_mode`, `max_rows`, `max_cell_chars`, `summary_only` use a bounded projection for advertised report tools. `elapsed_ms: 0` is an **unmeasured sentinel**, disclosed with `elapsed_ms_measured: false`; it is not zero actual latency.

Historical cursors are query-bound, offset-bounded and not a snapshot; some providers/tool families have no cursor. See [CONTRACT.md](CONTRACT.md), the native [tool guide](NATIVE_TOOL_GUIDE.md), and [tool provenance](TOOL_PROVENANCE.md). Do not assume an agent that consumes SednaLabs' scalar/compact data can directly consume provider JSON unchanged.

## Authentication, transports and native dependencies

- Synmetra's preferred setup is a private Google **Desktop OAuth** client JSON with native PKCE and isolated XDG token storage. It does **not** modify or reuse shared gcloud ADC by default, and does not yet implement every optional upstream shared-ADC/service-account or per-command quota-project mode. An explicit operator environment override `SYNMETRA_GOOGLE_QUOTA_PROJECT` can send the `x-goog-user-project` quota attribution header but does not grant IAM permissions. `ga4_auth_login_command` still rejects `shared_adc=true` or `quota_project` rather than silently misrepresenting support. See [GOOGLE_CLOUD.md](GOOGLE_CLOUD.md).
- Installation needs Rust and Linux native toolchain but **no Python, npm, Node or gcloud runtime**. Optional embedded DuckDB is native C/C++; its feature build does **not** become pure-Rust. See [INSTALL.md](INSTALL.md) and [SCRATCHPAD.md](SCRATCHPAD.md).
- Use read-only **stdio** for an agent first. Loopback Streamable HTTP requires a separate inbound bearer and rejects remote bind/scratchpad sharing. Use [CODEX.md](CODEX.md) or [MCP_CLIENTS.md](MCP_CLIENTS.md) for reference configurations. Never migrate saved refresh tokens by copying them between implementations or checking them into Git.

## Safe migration sequence

1. Pin a reviewed Synmetra commit and build or install using the locked Rust toolchain. Keep the existing MCP registration intact.
2. Inspect the **actual advertised profile** with synthetic stdio discovery. Compare the names and JSON inputs against [NATIVE_TOOL_GUIDE.md](NATIVE_TOOL_GUIDE.md) and the known field-gap table above. Do not substitute name parity for contract parity.
3. Register Synmetra under its own **distinct** MCP name (for example, `synmetra`) rather than overriding an existing production server. Run the offline help/auth and synthetic CI tests first.
4. Only with express authorization, authenticate a separate safe GA4 test property. Compare authorized historical/realtime and Admin responses, UTC/date boundaries, role restrictions and report totals, recording **redacted** acceptance under T178–T186. Never commit property data or tokens.
5. Keep all annotation writes disabled by default. Opt in to `--allow-write` and edit-scoped credentials only after separate permission/confirmation and a test property are available.
6. Do not deactivate upstream servers or tag Synmetra `v1.0.0` until native schema/output, client interoperability, security and live Google acceptance gates are independently passed.

This guide is specifically for the two **pinned** source revisions, not an audit of all future upstream releases. Revisit schema and Google API changes before relying on the guide for a later revision.

## Input-name closure is not semantic closure

Source-reviewed native ingestion now advertises each pinned upstream optional input field. The upstream explicitly rejects non-null ingest cursors and `summary_only=true`; native does likewise. Native historical `max_rows` caps each Google API page (1–1000), and explicit `limit` up to 20,000 may collect at most 20 pages and 2 MiB of combined JSON before a single transactional insert. Native realtime still reads one page. Receipts report fetched pages, offsets, and whether the requested or known provider range completed. `max_cell_chars` is applied only to the receipt, not DuckDB's persisted values. Alternate `output_mode` variants are rejected before Google network requests rather than silently ignored. Full nested type/default, cursor, projection and output semantics remain open; passing a tool-name or property-set comparison alone must **never** certify parity.

**Width compatibility note:** Native historical and realtime ingest limits both accept pinned `uint64` JSON numbers. Historical ingest accepts 1–20,000 rows over at most 20 bounded pages, with aggregate JSON limited to 2 MiB; realtime remains 1–1000 rows. Excessive limits or page counts fail before Google network access. This remains synthetic compatibility evidence, not live Google parity.
