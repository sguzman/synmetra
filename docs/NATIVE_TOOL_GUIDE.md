# 📚 Synmetra native MCP tool parameter guide (36 names)

**Source of truth:** the exact input schemas exported by Synmetra's Rust rmcp router under the full opt-in capability profile, captured in [native snapshot](../spec/native/tool-schema.v1.json) from [feature CI](https://github.com/sguzman/synmetra/actions/runs/40269115096). The names are checked against [the pinned 36-tool inventory](tool-inventory.json). **This is a native tool guide, not proof of SednaLabs/Google parity or live GA authorization.** See [upstream schema parity audit](SCHEMA_PARITY_AUDIT.md) for known differences.

Each example below is a **synthetic input fixture**. A property ID of `42`, account ID `42`, and session `fixture_session` are placeholders. Do **not** run Google-reading examples without your own authorized test property; do **not** run the annotation write without separate explicit approval. Local scratchpad examples assume a prior `scratchpad_open_session` and, where needed, ingested synthetic table rows. Nested schema definitions and additional optional parameter types are in the snapshot, not inferred from upstream. Default Google provider responses are projected into native MCP envelopes with explicit error/category metadata; outbound Google Data API eligibility remains unverified.

The same native contract may be carried as JSON-RPC structured content and legacy JSON text. Common report projection options are `output_mode` (`rows`, `tuples`, `scalar`, `compact`), `summary_only`, `max_rows` and `max_cell_chars` when advertised for that specific tool. Never send unadvertised options just because another tool accepts them.

### `batch_run_reports`

**Required inputs:** `property_id`, `requests`. **Optional inputs:** none.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** 1-5 report requests; outbound Google response <=4 MiB and read-only retries limited to three total attempts.

**Synthetic example arguments:**

```json
{
  "property_id": 42,
  "requests": [
    {
      "date_ranges": [
        {
          "start_date": "7daysAgo",
          "end_date": "today"
        }
      ],
      "dimensions": [
        "country"
      ],
      "metrics": [
        "activeUsers"
      ]
    }
  ]
}
```

### `check_report_compatibility`

**Required inputs:** `property_id`, `dimensions`, `metrics`. **Optional inputs:** none.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Google bearer required for live request; fixed Google HTTPS origins, <=4 MiB provider response, timeout and read-only retry caps. Live property/role acceptance still open.

**Synthetic example arguments:**

```json
{
  "property_id": 42,
  "dimensions": [
    "country"
  ],
  "metrics": [
    "activeUsers"
  ]
}
```

### `create_property_annotation`

**Required inputs:** `property_id`, `title`, `start_date`. **Optional inputs:** `color`, `description`, `end_date`.

**Output:** Google Admin v1alpha mutation; hidden unless --allow-write and an authorized edit-scoped identity. This example must never be executed on an unapproved property.

**Limits / permissions:** Explicit Google analytics.edit and edit-capable property permission; title <=60 chars, description <=150 chars; annotation writes never auto-retry.

**Synthetic example arguments:**

```json
{
  "property_id": 42,
  "title": "Synthetic QA annotation",
  "start_date": "2026-10-25"
}
```

### `ga4_auth_login_command`

**Required inputs:** none. **Optional inputs:** `account`, `callback_port`, `client_id_file`, `headless`, `quota_project`, `shared_adc`.

**Output:** Local setup/preflight response; no Google API or live property access for this helper.

**Limits / permissions:** Command renderer only; unsupported quota_project/shared_adc options are rejected explicitly. Never include secrets in chat, logs or commits.

**Synthetic example arguments:**

```json
{}
```

### `ga4_auth_status`

**Required inputs:** none. **Optional inputs:** `verify_token`.

**Output:** Local setup/preflight response; no Google API or live property access for this helper.

**Limits / permissions:** No Google API call; safe local guidance/status only; real OAuth consent must be initiated separately.

**Synthetic example arguments:**

```json
{}
```

### `ga4_get_started`

**Required inputs:** none. **Optional inputs:** none.

