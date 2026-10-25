# 🦀 Native MCP implementation status

The executable `synmetra` now boots an experimental **read-only** MCP stdio server using the official Rust `rmcp` SDK. Twenty read-only tool names are advertised by default because they have provisional handlers: `get_account_summaries`, `get_property_details`, `list_google_ads_links`, `list_property_annotations`, `get_custom_dimensions_and_metrics`, `run_report`, `run_realtime_report`, `get_account_data_sharing_settings`, `get_property_data_retention_settings`, `run_pivot_report`, `batch_run_reports`, `check_report_compatibility`, `run_property_access_report`, `run_account_access_report`, `preview_report_request`, `run_funnel_report`, and `run_conversions_report`.

An additional `create_property_annotation` tool is enabled only by `--allow-write`; see [annotation policy](ANNOTATIONS.md). **Not feature parity.** These handlers are an initial slice: they still need upstream parameter-level and output contract comparisons, authorized live GA testing, complete filtering and pagination cases, secure remote HTTP transport, and DuckDB scratchpad. The 36-name ledger remains a *target* inventory, not an advertised set of working tools.

Build and run: `cargo run --bin synmetra -- serve`. For a short-lived local development token, set `SYNMETRA_GOOGLE_ACCESS_TOKEN` to a Google OAuth access token granted `analytics.readonly` (the process must be restarted on expiry). Tokens are neither printed nor persisted. Do not put the token in command-line arguments, shell history, source, CI secrets, or public issue logs. Without a token, the server starts but tools return a structured missing-credentials error.

MCP uses stdout exclusively. No credentials, real GA property IDs or private report bodies are used in automated CI; tests use synthetic fixtures. Network calls target fixed Google API hosts with rustls, bounded timeouts, 4 MiB response guard and no redirects. No Python or npm required by the binary. Native OAuth is coded and offline-tested; authorized real-Google login and refresh QA remain outstanding.

Additional read-only endpoints now include account data sharing, property retention, pivot and batched reports, metric compatibility, and account/property access reports. Advanced report tools currently take a provider-style `request` JSON object, not complete SednaLabs parameter schemas, so upstream parity remains open. Google access reports require Administrator permissions. Batch reports reject zero or more than five requests. The HTTP client refuses unknown operation names.

## 🔑 Native OAuth setup

Use a Google Cloud **Desktop** OAuth client JSON file (from an enabled Analytics Admin + Data API project), then run `synmetra auth login --client-id-file /path/to/client.json`. Synmetra binds a random port on `127.0.0.1`, opens the browser with `xdg-open`, and uses PKCE S256 plus a random callback state. Access is requested with **analytics.readonly** only and `access_type=offline`; no Python, npm, or gcloud is invoked. `synmetra auth status` reports whether private credentials exist; `synmetra auth logout` clears local credentials.

Credentials are written as `$XDG_CONFIG_HOME/synmetra/credentials.json` (or `~/.config/synmetra/credentials.json`) with directory mode 0700 and file mode 0600. Tokens never enter public logs or CLI arguments. The process automatically refreshes an expiring access token using its private refresh token; `SYNMETRA_GOOGLE_ACCESS_TOKEN`, if explicitly set, overrides local storage and does not auto-refresh.

**Security/acceptance:** OAuth is implemented with synthetic callback and PKCE tests, but no real Google login or refresh flow has yet been verified. An interactive OAuth login necessarily requires a one-time browser authorization by the account owner. Do not commit OAuth client JSON, credentials.json or browser callback URLs.

The new v1alpha conversion and funnel handlers are intentionally **eligibility-sensitive**: conversion reports use v1alpha `runReport` with conversionSpec or conversion metrics; funnel reports use v1alpha `runFunnelReport` and bounded step counts. Preview constructs a report body without network access. These are initial request routes rather than completed upstream contract parity.

Three new read-only setup helpers provide commands and safe credential status without returning tokens, client identifiers, or private paths.

