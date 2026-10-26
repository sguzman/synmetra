# 📑 Synmetra native MCP input-schema parity audit

## Provenance and interpretation

- **Captured native code:** [43a69ecc](https://github.com/sguzman/synmetra/commit/43a69ecc66e441c49dbdc8afa81285c0951b8688); source: passing [GitHub Actions 40224261534](https://github.com/sguzman/synmetra/actions/runs/40224261534), native job 121262186234. The run executed the 36-tool router capture test under `--features scratchpad`.
- **Pinned comparisons:** [SednaLabs full MCP JSON schemas](../spec/upstream/sednalabs-tool-schema.v1.json) (35 names, SHA `039fd462`) and [Google official extracted Python signatures](../spec/upstream/google-official-signatures.json) (10 names, SHA `ce84226b`). Google fixture is NOT a full runtime ADK schema.
- **Time boundary:** this is a **historical baseline**. It predates later auth-status, auth-command, and quota-default corrections on `main`. Never present the counts as current-head measurements without a fresh capture.
- **Nature of differences:** this is a raw syntactic comparison of field names, required sets, and selected JSON Schema attributes without resolving `$ref` or normalizing equivalent `anyOf` alternatives. Differences can include schema descriptions or representations, not necessarily behavior failures.

## Snapshot results

| Comparison | Result |
| --- | ---: |
| Native MCP schemas captured | 36 |
| SednaLabs tool schemas compared | 35 |
| SednaLabs tools with ≥1 raw schema difference | 33 |
| Missing SednaLabs optional/required properties (total) | 75 |
| Additional native properties vs SednaLabs | 2 |
| Raw attribute differences | 163 |
| Required-field set mismatches vs SednaLabs | 0 |
| Official Google function signatures compared | 10 |
| Missing native parameters vs official | 2 |
| Additional native parameters vs official | 7 |
| Required-field set mismatches vs official | 0 |

Raw attribute types: `description` 115; `type` 23; `items` 10; `format` 8; `anyOf` 5; `default` 2.

## Missing arguments needing parity decisions

- **Google official signature:** `run_realtime_report` lacks `offset`, `order_bys` relative to the official Python function; native's real-time API constraints need an explicitly documented compatibility contract.
- **SednaLabs:** important missing output controls include `cursor`, `max_rows`, `max_cell_chars`, `summary_only`, and `output_mode` across multiple reports and scratchpad reads. These affect response handling, not just documentation.
- **Funnel:** the pinned SednaLabs tool exposes additional visualization/open-funnel/filter/limit parameters not yet registered in this snapshot.
- **Representations:** numeric-or-string Google property IDs are currently advertised as string-only in several native schemas. Some input groups are also untyped JSON arrays instead of constrained nested objects.
- **Defaults and descriptions:** several native schemas differ in quoted defaults, optional nullability, and description text; resolve per-tool before claiming exact parity.

## Highest raw missing-property counts (captured baseline)

| Tool | Missing property count | Missing names |
| --- | ---: | --- |
| `run_funnel_report` | 8 | `dimension_filter`, `funnel_visualization_type`, `is_open_funnel`, `limit`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |
| `run_realtime_report` | 7 | `cursor`, `max_cell_chars`, `max_rows`, `offset`, `order_bys`, `output_mode`, `summary_only` |
| `ga4_auth_login_command` | 6 | `account`, `callback_port`, `client_id_file`, `headless`, `quota_project`, `shared_adc` |
| `preview_report_request` | 5 | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |
| `run_account_access_report` | 5 | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |
| `run_conversions_report` | 5 | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |
| `run_pivot_report` | 5 | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |
| `run_property_access_report` | 5 | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |
| `run_report` | 5 | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |
| `scratchpad_describe_table` | 5 | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` |

## Reproducible evidence

- [Captured 36-tool native input schemas](../spec/audits/native-tool-input-schemas-40224261534.json).
- [Complete per-tool input-schema differences](../spec/audits/input-schema-differences-40224261534.json), including both pinned upstream references, required-field comparisons, and field-attribute differences.
- Do not mark individual tool parity as accepted from this audit alone. This records what remains to implement or justify; behavior, output envelopes, security and authorized live Google tests are separate gates.

## Post-capture remediation (not reflected in baseline counts)

- [Realtime compatibility correction](https://github.com/sguzman/synmetra/commit/2d74a1493d60cf2cc92ac7472febf0961d527910): native `run_realtime_report` now accepts official `order_bys` and `offset` arguments, forwards `orderBys`, and rejects nonzero `offset` instead of silently ignoring it. Google's [current v1beta request schema](https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/properties/runRealtimeReport) supports `orderBys` but not `offset`. The official Python function still declares `offset`; the semantic discrepancy is explicitly documented rather than counted as achieved parity.
- [Realtime envelope regression](https://github.com/sguzman/synmetra/commit/c24502b6fe5d882eae2503eabddfc9a8838562fa): synthetic provider response retains row count and quota with no fabricated continuation token. GitHub Actions 40227043192 core tests passed; native DuckDB job is a separate acceptance gate.

## Current native property-name recheck (2026-10-26)

A later **feature-enabled** full-profile router capture from [Synmetra `be2bb3c`, GitHub Actions 40269115096](https://github.com/sguzman/synmetra/actions/runs/40269115096) has now been locked as [spec/native/tool-schema.v1.json](../spec/native/tool-schema.v1.json). The native snapshot is subsequently checked for byte-equivalent JSON schema semantics by Rust's full-profile feature test. This section compares **top-level advertised property names only**, not all nested field types/defaults, return schemas or live behavior.

| Comparison | Current capture |
| --- | ---: |
| Native tools captured | 36 |
| Pinned SednaLabs tools compared | 35 |
| Tools with property-name set differences | 8 |
| Missing SednaLabs property names | **24** |
| Native-only extra property names | 2 |
| Missing Google official extracted parameter names | **0** |

All 24 missing names are confined to six scratchpad tools: `scratchpad_describe_table`, `scratchpad_ingest_realtime_report`, `scratchpad_ingest_report`, `scratchpad_summarize_table` (five output/paging options each: `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only`), and `scratchpad_landing_param_shift_report` / `scratchpad_release_regression_report` (two options each: `max_cell_chars`, `max_rows`). The two native-only properties are `keep_empty_rows` on `preview_report_request` and `run_report`.

This reduces the historical field-name gap from 75 to 24; **it does not establish contract parity**. Input type/description/default drift, SednaLabs response projection differences, Google account role/alpha behavior and full client conformance remain unverified. The [migration guide](MIGRATION.md), [native per-tool guide](NATIVE_TOOL_GUIDE.md), and T164–T165 acceptance tasks track these boundaries. The historical table above is retained with its original source SHA and must not be rewritten to imply later acceptance.

## Follow-on table metadata projection update (feature CI pending)

The native source now adds opt-in **bounded column metadata projections** for both `scratchpad_describe_table` and `scratchpad_summarize_table`, including `cursor`, `max_cell_chars`, `max_rows`, `output_mode` and `summary_only`. The default response remains unchanged. This removes ten missing *advertised top-level* fields relative to the preceding 24-field snapshot, leaving **14 missing SednaLabs fields in four tools** (historical/realtime ingestion: five each; release/landing diagnostics: two each). The two native-only `keep_empty_rows` fields remain. The [native schema lock](../spec/native/tool-schema.v1.json) has been revised for this implementation and must pass a native feature test before treating this as a verified current-router measurement.

A metadata cursor fingerprints the validated session, table and column catalog and is not a Google row/report cursor. Projection only shapes a maximum 32 visible DuckDB column descriptors and does **not** add pagination over full table data, bypass DuckDB's session isolation limits, or establish full upstream response equivalence. This update is an implementation improvement under T164–T165, not complete 36-tool parity.

## Follow-on diagnostic projection update (feature CI pending)

The local `scratchpad_release_regression_report` and `scratchpad_landing_param_shift_report` now advertise and implement `max_rows` and `max_cell_chars` as opt-in bounded output controls (1–1000 rows, 1–2048 characters). Ranked landing shifts are truncated with `groups_total` preserved; the release diagnostic remains scalar and honors string clipping. These remove four further advertised top-level differences. The current reviewed native schema lock therefore has **10 missing SednaLabs top-level inputs**, limited to five output/paging controls each on `scratchpad_ingest_report` and `scratchpad_ingest_realtime_report`, plus the two existing native-only `keep_empty_rows` fields. Live response parity remains explicitly open, and this change must pass feature-enabled CI before it can be credited.