**Output:** Local setup/preflight response; no Google API or live property access for this helper.

**Limits / permissions:** No Google API call; safe local guidance/status only; real OAuth consent must be initiated separately.

**Synthetic example arguments:**

```json
{}
```

### `get_account_data_sharing_settings`

**Required inputs:** `account_id`. **Optional inputs:** none.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Google bearer required for live request; fixed Google HTTPS origins, <=4 MiB provider response, timeout and read-only retry caps. Live property/role acceptance still open.

**Synthetic example arguments:**

```json
{
  "account_id": 42
}
```

### `get_account_summaries`

**Required inputs:** none. **Optional inputs:** `max_pages`, `page_size`.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Collection page_size 1-200; max_pages 1-100; total collection <=10000 entries / 16 MiB; no silent truncation.

**Synthetic example arguments:**

```json
{}
```

### `get_custom_dimensions_and_metrics`

**Required inputs:** `property_id`. **Optional inputs:** none.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Google bearer required for live request; fixed Google HTTPS origins, <=4 MiB provider response, timeout and read-only retry caps. Live property/role acceptance still open.

**Synthetic example arguments:**

```json
{
  "property_id": 42
}
```

### `get_property_data_retention_settings`

**Required inputs:** `property_id`. **Optional inputs:** none.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Google bearer required for live request; fixed Google HTTPS origins, <=4 MiB provider response, timeout and read-only retry caps. Live property/role acceptance still open.

**Synthetic example arguments:**

```json
{
  "property_id": 42
}
```

### `get_property_details`

**Required inputs:** `property_id`. **Optional inputs:** none.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Google bearer required for live request; fixed Google HTTPS origins, <=4 MiB provider response, timeout and read-only retry caps. Live property/role acceptance still open.

**Synthetic example arguments:**

```json
{
  "property_id": 42
}
```

### `list_google_ads_links`

**Required inputs:** `property_id`. **Optional inputs:** `max_pages`, `page_size`.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Collection page_size 1-200; max_pages 1-100; total collection <=10000 entries / 16 MiB; no silent truncation.

**Synthetic example arguments:**

```json
{
  "property_id": 42
}
```

### `list_property_annotations`

**Required inputs:** `property_id`. **Optional inputs:** `max_pages`, `page_size`.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Collection page_size 1-200; max_pages 1-100; total collection <=10000 entries / 16 MiB; no silent truncation.

**Synthetic example arguments:**

```json
{
  "property_id": 42
}
```

### `preview_report_request`

**Required inputs:** `property_id`, `date_ranges`, `dimensions`, `metrics`. **Optional inputs:** `currency_code`, `cursor`, `dimension_filter`, `keep_empty_rows`, `limit`, `max_cell_chars`, `max_rows`, `metric_filter`, `offset`, `order_bys`, `output_mode`, `return_property_quota`, `summary_only`.

**Output:** Local setup/preflight response; no Google API or live property access for this helper.

**Limits / permissions:** Pure local request preview, not an authorized Google call; 64 KiB request size policy and report field validation apply.

**Synthetic example arguments:**

```json
{
  "property_id": 42,
  "date_ranges": [
    {
      "start_date": "7daysAgo",
      "end_date": "today"
    }
  ],
  "dimensions": [
    "country"
  ],
  "metrics": [
    "activeUsers"
  ]
}
```

### `run_account_access_report`

**Required inputs:** `account_id`, `date_ranges`, `dimensions`, `metrics`. **Optional inputs:** `cursor`, `dimension_filter`, `limit`, `max_cell_chars`, `max_rows`, `metric_filter`, `offset`, `order_bys`, `output_mode`, `summary_only`, `time_zone`.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Google bearer required for live request; fixed Google HTTPS origins, <=4 MiB provider response, timeout and read-only retry caps. Live property/role acceptance still open.

**Synthetic example arguments:**

```json
{
  "account_id": 42,
  "date_ranges": [
    {
      "start_date": "7daysAgo",
      "end_date": "today"
    }
  ],
  "dimensions": [
    "userEmail"
  ],
  "metrics": [
    "accessCount"
  ]
}
```

