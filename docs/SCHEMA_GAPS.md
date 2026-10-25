# 📐 Synmetra MCP signature audit against pinned SednaLabs

Pinned reference: `sednalabs/ga4-mcp@039fd4620064fc4b76dc6bca641945aba52c14bc`. Native source: `src/main.rs` at `5e781197fdbe94b8206ec303a18a90bfadbecbd1`.

**Static field-name assessment:** **29/35 upstream tools have provisional Rust handlers; 6 remain unimplemented.** All 29 named handlers now declare their upstream **required input field names** (down from nine requiring fixes at the start of this continuation). This **does not mean JSON schema or behavioral parity**: optional controls, input types/defaults, actual Google API execution, response envelopes, pagination and security acceptance still require verification.

| Upstream tool | Status | Native argument type | Missing required | Missing optional count |
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
| `run_account_access_report` | ⚠️ Provisional | `AccountAccessArgs` | — | 5 |
| `run_conversions_report` | ⚠️ Provisional | `ConversionArgs` | — | 5 |
| `run_funnel_report` | ⚠️ Provisional | `FunnelArgs` | — | 8 |
| `run_pivot_report` | ⚠️ Provisional | `PivotArgs` | — | 5 |
| `run_property_access_report` | ⚠️ Provisional | `PropertyAccessArgs` | — | 5 |
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

## 🎯 Next parity gates

1. Implement six missing scratchpad tools: historical/realtime ingestion, policy-restricted SQL query, release regression, landing-parameter shift, and evidence export.
2. Support optional pagination, bounded output modes, date/filter validation and accurate Contract V1 success/error metadata.
3. Generate actual Rust MCP tools/list JSON schemas and compare field types, defaults, required lists and descriptions against pinned upstream fixtures.
4. Exercise every tool against synthetic provider fixtures and then authorized live Google Analytics test properties.

The official Google implementation contains a separately gated annotation-write tool outside SednaLabs' 35-name inventory. **0/36 full tool-parity acceptance** remains the current evidence status.
