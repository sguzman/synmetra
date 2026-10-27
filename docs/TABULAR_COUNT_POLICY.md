# 📏 Tabular totals, truncation and cursor acceptance (T159)

**Scope:** experimental Synmetra 0.0.1, native full-feature router, pinned Google/SednaLabs contracts. This is an audit of synthetic metadata semantics, not a declaration of live Google response accuracy, stable provider snapshots or complete upstream parity. Related tests are in `src/tabular.rs`, `src/main.rs` and `src/scratchpad.rs`.

| Tool family | Total and completeness contract | Continuation boundary |
| --- | --- | --- |
| Historical `run_report` and `run_conversions_report` | A valid Google `rowCount` is known; if omitted, `row_count_total_known=false` and only locally observed rows are counted. `truncated` respects the selected offset and rows returned, not earlier pages. | A bounded `v1` cursor only with an explicit trusted total and query/resource SHA. No snapshot guarantee |
| `run_account_access_report` / `run_property_access_report` | The same known/unknown rule applies; the access resource and query contribute to the cursor binding | Scoped historical cursor only; access to the underlying account/property remains a separate role gate |
| `run_realtime_report` | Known/unknown totals are not confused with realtime offset pagination | No cursor or positive offset is accepted |
| `run_pivot_report` | Provider totals and pivot headers are preserved independently of optional flattened projections | No invented flat pivot continuation cursor |
| `run_funnel_report` | Table and visualization subreports have **independent** `row_count_total_known`, counts and truncation flags | No invented cursor for either subreport |
| `batch_run_reports` | Each opt-in projected subreport has its own totals, query binding, truncation and source report metadata. Unprojected results retain their original provider-shaped `rowCount` or absence. | Per-subreport bounded cursor only if its source total is known and its query matches |
| `scratchpad_query` | Bounded local SELECT fetches one look-ahead row, allowing truthful window `truncated`; `returned_rows` remains available in `summary_only`. It does **not** invent a global row total on an ordinary query | No automatic cursor or snapshot; the trusted local caller can issue a new bounded SELECT/OFFSET |
| `scratchpad_describe_table` / `scratchpad_summarize_table` | Legacy no-control output is unchanged. Opt-in catalog projections show exact visible column count only when complete. If `columns_truncated=true`, `column_count_total=null`, `column_count_total_known=false`, and `column_count_lower_bound` reports the visible descriptor count | Catalog cursor is SHA-bound to session/table/visible schema, not table data. No false continuation beyond the 32-column visibility limit |
| `scratchpad_ingest_report` / `scratchpad_ingest_realtime_report` | Local receipts separately expose actual ingested rows/page counts and whether provider collection is complete; a read receipt is not a live-provider snapshot certificate | Bounded 20-page historical ingest; one-page realtime ingest; unsupported cursors fail closed |
| `scratchpad_landing_param_shift_report` / `scratchpad_release_regression_report` | Local diagnostic totals and conservative flags are distinct from Google provider `rowCount`; a bounded ranked landing list is not a count of all events | No Google pagination cursor. These are derived local diagnostics, not an online report |

## Acceptance evidence and remaining limitations

The offline fixtures explicitly exercise known and absent Google `rowCount` in historical, realtime, pivot and access projections, independent funnel subreports, query-scoped batch subreport cursors, and three classes of DuckDB local window behavior. The source tests also cover last-page detection, malformed cursors, UTF-8 clipping, default unprojected passthrough and summary-only behavior. The latest changes add negative tests for an **unknown batch total**, a **truncated local column catalog**, and first/final/summary-only DuckDB SELECT windows. These additions require passing current pinned Rust and native DuckDB CI before T159's final check can be accepted.

`truncated=false` on a window with **unknown upstream row total** means no returned rows were locally dropped; it does **not** prove an external provider has no further results. `row_count_total_known=false` must remain available. A missing next cursor is not proof of completion when a provider total is unknown.

No source/test combination here establishes a real GA property role, client Inspector compatibility, source-snapshot consistency or all upstream SednaLabs output semantics. Keep T164, T161 and T178–T186 separate from this metadata acceptance.
