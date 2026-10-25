# 🦀 Native MCP implementation status

The executable `synmetra` now boots an experimental **read-only** MCP stdio server using the official Rust `rmcp` SDK. Seven tool names are currently advertised because they have concrete handlers: `get_account_summaries`, `get_property_details`, `list_google_ads_links`, `list_property_annotations`, `get_custom_dimensions_and_metrics`, `run_report`, `run_realtime_report`.

**Not feature parity.** These handlers are an initial slice: they still need upstream parameter-level and output contract comparisons, Google ADC/OAuth login, authorized live GA testing, complete filtering and pagination cases, HTTP transport, and scratchpad. The 36-name ledger remains a *target* inventory, not an advertised set of working tools.

Build and run: `cargo run --bin synmetra -- serve`. For a short-lived local development token, set `SYNMETRA_GOOGLE_ACCESS_TOKEN` to a Google OAuth access token granted `analytics.readonly` (the process must be restarted on expiry). Tokens are neither printed nor persisted. Do not put the token in command-line arguments, shell history, source, CI secrets, or public issue logs. Without a token, the server starts but tools return a structured missing-credentials error.

MCP uses stdout exclusively. No credentials, real GA property IDs or private report bodies are used in automated CI; tests use synthetic fixtures. Network calls target fixed Google API hosts with rustls, bounded timeouts, 4 MiB response guard and no redirects. No Python or npm required by the binary. For real integrations, native OAuth remains outstanding.
