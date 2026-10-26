# 🔍 Pinned upstream parameter-level union audit

**Scope:** all 36 names from the two pinned upstreams. This report compares SednaLabs' JSON input schema against extracted Google Python function signatures. It is an upstream-to-upstream comparison, **not** a statement that Synmetra already implements the union.

- SednaLabs source: `sednalabs/ga4-mcp@039fd462`, [pinned input schemas](../spec/upstream/sednalabs-tool-schema.v1.json).
- Google source: `googleanalytics/google-analytics-mcp@ce84226b`, [pinned extracted signatures](../spec/upstream/google-official-signatures.json).
- Machine-readable per-tool/field evidence: [upstream-parameter-union.json](../spec/audits/upstream-parameter-union.json). It retains all SednaLabs properties, nullable types, explicit defaults, bounds, references, enums, and the Google name/required inventory.

## Measured union

| Finding | Count |
| --- | ---: |
| tools in feature union | 36 |
| sednalabs only tools | 26 |
| google only tools | 1 |
| shared tools | 9 |
| shared fields | 43 |
| sednalabs only fields on shared tools | 29 |
| google only fields on shared tools | 0 |
| required mismatches | 0 |

The nine shared tools have **no disagreement about which overlapping parameters are required**. This does not imply type/default/behavior equivalence: the pinned Google extraction contains parameter names but does not capture field annotations, defaults or per-tool response/error schemas.

## Differences on shared tools

| Tool | Shared fields | Only in SednaLabs | Only in Google | Required mismatches |
| --- | ---: | --- | --- | ---: |
| `get_account_summaries` | 0 | `max_pages`, `page_size` | — | 0 |
| `get_custom_dimensions_and_metrics` | 1 | — | — | 0 |
| `get_property_details` | 1 | — | — | 0 |
| `list_google_ads_links` | 1 | `max_pages`, `page_size` | — | 0 |
| `list_property_annotations` | 1 | `max_pages`, `page_size` | — | 0 |
| `run_conversions_report` | 12 | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` | — | 0 |
| `run_funnel_report` | 7 | `dimension_filter`, `funnel_visualization_type`, `is_open_funnel`, `limit`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` | — | 0 |
| `run_realtime_report` | 9 | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` | — | 0 |
| `run_report` | 11 | `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only` | — | 0 |

## Exclusive tools

**Google-only:** `create_property_annotation`. This annotation write is independently gated by the read-only capability profile; matching the tool name is not authorization.

**SednaLabs-only (26):** `batch_run_reports`, `check_report_compatibility`, `ga4_auth_login_command`, `ga4_auth_status`, `ga4_get_started`, `get_account_data_sharing_settings`, `get_property_data_retention_settings`, `preview_report_request`, `run_account_access_report`, `run_pivot_report`, `run_property_access_report`, `scratchpad_close_session`, `scratchpad_describe_table`, `scratchpad_drop_table`, `scratchpad_export_evidence_bundle`, `scratchpad_get_runtime_limits`, `scratchpad_ingest_realtime_report`, `scratchpad_ingest_report`, `scratchpad_landing_param_shift_report`, `scratchpad_list_sessions`, `scratchpad_list_tables`, `scratchpad_open_session`, `scratchpad_query`, `scratchpad_release_regression_report`, `scratchpad_set_runtime_limits`, `scratchpad_summarize_table`.

## Contract resolution and limits

- **Names and required fields:** retain the full union. The SednaLabs-only optional `cursor`, `max_rows`, `max_cell_chars`, `output_mode`, `summary_only`, pagination options, funnel controls, and other parameters must not be silently discarded merely because Google's signature does not mention them.
- **Types, nullability and defaults:** the SednaLabs JSON Schema gives these explicitly for many fields. The extracted Google signature fixture does **not** preserve type annotations or default expressions. Do not fabricate a Google equivalence result. Where a provider-specific behavior conflicts, document the behavior with a separate Google Data API request/response source and tests.
- **Alias handling:** preserve SednaLabs' snake_case MCP inputs and normalize provider camelCase JSON at the Google boundary. Retain intentionally supported runtime aliases as an explicit compatibility extension, not an undocumented substitution.
- **Limits:** preserve numeric width, request and result bounds, per-call page caps, and realtime pagination restrictions as separately tested behavior. The original source fixtures do not prove that Google accepts a supplied SednaLabs-only control.
- **Error responses:** neither the SednaLabs input schema nor extracted Google signature fixture is a per-tool error contract. See [pinned SednaLabs payload contract](../spec/upstream/sednalabs-payload-contract-v1.schema.json) and [native MCP contract](CONTRACT.md) for the `ok/data/meta` and `ok/error/meta` envelopes. The errors remain subject to redacted fixture tests and authorized live validation.
- **Current implementation:** [native schema audit](SCHEMA_PARITY_AUDIT.md) captures an older GitHub Actions native-schema baseline and explicitly predates later implementation work. Re-capture native schemas before claiming current-head 36-tool parameter parity.

## Acceptance boundary

This audit enumerates all fields available from both pinned sources, including optional controls that appear only in one. It deliberately marks unknown Google types/defaults and error behavior as **not captured**, rather than asserting full behavioral equivalence or live Google acceptance.

**Current native-router comparison:** see [CANONICAL_CONTRACT_DECISION.md](CANONICAL_CONTRACT_DECISION.md) and [spec/audits/native-upstream-diff.v1.json](../spec/audits/native-upstream-diff.v1.json). The historical native baseline discussed above predates full top-level field coverage; the pinned upstream-to-upstream union source data remain unchanged.
