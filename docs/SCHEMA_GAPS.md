# 📐 SednaLabs MCP parameter parity audit

**Baseline:** pinned SednaLabs GA4 MCP commit `039fd4620064fc4b76dc6bca641945aba52c14bc`; [verbatim schema fixture](../spec/upstream/sednalabs-tool-schema.v1.json). Current source: `src/main.rs` at revision `3c3e993435956267fe0576324a0f26198749d1bd`. This is a **static source-inspection audit**, not generated runtime MCP schema equality or successful Google API behavior.

**Counts:** 35 upstream tool names; 27 currently have a named Rust handler; 8 have no Rust handler. Among named handlers, 9 have at least one missing **required** parameter, and many additional optional controls and output semantics remain unimplemented. Google's extra annotation-write tool is excluded from this SednaLabs-only matrix.

| Upstream tool | Status | Native input type | Missing required upstream fields | Missing optional count |
|---|---|---|---|---:|
| `batch_run_reports` | ⚠️ Provisional | `RawPropertyRequestArgs` | `requests` | 0 |
| `check_report_compatibility` | ⚠️ Provisional | `RawPropertyRequestArgs` | `dimensions`, `metrics` | 0 |
| `ga4_auth_login_command` | ⚠️ Provisional | `NoArgs` | — | 6 |
| `ga4_auth_status` | ⚠️ Provisional | `NoArgs` | — | 1 |
| `ga4_get_started` | ⚠️ Provisional | `NoArgs` | — | 0 |
| `get_account_data_sharing_settings` | ⚠️ Provisional | `AccountArgs` | — | 0 |
| `get_account_summaries` | ⚠️ Provisional | `NoArgs` | — | 2 |
| `get_custom_dimensions_and_metrics` | ⚠️ Provisional | `PropertyArgs` | — | 0 |
| `get_property_data_retention_settings` | ⚠️ Provisional | `PropertyArgs` | — | 0 |
| `get_property_details` | ⚠️ Provisional | `PropertyArgs` | — | 0 |
| `list_google_ads_links` | ⚠️ Provisional | `PropertyArgs` | — | 2 |
| `list_property_annotations` | ⚠️ Provisional | `PropertyArgs` | — | 2 |
| `preview_report_request` | ⚠️ Provisional | `ReportArgs` | — | 6 |
| `run_account_access_report` | ⚠️ Provisional | `RawAccountRequestArgs` | `date_ranges`, `dimensions`, `metrics` | 11 |
| `run_conversions_report` | ⚠️ Provisional | `RawPropertyRequestArgs` | `date_ranges`, `dimensions`, `metrics`, `conversion_spec` | 12 |
| `run_funnel_report` | ⚠️ Provisional | `RawPropertyRequestArgs` | `funnel_steps` | 13 |
| `run_pivot_report` | ⚠️ Provisional | `RawPropertyRequestArgs` | `date_ranges`, `dimensions`, `metrics`, `pivots` | 11 |
| `run_property_access_report` | ⚠️ Provisional | `RawPropertyRequestArgs` | `date_ranges`, `dimensions`, `metrics` | 11 |
| `run_realtime_report` | ⚠️ Provisional | `RealtimeArgs` | — | 8 |
| `run_report` | ⚠️ Provisional | `ReportArgs` | — | 6 |
| `scratchpad_close_session` | ⚠️ Provisional | `ScratchSessionArgs` | — | 0 |
| `scratchpad_describe_table` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `table_name` | 5 |
| `scratchpad_drop_table` | ⚠️ Provisional | `ScratchTableArgs` | `table_name` | 1 |
| `scratchpad_export_evidence_bundle` | ❌ Missing | `NOT IMPLEMENTED` | `session_id` | 2 |
| `scratchpad_get_runtime_limits` | ⚠️ Provisional | `NoArgs` | — | 0 |
| `scratchpad_ingest_realtime_report` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `table_name`, `property_id`, `dimensions`, `metrics` | 12 |
| `scratchpad_ingest_report` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `table_name`, `property_id`, `date_ranges`, `dimensions`, `metrics` | 13 |
| `scratchpad_landing_param_shift_report` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `table_name`, `release_date` | 13 |
| `scratchpad_list_sessions` | ⚠️ Provisional | `NoArgs` | — | 1 |
| `scratchpad_list_tables` | ⚠️ Provisional | `ScratchSessionArgs` | — | 1 |
| `scratchpad_open_session` | ⚠️ Provisional | `NoArgs` | `session_id` | 0 |
| `scratchpad_query` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `sql` | 5 |
| `scratchpad_release_regression_report` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `table_name`, `release_date`, `anchor_event`, `comparison_event` | 11 |
| `scratchpad_set_runtime_limits` | ⚠️ Provisional | `ScratchLimitArgs` | — | 1 |
| `scratchpad_summarize_table` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `table_name` | 5 |

## 🎯 Priority repairs

1. **Core read reports:** `run_report`, `run_realtime_report` and `preview_report_request` need the full upstream pagination/output controls, not only the subset currently declared. Preserve date and dimension semantics.
2. **Alpha/report categories:** `run_conversions_report`, `run_funnel_report`, `run_pivot_report`, `batch_run_reports`, `check_report_compatibility`, and access reports currently accept generic `request` blobs or reduced fields. Add explicit validated upstream arguments and maintain the provider JSON escape hatch only if safe.
3. **Scratchpad:** complete seven lifecycle signatures before marking them parity-complete; notably `scratchpad_open_session` requires a `session_id` in the pinned reference, and `scratchpad_drop_table` expects `table_name`, while the provisional Rust API differs. Remaining eight analytic tools still need implementation.
4. **Authentication and collection pagination:** expose safe optional native setup inputs and page controls from the snapshot, while honoring scopes and quota bounds.
5. **Output contract:** audit SednaLabs Contract V1 `ok/data/meta` and `ok/error/meta` envelopes and response projections. Rust source field presence alone cannot establish output equivalence.

**Audit method:** handler names parsed from Rust `#[tool]` method declarations and field names inspected from known request structs, compared with required/optional names in the pinned upstream `inputSchema`. This intentionally does not assert that a same-named field has compatible type/defaults, that runtime validation is equivalent, or that a route passes live GA4 integration tests. Future work should replace this static audit with generated MCP tools/list contract diffs and per-tool fixture execution.