### `run_conversions_report`

**Required inputs:** `property_id`, `date_ranges`, `dimensions`, `metrics`, `conversion_spec`. **Optional inputs:** `currency_code`, `cursor`, `dimension_filter`, `limit`, `max_cell_chars`, `max_rows`, `metric_filter`, `offset`, `order_bys`, `output_mode`, `return_property_quota`, `summary_only`.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** v1alpha conversion eligibility not verified; conversion action count bounded; projected max_rows <=1000, max_cell_chars <=2048.

**Synthetic example arguments:**

```json
{
  "property_id": 42,
  "date_ranges": [
    {
      "start_date": "7daysAgo",
      "end_date": "today"
    }
  ],
  "dimensions": [
    "country"
  ],
  "metrics": [
    "activeUsers"
  ],
  "conversion_spec": {
    "attribution_model": "LAST_CLICK",
    "conversion_actions": []
  }
}
```

### `run_funnel_report`

**Required inputs:** `property_id`, `funnel_steps`. **Optional inputs:** `date_ranges`, `dimension_filter`, `funnel_breakdown`, `funnel_next_action`, `funnel_visualization_type`, `is_open_funnel`, `limit`, `max_cell_chars`, `max_rows`, `output_mode`, `return_property_quota`, `segments`, `summary_only`.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Google v1alpha eligibility not verified; 1-10 funnel steps; bounded segments and dates; opt-in projected max_rows <=1000 and max_cell_chars <=2048.

**Synthetic example arguments:**

```json
{
  "property_id": 42,
  "funnel_steps": [
    {
      "event": "page_view"
    }
  ]
}
```

### `run_pivot_report`

**Required inputs:** `property_id`, `date_ranges`, `dimensions`, `metrics`, `pivots`. **Optional inputs:** `currency_code`, `cursor`, `dimension_filter`, `keep_empty_rows`, `max_cell_chars`, `max_rows`, `metric_filter`, `order_bys`, `output_mode`, `return_property_quota`, `summary_only`.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Pivot fieldNames must match dimensions and per-axis limits be positive; combined pivot size <=250000; no flat pivot cursor.

**Synthetic example arguments:**

```json
{
  "property_id": 42,
  "date_ranges": [
    {
      "start_date": "7daysAgo",
      "end_date": "today"
    }
  ],
  "dimensions": [
    "country"
  ],
  "metrics": [
    "activeUsers"
  ],
  "pivots": [
    {
      "fieldNames": [
        "country"
      ],
      "limit": "10"
    }
  ]
}
```

### `run_property_access_report`

**Required inputs:** `property_id`, `date_ranges`, `dimensions`, `metrics`. **Optional inputs:** `cursor`, `dimension_filter`, `limit`, `max_cell_chars`, `max_rows`, `metric_filter`, `offset`, `order_bys`, `output_mode`, `summary_only`, `time_zone`.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Google bearer required for live request; fixed Google HTTPS origins, <=4 MiB provider response, timeout and read-only retry caps. Live property/role acceptance still open.

**Synthetic example arguments:**

```json
{
  "property_id": 42,
  "date_ranges": [
    {
      "start_date": "7daysAgo",
      "end_date": "today"
    }
  ],
  "dimensions": [
    "userEmail"
  ],
  "metrics": [
    "accessCount"
  ]
}
```

### `run_realtime_report`

**Required inputs:** `property_id`, `dimensions`, `metrics`. **Optional inputs:** `cursor`, `dimension_filter`, `limit`, `max_cell_chars`, `max_rows`, `metric_filter`, `offset`, `order_bys`, `output_mode`, `return_property_quota`, `summary_only`.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Realtime offset/cursor not supported; provider response <=4 MiB; optional bounded projection max_rows <=1000, max_cell_chars <=2048.

**Synthetic example arguments:**

```json
{
  "property_id": 42,
  "dimensions": [
    "country"
  ],
  "metrics": [
    "activeUsers"
  ]
}
```

