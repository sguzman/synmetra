# 📌 Pinned upstream reference audit

**Audited:** 2026-10-25. These source revisions are frozen **reference baselines**, not rolling promises of current upstream behavior.

| Reference | Commit SHA | License | Tool names |
| --- | --- | --- | ---: |
| [Google Analytics MCP](https://github.com/googleanalytics/google-analytics-mcp) | [`ce84226bf4cedea7f2910f5afb07a6b9138f6553`](https://github.com/googleanalytics/google-analytics-mcp/tree/ce84226bf4cedea7f2910f5afb07a6b9138f6553) (v0.8.0) | Apache-2.0 | 10 |
| [SednaLabs GA4 MCP](https://github.com/sednalabs/ga4-mcp) | [`039fd4620064fc4b76dc6bca641945aba52c14bc`](https://github.com/sednalabs/ga4-mcp/tree/039fd4620064fc4b76dc6bca641945aba52c14bc) | Apache-2.0 | 35 |

**Observed union: 10 + 35 - 9 = 36 distinct tool names.** Google's 10 count includes `create_property_annotation`, a separately enabled mutating tool. The other nine are shared with SednaLabs' 35-tool read-only-plus-scratchpad surface. Do not confuse the default 9 read-only Google tools with the full 10-name server when `--allow-write` is enabled.

## 🏢 Google's pinned tool surface (10)

- `get_account_summaries`
- `list_google_ads_links`
- `get_property_details`
- `list_property_annotations`
- `get_custom_dimensions_and_metrics`
- `run_report`
- `run_realtime_report`
- `run_funnel_report`
- `run_conversions_report`
- `create_property_annotation` — mutates GA only when --allow-write and analytics.edit are configured

Source: [coordinator.py](https://github.com/googleanalytics/google-analytics-mcp/blob/ce84226bf4cedea7f2910f5afb07a6b9138f6553/analytics_mcp/coordinator.py), [README](https://github.com/googleanalytics/google-analytics-mcp/blob/ce84226bf4cedea7f2910f5afb07a6b9138f6553/README.md).

## 🦀 SednaLabs pinned tool surface (35)

**Helpers (3)**: `ga4_get_started`, `ga4_auth_status`, `ga4_auth_login_command`.

**Admin (7)**: `get_account_summaries`, `get_account_data_sharing_settings`, `get_property_details`, `get_property_data_retention_settings`, `list_google_ads_links`, `list_property_annotations`, `get_custom_dimensions_and_metrics`.

**Reports (8)**: `run_report`, `run_conversions_report`, `run_funnel_report`, `run_realtime_report`, `run_pivot_report`, `batch_run_reports`, `run_property_access_report`, `run_account_access_report`.

**Preflight (2)**: `preview_report_request`, `check_report_compatibility`.

**Scratchpad (15)**: `scratchpad_open_session`, `scratchpad_close_session`, `scratchpad_list_sessions`, `scratchpad_get_runtime_limits`, `scratchpad_set_runtime_limits`, `scratchpad_list_tables`, `scratchpad_drop_table`, `scratchpad_ingest_report`, `scratchpad_ingest_realtime_report`, `scratchpad_query`, `scratchpad_describe_table`, `scratchpad_summarize_table`, `scratchpad_release_regression_report`, `scratchpad_landing_param_shift_report`, `scratchpad_export_evidence_bundle`.

Sources: [TOOL_GUIDE](https://github.com/sednalabs/ga4-mcp/blob/039fd4620064fc4b76dc6bca641945aba52c14bc/docs/TOOL_GUIDE.md), [README](https://github.com/sednalabs/ga4-mcp/blob/039fd4620064fc4b76dc6bca641945aba52c14bc/README.md) and [schema snapshot](https://github.com/sednalabs/ga4-mcp/blob/039fd4620064fc4b76dc6bca641945aba52c14bc/spec/tool_schema_snapshot.v1.json).

## 🔍 Compatibility obligations

1. Freeze **parameter-level** schemas from the exact source commits before implementation; source name coverage alone is insufficient.
2. Verify optional arguments, defaults, Google alpha/beta endpoints, null behavior, errors, quotas, batch limits, complex funnels and attribution.
3. Preserve SednaLabs' response Contract V1 (`ok/data/meta` or `ok/error/meta`) or provide an explicit compatibility adapter for official-style callers where there are semantic differences.
4. Preserve realtime non-paged semantics, arbitrary but bounded standard report offsets/cursors, and scratchpad SQL restrictions.
5. Treat Google property annotations as writes requiring `analytics.edit` and role permission; do not expose on startup by default.
6. Audit and implement 15 scratchpad tools, including runtime limits, sanitized evidence bundles, release regression and landing parameter shift analytics.
7. Detect and record future upstream drift against these fixed baselines and only update the parity target through reviewed commits.

**Not verified:** Tool signatures, output schemas and runtime results have not yet been exhaustively compared. This document pins the tool **name inventory**, not certified behavioral parity.

## 📜 Archived input schemas

[Exact SednaLabs input-schema snapshot](../spec/upstream/tool_schema_snapshot.v1.json) is retained as a pinned, Apache-2.0 attributed test reference. It includes 35 tool names with the original `inputSchema` objects, not acceptance of any Synmetra implementation. Google's 10 dynamic schemas still need a separate authenticated reference export before T007 reaches full completion.

### 📄 Exact SednaLabs schema fixture

The complete 35-tool MCP JSON schema snapshot is archived unchanged at [`spec/upstream/sednalabs-tool-schema.v1.json`](../spec/upstream/sednalabs-tool-schema.v1.json) with the original Apache 2.0 LICENSE in [`licenses/SEDNALABS-APACHE-2.0.txt`](../licenses/SEDNALABS-APACHE-2.0.txt). The snapshot pins argument-level signature expectations and will support differential tests; it **does not** certify native Rust argument or behavior parity. Google's pinned schema capture remains separate work.

### 📗 Google's official Python tool signatures

[Independent signature extraction](../spec/upstream/google-official-signatures.json) records all ten official upstream function parameter lists from pinned commit `ce84226bf4cedea7f2910f5afb07a6b9138f6553`, including the optional gated annotation write. This is a source-derived reference, **not** a copy of Google ADK's runtime-generated MCP JSON schema, and cannot alone establish inputSchema parity. The corresponding pinned Python sources are linked in this document. Future work must compare live generated JSON schemas and error/response behavior.

## 🔔 Automated drift detection

A read-only [weekly GitHub Actions watch](../.github/workflows/upstream-drift.yml) checks public upstream `main` Git blob IDs against [drift-lock.json](../spec/upstream/drift-lock.json) each Monday and whenever the watch or pins change. It checks SednaLabs' published 35-tool schema snapshot and Google's signature-bearing `analytics_mcp/coordinator.py`. A changed blob fails the workflow so source differences must be reviewed and the baseline updated explicitly; no reference or application code is automatically replaced. Google's entire dynamic runtime schema is not captured in this baseline, and a source-file change can be non-semantic. The alert therefore indicates **possible** input/signature drift requiring inspection, not proof of a breaking change.