The optional loopback Streamable HTTP transport has an authenticated MCP initialization integration test (GitHub Actions run 40204566566). It does not support public remote multiuser access or TLS. See [HTTP policy](HTTP.md).

The provisional `run_conversions_report` handler now takes the official required fields directly (property ID, date ranges, dimension/metric names and conversion specification). It validates supported v1alpha conversion dimensions/metrics, action identifiers, attributions and the 50-action limit before forming a Google request. SednaLabs' pagination/output projection controls, full response schema and live eligibility still require separate parity acceptance.

The provisional `run_funnel_report` tool now uses the official required `funnel_steps` argument and normalizes simple event steps, named steps, breakdowns, next actions, dates and up to four nonempty segments into Data API v1alpha JSON. Invalid/ambiguous step expressions and oversized payloads fail locally. More specialized SednaLabs funnel controls, output projections and live property eligibility remain unverified.

Three additional read-only tools now use typed upstream argument shapes rather than generic provider request blobs: `run_pivot_report` (explicit dates, dimensions, metrics and pivot structures), `batch_run_reports` (1–5 report entries) and `check_report_compatibility` (dimension and metric lists). Input validation is synthetic-testable without Google credentials. Full SednaLabs optional controls, output projections and live API eligibility are separate unverified gates.

Both Admin access-report tools now take explicit required date-range, dimension and metric arguments instead of generic provider JSON. The request builder uses Admin-specific `dimensionName` and `metricName` fields, with bounded input validation and synthetic tests. Authorized Administrator-role GA4 integration, optional SednaLabs projection controls and filter normalization remain unverified.

## 🔎 Bounded GA4 collection pagination

Account summaries and Google Ads links follow Google Admin API `nextPageToken` and merge multiple pages in source order, with a 100-page request limit. To bound persistent-agent memory, Synmetra enforces a maximum of 10,000 collected entries and 16 MiB of serialized items; it rejects overlong (over 1024-byte) or repeated continuation tokens. Two-page, empty/replayed-token, and oversized-result cases are tested against local HTTP mocks. These ceilings can reject very large legitimate properties rather than silently returning incomplete totals. They are implementation safeguards, not full real-Google acceptance.

## 🧪 Structured and shorthand filters

`dimension_filter` and `metric_filter` accept bounded Google FilterExpression objects, JSON-encoded objects, or equality shorthand. Examples: `country==Mexico`, `sessionDefaultChannelGroup=="Paid Other"`, `sessions==7`, and `engagementRate==0.5`. Dimension shorthand builds an exact string filter; metric shorthand builds an integer or finite decimal numeric filter. Ambiguous values, malformed nested expressions, and excessive depth/size are rejected before Google requests. Other shorthand operators and live field eligibility are not implemented.

### 📑 Bounded Admin pagination options

`get_account_summaries`, `list_google_ads_links`, and `list_property_annotations` now expose the pinned upstream optional `page_size` and `max_pages` controls. The implementation bounds page size to 1–200, page count to 1–100, total items to 10,000 and aggregate response to 16 MiB, and rejects reused/oversized pagination tokens. Explicitly stopping at a user-specified page count returns `truncated: true` and a `nextPageToken` rather than claiming the collection is complete. Synthetic HTTP tests validate the transmitted `pageSize` and bounded partial-result contract; authorized live account testing is still required.

See the [Contract V1 compatibility policy](CONTRACT.md) for the current dual structured/text envelope, stable error codes, and outstanding response-parity gates.

Explicit edit-scoped OAuth for optional annotation creation: `synmetra auth login --client-id-file /path/to/desktop-oauth.json --allow-edit`. This requests both `analytics.readonly` and `analytics.edit`, verifies that both scopes were granted, and stores the permission tier privately. Default login remains read-only, and all write tools remain hidden without a separately explicit `synmetra serve --allow-write`. No real Google consent has been verified yet.