### `run_report`

**Required inputs:** `property_id`, `date_ranges`, `dimensions`, `metrics`. **Optional inputs:** `currency_code`, `cursor`, `dimension_filter`, `keep_empty_rows`, `limit`, `max_cell_chars`, `max_rows`, `metric_filter`, `offset`, `order_bys`, `output_mode`, `return_property_quota`, `summary_only`.

**Output:** Google Analytics provider-shaped data under the MCP ok/data/meta envelope. A tool error uses ok/error/meta. Some report tools support opt-in bounded projections; no real Google success has been verified.

**Limits / permissions:** Google bearer required for live request; fixed Google HTTPS origins, <=4 MiB provider response, timeout and read-only retry caps. Live property/role acceptance still open.

**Synthetic example arguments:**

```json
{
  "property_id": 42,
  "date_ranges": [
    {
      "start_date": "7daysAgo",
      "end_date": "today"
    }
  ],
  "dimensions": [
    "country"
  ],
  "metrics": [
    "activeUsers"
  ]
}
```

### `scratchpad_close_session`

**Required inputs:** `session_id`. **Optional inputs:** none.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** Only with --features scratchpad and serve --scratchpad on trusted stdio; in-memory session max 20000 rows; feature security gates still open.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session"
}
```

### `scratchpad_describe_table`

**Required inputs:** `session_id`, `table_name`. **Optional inputs:** none.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** Only with --features scratchpad and serve --scratchpad on trusted stdio; in-memory session max 20000 rows; feature security gates still open.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session",
  "table_name": "events"
}
```

### `scratchpad_drop_table`

**Required inputs:** `session_id`, `table_name`. **Optional inputs:** `if_exists`.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** Only with --features scratchpad and serve --scratchpad on trusted stdio; in-memory session max 20000 rows; feature security gates still open.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session",
  "table_name": "events"
}
```

### `scratchpad_export_evidence_bundle`

**Required inputs:** `session_id`. **Optional inputs:** `sample_rows_per_table`, `table_names`.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** At most 8 tables, 20 sampled rows/table, <=2048 bytes per sampled row and <=256 KiB serialized output; no server-side arbitrary export path.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session"
}
```

### `scratchpad_get_runtime_limits`

**Required inputs:** none. **Optional inputs:** none.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** Only with --features scratchpad and serve --scratchpad on trusted stdio; in-memory session max 20000 rows; feature security gates still open.

**Synthetic example arguments:**

```json
{}
```

### `scratchpad_ingest_realtime_report`

**Required inputs:** `session_id`, `table_name`, `property_id`, `dimensions`, `metrics`. **Optional inputs:** `append`, `dimension_filter`, `limit`, `metric_filter`, `offset`, `order_bys`, `return_property_quota`.

**Output:** Authorized Google Data API request followed by an opt-in local DuckDB write; output includes local ingestion status, not raw live data in this guide.

**Limits / permissions:** Only with --features scratchpad and serve --scratchpad on trusted stdio; in-memory session max 20000 rows; feature security gates still open.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session",
  "table_name": "events",
  "property_id": 42,
  "dimensions": [
    "country"
  ],
  "metrics": [
    "activeUsers"
  ]
}
```

### `scratchpad_ingest_report`

**Required inputs:** `session_id`, `table_name`, `property_id`, `dimensions`, `metrics`, `date_ranges`. **Optional inputs:** `append`, `currency_code`, `dimension_filter`, `limit`, `metric_filter`, `offset`, `order_bys`, `return_property_quota`.

**Output:** Authorized Google Data API request followed by an opt-in local DuckDB write; output includes local ingestion status, not raw live data in this guide.

**Limits / permissions:** Only with --features scratchpad and serve --scratchpad on trusted stdio; in-memory session max 20000 rows; feature security gates still open.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session",
  "table_name": "events",
  "property_id": 42,
  "dimensions": [
    "country"
  ],
  "metrics": [
    "activeUsers"
  ],
  "date_ranges": [
    {
      "start_date": "7daysAgo",
      "end_date": "today"
    }
  ]
}
```

