# 🧪 Synthetic scratchpad ingestion → query → export evidence

**Scope:** T183 synthetic lifecycle acceptance, **not** a live Google Analytics certification or proof of 36-tool upstream parity. The associated regression is `scratchpad_ga_ingest_round_trips_historical_realtime_and_private_evidence` in `src/main.rs`, executed by `cargo +1.99.1 test --locked --features scratchpad`. No actual Google property, OAuth user or visitor records are used.

## Reproducible fixture

The feature-enabled test creates a local `TcpListener` on ephemeral loopback, responding to the canonical historical `runReport` and realtime `runRealtimeReport` Google Data API routes for synthetic numeric property `42`. Both return one fabricated row with `country` and `activeUsers`; the expected countries are `Mexico` and `Canada`, and synthetic counts are `7` and `3`. The test asserts the outbound request limit and that only a synthetic test bearer is sent upstream. The response server is controlled by the test, **not** by Google.

One trusted in-memory DuckDB session receives the two responses into distinct tables (`historical` and `realtime`). The test checks receipt envelopes and their row counts, summarizes a table, runs the restricted local SELECT on the historical table, and exports a JSON/Markdown evidence bundle for both tables with one sampled row each. The export is bounded by 256 KiB and reports `provider_called=false` because export is a local operation. The emitted receipts, query results, rejection envelopes and export are checked for absence of the synthetic bearer.

A second synthetic session is opened to verify that it cannot read the first session's table. This is **in-process session separation**, not per-human identity isolation or permission isolation under shared HTTP. The test also rejects an oversized realtime ingest request before creating another table, checks that the original two tables remain intact, closes both sessions, and refuses evidence export after closure.

## Evidence and privacy rules

- All inputs are test literals. There are no real account/property IDs, user emails, customer reports, token values or captured Google responses in committed test fixtures.
- The arbitrary synthetic bearer appears in source assertions as a **deliberately fake test marker**, never as a usable Google credential. Public CI prints only synthetic test names/outcomes.
- No private file path is selected for export; the implementation returns bounded structured JSON plus Markdown text in the MCP tool response and does not open a user-controlled export destination.
- The successful native feature-CI run at a particular source SHA is required before T183 can be marked accepted. A successful mock responder is **not** evidence of live Google response fidelity or signed-off customer analytics.
- A real GA lifecycle remains a different gate: T178–T186 cover explicitly authorized OAuth/property/role/endpoint tests and redacted live evidence. T143 and T146 retain separate identity and cleanup gates.

See [SCRATCHPAD.md](SCRATCHPAD.md), [LIVE_ACCEPTANCE_MATRIX.md](LIVE_ACCEPTANCE_MATRIX.md), [SECURITY.md](SECURITY.md) and [progress.json](progress.json).
