# 📐 MCP Contract V1 compatibility policy

Synmetra targets the **union of 36 named tools**: 35 pinned SednaLabs Rust tools and Google's 10 official Python functions, with nine overlapping. The [pinned tool inventory](tool-inventory.json), [SednaLabs tool input schemas](../spec/upstream/sednalabs-tool-schema.v1.json), [Google function signatures](../spec/upstream/google-official-signatures.json) and [SednaLabs payload schema](../spec/upstream/sednalabs-payload-contract-v1.schema.json) are reference fixtures, not proof of parity.

## Response representation

The native Rust server emits both a JSON text content block and MCP `structuredContent`, with the same envelope. This supports clients that only display text and clients capable of consuming structured results. For successful calls, the envelope contains `ok: true`, `data`, and `meta`. For tool-level failures, it contains `ok: false`, `error` with code/reason/message/category and optional HTTP status, and `meta`. Google API `data` remains provider-shaped for ordinary read calls by default, preserving Google client expectations. Historical `run_report` and `run_realtime_report` now have an **explicit opt-in** Contract V1 tabular projection with four output modes (`rows`, `tuples`, `scalar`, `compact`), `summary_only`, `max_rows`, and `max_cell_chars`. Those controls are validated locally and keep quota/warning metadata under `meta.ga4`. The projection retains known row totals and marks `row_count_total_known: false` when Google does not supply a total, without inventing one.

The `meta.elapsed_ms` value is currently a **zero sentinel, explicitly accompanied by `elapsed_ms_measured: false`**. It does not claim zero latency or compliance with upstream timing accuracy. Future per-tool instrumentation must replace it before complete Contract V1 acceptance. `structuredContent` provides basic interop, with bounded opt-in projection and UTF-8/Unicode-safe clipping in the two core report tools. **Cursor compatibility remains unimplemented**: a supplied `cursor` is rejected rather than ignored, and the projection deliberately emits `next_cursor: null` even for truncated historical reports. For historical pagination use the default provider-shaped response with explicit `offset` and `limit` until a query-bound cursor codec is independently accepted. Realtime reports cannot paginate with offsets. The remaining upstream projections and metadata fields are not fully implemented.

## Capability boundaries

The Google annotation mutation is disabled unless `--allow-write` is explicit and Google separately grants an edit-scoped identity. Scratchpad tools are disabled unless built with the optional DuckDB feature and started with `--scratchpad`. Scratchpad is stdio-only until cross-client session isolation has been implemented. The full 36-tool registry is therefore only visible in the fully opt-in test profile, and normal read-only clients see a subset by design.

## What can count toward parity

Required field names, tool presence, or mock-based HTTP tests are **implementation milestones, not full equivalence**. Every tool still needs contract-schema comparison (types/defaults/descriptions), error and response fixtures, bounded output semantics, appropriate privileges, live Google acceptance where allowed, and end-to-end Codex MCP usage. The [progress ledger](progress.json) records partial checks independently; no tool has passed all upstream parity gates yet.

Pinned reference license: SednaLabs Apache-2.0 license in [licenses](../licenses/SEDNALABS-APACHE-2.0.txt).