### `scratchpad_landing_param_shift_report`

**Required inputs:** `session_id`, `table_name`, `release_date`. **Optional inputs:** `channel_column`, `cursor`, `date_column`, `landing_url_column`, `output_mode`, `post_days`, `pre_days`, `source_medium_column`, `summary_only`, `top_n`, `transition_days`.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** Only with --features scratchpad and serve --scratchpad on trusted stdio; in-memory session max 20000 rows; feature security gates still open.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session",
  "table_name": "events",
  "release_date": "2026-10-25"
}
```

### `scratchpad_list_sessions`

**Required inputs:** none. **Optional inputs:** `limit`.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** In-memory single-client session scope; 30-minute expiry, default 4/hard 8 concurrent sessions, hard 16 tables per session. HTTP scratchpad intentionally disabled.

**Synthetic example arguments:**

```json
{}
```

### `scratchpad_list_tables`

**Required inputs:** `session_id`. **Optional inputs:** `limit`.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** Only with --features scratchpad and serve --scratchpad on trusted stdio; in-memory session max 20000 rows; feature security gates still open.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session"
}
```

### `scratchpad_open_session`

**Required inputs:** `session_id`. **Optional inputs:** none.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** In-memory single-client session scope; 30-minute expiry, default 4/hard 8 concurrent sessions, hard 16 tables per session. HTTP scratchpad intentionally disabled.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session"
}
```

### `scratchpad_query`

**Required inputs:** `session_id`, `sql`. **Optional inputs:** `cursor`, `max_cell_chars`, `max_rows`, `output_mode`, `summary_only`.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** SELECT-only reconstructed grammar; max_rows 1-1000, max_cell_chars 1-2048; 256 KiB response budget; no external file/SQL functions.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session",
  "sql": "SELECT COUNT(*) FROM events"
}
```

### `scratchpad_release_regression_report`

**Required inputs:** `session_id`, `table_name`, `release_date`, `anchor_event`, `comparison_event`. **Optional inputs:** `cursor`, `date_column`, `event_column`, `metric_column`, `output_mode`, `post_days`, `pre_days`, `summary_only`, `transition_days`.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** Only with --features scratchpad and serve --scratchpad on trusted stdio; in-memory session max 20000 rows; feature security gates still open.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session",
  "table_name": "events",
  "release_date": "2026-10-25",
  "anchor_event": "page_view",
  "comparison_event": "session_start"
}
```

### `scratchpad_set_runtime_limits`

**Required inputs:** none. **Optional inputs:** `max_sessions`, `max_tables_per_session`.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** In-memory single-client session scope; 30-minute expiry, default 4/hard 8 concurrent sessions, hard 16 tables per session. HTTP scratchpad intentionally disabled.

**Synthetic example arguments:**

```json
{
  "max_sessions": 4,
  "max_tables_per_session": 8
}
```

### `scratchpad_summarize_table`

**Required inputs:** `session_id`, `table_name`. **Optional inputs:** none.

**Output:** Local DuckDB operation; result is an MCP ok/data/meta envelope with bounded local structured data, not a Google provider response.

**Limits / permissions:** Only with --features scratchpad and serve --scratchpad on trusted stdio; in-memory session max 20000 rows; feature security gates still open.

**Synthetic example arguments:**

```json
{
  "session_id": "fixture_session",
  "table_name": "events"
}
```

## Acceptance limits

Schema locks catch changes to *advertised native inputs*. They do not verify Google API responses, live OAuth/property roles, alpha eligibility, external MCP client compatibility, DuckDB multi-user isolation or complete upstream semantics. Consult [LIVE_ACCEPTANCE_MATRIX.md](LIVE_ACCEPTANCE_MATRIX.md), [CONTRACT.md](CONTRACT.md), [ROADMAP.md](ROADMAP.md), and [SECURITY.md](SECURITY.md) before treating any example as production-ready.
