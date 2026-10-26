# 🔬 Synmetra live-account eligibility and acceptance matrix

**Scope:** all 36 pinned MCP tool names from [tool-inventory.json](tool-inventory.json); status as of **2026-10-25**. **No live Google Analytics OAuth/property/role QA has been accepted.** No real account IDs, credentials, Google report payloads or personal analytics are committed. This table records *intended* authorization requirements and **unverified** eligibility, not an external certification, current Google policy guarantee, or successful upstream-parity claim.

The core native Rust suite and mock API fixtures exercise many handlers offline. Those tests do **not** establish whether a real property or Google account is authorized, whether a v1alpha endpoint is enabled, whether requests produce exactly equivalent GA4 reports, or whether an authorized write safely commits. Feature acceptance remains separate in [progress.json](progress.json), [ROADMAP.md](ROADMAP.md), and [SCHEMA_PARITY_AUDIT.md](SCHEMA_PARITY_AUDIT.md).

## Per-tool matrix (36/36 names)

| MCP tool | Trust boundary | Intended account/scope eligibility | Authorized live GA evidence | Remaining acceptance |
| --- | --- | --- | --- | --- |
| `batch_run_reports` | Google Data report | analytics.readonly plus GA4 property read access | Unverified | T182 live pivot/batch or T179 compatibility access |
| `check_report_compatibility` | Google Data report | analytics.readonly plus GA4 property read access | Unverified | T182 live pivot/batch or T179 compatibility access |
| `create_property_annotation` | Google Admin mutation | analytics.edit plus edit-capable property role and --allow-write; explicit client approval | Unverified; no write performed | T118–T119 permission/approval; T184 opt-in test-property mutation |
| `ga4_auth_login_command` | Local auth/guidance | Local CLI/MCP handler; no property credentials required | Not applicable: no Google API request | Synthetic local command only; live OAuth via T178 still open |
| `ga4_auth_status` | Local auth/guidance | Local CLI/MCP handler; no property credentials required | Not applicable: no Google API request | Synthetic local command only; live OAuth via T178 still open |
| `ga4_get_started` | Local auth/guidance | Local CLI/MCP handler; no property credentials required | Not applicable: no Google API request | Synthetic local command only; live OAuth via T178 still open |
| `get_account_data_sharing_settings` | Google Admin read | analytics.readonly plus access to relevant account/property; endpoint-specific role not certified | Unverified | T179 live account/property access, T186 redacted evidence |
| `get_account_summaries` | Google Admin read | analytics.readonly plus access to relevant account/property; endpoint-specific role not certified | Unverified | T179 live account/property access, T186 redacted evidence |
| `get_custom_dimensions_and_metrics` | Google Admin read | analytics.readonly plus access to relevant account/property; endpoint-specific role not certified | Unverified | T179 live account/property access, T186 redacted evidence |
| `get_property_data_retention_settings` | Google Admin read | analytics.readonly plus access to relevant account/property; endpoint-specific role not certified | Unverified | T179 live account/property access, T186 redacted evidence |
| `get_property_details` | Google Admin read | analytics.readonly plus access to relevant account/property; endpoint-specific role not certified | Unverified | T179 live account/property access, T186 redacted evidence |
| `list_google_ads_links` | Google Admin read | analytics.readonly plus access to relevant account/property; endpoint-specific role not certified | Unverified | T179 live account/property access, T186 redacted evidence |
| `list_property_annotations` | Google Admin read | analytics.readonly plus access to relevant account/property; endpoint-specific role not certified | Unverified | T179 live account/property access, T186 redacted evidence |
| `preview_report_request` | Local auth/guidance | Local CLI/MCP handler; no property credentials required | Not applicable: no Google API request | Synthetic local command only; live OAuth via T178 still open |
| `run_account_access_report` | Google Admin access report | analytics.readonly plus Google Analytics Administrator access to account/property | Unverified | T182 privileged role and source audit, T186 redacted evidence |
| `run_conversions_report` | Google Data report | analytics.readonly plus GA4 property read access; v1alpha API eligibility applies | Unverified | T181 alpha eligibility and permitted-property proof |
| `run_funnel_report` | Google Data report | analytics.readonly plus GA4 property read access; v1alpha API eligibility applies | Unverified | T181 alpha eligibility and permitted-property proof |
| `run_pivot_report` | Google Data report | analytics.readonly plus GA4 property read access | Unverified | T182 live pivot/batch or T179 compatibility access |
| `run_property_access_report` | Google Admin access report | analytics.readonly plus Google Analytics Administrator access to account/property | Unverified | T182 privileged role and source audit, T186 redacted evidence |
| `run_realtime_report` | Google Data report | analytics.readonly plus GA4 property read access | Unverified | T180 live report vs console comparison |
| `run_report` | Google Data report | analytics.readonly plus GA4 property read access | Unverified | T180 live report vs console comparison |
| `scratchpad_close_session` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_describe_table` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_drop_table` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_export_evidence_bundle` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_get_runtime_limits` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_ingest_realtime_report` | Google Data → local DuckDB | analytics.readonly plus GA4 property read access; trusted --scratchpad stdio client | Unverified | T183 authorized ingest/query/export lifecycle and T186 redacted proof |
| `scratchpad_ingest_report` | Google Data → local DuckDB | analytics.readonly plus GA4 property read access; trusted --scratchpad stdio client | Unverified | T183 authorized ingest/query/export lifecycle and T186 redacted proof |
| `scratchpad_landing_param_shift_report` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_list_sessions` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_list_tables` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_open_session` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_query` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_release_regression_report` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_set_runtime_limits` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |
| `scratchpad_summarize_table` | Local DuckDB | Trusted stdio client with --scratchpad; no Google property role | Not applicable: no Google API request | Synthetic lifecycle and native safety only; T183/T143–T149 open |

## Evidence ledger and role caveats

- **0 authorized GA live-account evaluations** are recorded. No OAuth user-consent, real property discovery, historical/realtime/alpha report verification, Admin role audit, or annotation creation has been credited under T178–T186. A local/mock successful status must never be labeled a live GA pass.
- Local guidance commands, `preview_report_request`, and local DuckDB operations do **not** call Google; "not applicable" means a Google-account live test is not meaningful for that operation. Their own native integration, session isolation, export permissions and security acceptance can still be incomplete.
- `scratchpad_ingest_report` and `scratchpad_ingest_realtime_report` **do** call the Google Data API before local ingestion, so a real authorized read is required to accept their live path. Scratchpad is currently restricted to one trusted stdio client and rejected over HTTP.
- Read-only Admin endpoints can vary in Google resource- and endpoint-specific permissions. **Do not infer a sufficient role just from `analytics.readonly`**: live authorization must record whether a test account has access to a test account/property and the operation's actual result.
- The two access-report tools have a stronger Administrator role requirement; API eligibility and account-vs-property permissions must be verified with an explicitly permitted test identity.
- Conversions and funnel reporting depend on Google Data API v1alpha eligibility. A documented Google API rejection can count as a test of *eligibility behavior* if it is reproducible and attributable to the authorized test account; it is not evidence the endpoint succeeded.
- `create_property_annotation` remains hidden without `--allow-write` and requires explicit `analytics.edit` OAuth consent, an edit-capable property role, an eligible test property and client confirmation. No production account mutation is approved by this matrix.
- Only a future redacted, independently reproducible test ledger with source SHA, Google scope/role tier, tested tool, date/timezone, expected vs observed behavior and explicit operator permission can change a row from **Unverified** to **Accepted**. Never copy account identifiers, auth traces, user tokens or report values into public commits.

## Protocol and release impact

Even if a tool obtains a live Google success, that does not automatically satisfy pinned SednaLabs/Google JSON input-schema parity, Contract V1 response equivalence, DuckDB native isolation or an authenticated MCP client transport test. Stable v1.0.0 and downloadable production packages remain blocked by the distinct M7–M9 acceptance gates. See [LOCAL_EXECUTION_THREAT_MODEL.md](LOCAL_EXECUTION_THREAT_MODEL.md) and [RELEASE_POLICY.md](RELEASE_POLICY.md).
