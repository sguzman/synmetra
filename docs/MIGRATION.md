# 🔀 Migrating from the pinned Google or SednaLabs GA4 MCP servers

**Purpose:** plan a controlled, non-destructive migration to the experimental standalone Rust `synmetra` binary. This is **not** a drop-in parity guarantee. No authorized live Google Analytics property QA or production release is certified.

## Pinned source baselines and native inventory

- Google's [official Analytics MCP reference](https://github.com/googleanalytics/google-analytics-mcp/tree/ce84226bf4cedea7f2910f5afb07a6b9138f6553) is Python-based with **10** named tools.
- [SednaLabs GA4 MCP reference](https://github.com/sednalabs/ga4-mcp/tree/039fd4620064fc4b76dc6bca641945aba52c14bc) is Rust with **35** named tools.
- Nine names overlap. Synmetra targets the **36-name union**: default read-only stdio exposes 20, the optional `--scratchpad` profile exposes 15 local operations in addition, and the optional `--allow-write` profile adds the one Google-only annotation write.
- The native tool-name and JSON input-schema snapshot is [spec/native/tool-schema.v1.json](../spec/native/tool-schema.v1.json), captured from Rust CI; source details are in [PROVENANCE.md](PROVENANCE.md). A matching name is not necessarily a matching JSON output or successfully authorized Google request.

## Current input-field differences to account for

A direct **property-name-only** comparison of the 35 pinned SednaLabs schemas to the captured native router has **8 tools with differing property sets**, **24 SednaLabs inputs not advertised natively**, and **2 native-only properties**. This is a narrower and newer assessment than the historical [SCHEMA_PARITY_AUDIT.md](SCHEMA_PARITY_AUDIT.md), which measured 75 missing fields before subsequent implementation. It does **not** compare nested types/defaults or prove behavioral equivalence.

| Tool(s) | SednaLabs inputs currently absent from native schema |
| --- | --- |
| `scratchpad_describe_table` | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |
| `scratchpad_ingest_realtime_report` | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |
| `scratchpad_ingest_report` | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |
| `scratchpad_summarize_table` | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |
| `scratchpad_landing_param_shift_report` | `max_cell_chars`, `max_rows` |
| `scratchpad_release_regression_report` | `max_cell_chars`, `max_rows` |

`preview_report_request` and `run_report` additionally advertise a native `keep_empty_rows` option. The **10 official extracted Google function signature parameter names have no missing native names** in this capture, but Google's fixture does not include full dynamic MCP type/default/output schemas. In particular, the official Python realtime tool declares `offset`, while Google's v1beta realtime API does not; Synmetra rejects nonzero realtime offsets rather than silently pretending to page.

**Do not silently forward removed/unsupported controls.** Update caller expectations explicitly and keep existing servers available during parallel non-production checks. Restoring the absent scratchpad output controls and documenting intentional compatibility choices remains tracked by T164–T165.

## Tool output and error behavior

Synmetra returns `ok/data/meta` for tool successes and `ok/error/meta` with categories and optional provider HTTP status for tool failures, both in MCP structured content and legacy JSON text. Default Google report `data` remains provider-shaped; opt-in `output_mode`, `max_rows`, `max_cell_chars`, `summary_only` use a bounded projection for advertised report tools. `elapsed_ms: 0` is an **unmeasured sentinel**, disclosed with `elapsed_ms_measured: false`; it is not zero actual latency.

Historical cursors are query-bound, offset-bounded and not a snapshot; some providers/tool families have no cursor. See [CONTRACT.md](CONTRACT.md), the native [tool guide](NATIVE_TOOL_GUIDE.md), and [tool provenance](TOOL_PROVENANCE.md). Do not assume an agent that consumes SednaLabs' scalar/compact data can directly consume provider JSON unchanged.

## Authentication, transports and native dependencies

- Synmetra's preferred setup is a private Google **Desktop OAuth** client JSON with native PKCE and isolated XDG token storage. It does **not** modify or reuse shared gcloud ADC by default, and does not yet implement every optional upstream quota-project/shared-ADC/service-account mode. `ga4_auth_login_command` rejects `shared_adc=true` or `quota_project` rather than silently misrepresenting support. See [GOOGLE_CLOUD.md](GOOGLE_CLOUD.md).
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
