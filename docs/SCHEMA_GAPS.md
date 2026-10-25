# 📐 SednaLabs MCP parameter parity audit

**Reference:** pinned SednaLabs GA4 MCP commit `039fd4620064fc4b76dc6bca641945aba52c14bc`, [verbatim JSON schema](../spec/upstream/sednalabs-tool-schema.v1.json). Native source inspected: `src/main.rs` at `12fc60a4ae6b5b498ca9c9eb02a8a84a7f6f2dd5`.

**Static counts:** 35 upstream tools; 29 have a native handler and 6 are not yet implemented. Among the 29 handlers, 2 still lack one or more upstream **required parameter names**. This improves on the previous static audit (nine such gaps) after typing conversion, funnel, pivot, batch and compatibility requests and correcting lifecycle signatures. **This is not runtime signature/behavior parity.** Optional controls, input types/defaults and output projections still differ, and most tools lack live Google acceptance.

| Upstream tool | Native status | Rust argument type | Missing required fields | Missing optional count |
|---|---|---|---|---:|
| `batch_run_reports` | ⚠️ Provisional | `BatchArgs` | — | 0 |
| `check_report_compatibility` | ⚠️ Provisional | `CompatibilityArgs` | — | 0 |
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
| `run_conversions_report` | ⚠️ Provisional | `ConversionArgs` | — | 5 |
| `run_funnel_report` | ⚠️ Provisional | `FunnelArgs` | — | 8 |
| `run_pivot_report` | ⚠️ Provisional | `PivotArgs` | — | 5 |
| `run_property_access_report` | ⚠️ Provisional | `RawPropertyRequestArgs` | `date_ranges`, `dimensions`, `metrics` | 11 |
| `run_realtime_report` | ⚠️ Provisional | `RealtimeArgs` | — | 8 |
| `run_report` | ⚠️ Provisional | `ReportArgs` | — | 6 |
| `scratchpad_close_session` | ⚠️ Provisional | `ScratchSessionArgs` | — | 0 |
| `scratchpad_describe_table` | ⚠️ Provisional | `ScratchReadTableArgs` | — | 5 |
| `scratchpad_drop_table` | ⚠️ Provisional | `ScratchTableArgs` | — | 0 |
| `scratchpad_export_evidence_bundle` | ❌ Missing | `NOT IMPLEMENTED` | `session_id` | 2 |
| `scratchpad_get_runtime_limits` | ⚠️ Provisional | `NoArgs` | — | 0 |
| `scratchpad_ingest_realtime_report` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `table_name`, `property_id`, `dimensions`, `metrics` | 12 |
| `scratchpad_ingest_report` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `table_name`, `property_id`, `date_ranges`, `dimensions`, `metrics` | 13 |
| `scratchpad_landing_param_shift_report` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `table_name`, `release_date` | 13 |
| `scratchpad_list_sessions` | ⚠️ Provisional | `ScratchListArgs` | — | 0 |
| `scratchpad_list_tables` | ⚠️ Provisional | `ScratchListTablesArgs` | — | 0 |
| `scratchpad_open_session` | ⚠️ Provisional | `ScratchOpenArgs` | — | 0 |
| `scratchpad_query` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `sql` | 5 |
| `scratchpad_release_regression_report` | ❌ Missing | `NOT IMPLEMENTED` | `session_id`, `table_name`, `release_date`, `anchor_event`, `comparison_event` | 11 |
| `scratchpad_set_runtime_limits` | ⚠️ Provisional | `ScratchLimitArgs` | — | 0 |
| `scratchpad_summarize_table` | ⚠️ Provisional | `ScratchReadTableArgs` | — | 5 |

## 🧪 Remaining acceptance work

- Replace raw `request` wrappers on the two access-report tools with explicit fields; correctly handle property/account privileges and provider request validation.
- Complete missing optional request controls, especially pagination, `output_mode`, `summary_only`, `max_cell_chars`, quota flags, metadata and source-side bounded filters.
- Implement the six remaining DuckDB tools (report/realtime ingestion, restricted SQL query, release comparison, landing-parameter shift, evidence exports) and finish analysis tool output contracts.
- Compare actual Rust `tools/list` JSON schemas with both pinned upstream references, not only source-declared parameter field names, then run synthetic fixtures for all 36 union tools.
- Require allowed-scope live Google API acceptance before claiming any of the 36 tools fully parity-complete.

**Method:** This document is derived from the pinned upstream inputSchema `properties`/`required` fields and review of named native Rust request structs. A field listed as present does not establish equivalent type, default, validation, quota handling, or output behavior. A provisional tool might still reject legitimate upstream inputs. The accompanying [machine-readable JSON](../spec/contract-gaps.json) records the exact comparison for follow-on tests.
