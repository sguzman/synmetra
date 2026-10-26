# 🧳 DuckDB evidence export: permission and disclosure boundary

**Status:** local-only, bounded in-memory output; not a disk export API, not an automatic data-redaction or client-side permissions system. This is security evidence for the implementation portion of T148, not full acceptance of its "no secrets" guarantee.

## Trust boundary

`scratchpad_export_evidence_bundle` is exposed only by a binary built with the optional `scratchpad` feature and launched as `synmetra serve --scratchpad`. Synmetra refuses to combine that feature with the loopback HTTP endpoint while per-client identity isolation is incomplete. A session ID is a selector, **not an authorization token**: only a single trusted stdio client/OS user is supported. Anyone who already controls that client may export the contents of its accessible local scratchpad tables. No other-user access claims are made.

The MCP input has `session_id`, optional `table_names` and optional `sample_rows_per_table` only. There is **no** arbitrary output-file or directory argument. The server returns JSON plus a Markdown string in the MCP response rather than writing a file. If an agent copies that response into a document or external tool, the receiving application owns its file permissions, retention and onward disclosure rules. No hidden "save evidence" path or private upload occurs in Synmetra.

## Implemented controls and verified synthetic tests

- `src/scratchpad.rs::session_id` requires 1–128 ASCII letters/digits/underscores/hyphens. `table_name` requires a validated 1–64-character SQL identifier. This rejects paths, URLs, punctuation, newlines, and apparent SQL injection fragments before any export query is constructed.
- `export_evidence_bundle` validates selected names against the current local DuckDB catalog, rejects duplicates, permits at most eight tables, samples at most twenty rows per table, clips serialized cell text, and rejects bundles above the 256 KiB aggregate response budget.
- The sampling query quotes catalog-derived column identifiers, binds table names in catalog checks, orders results deterministically, hashes the effective sample query and **does not use DuckDB file COPY/ATTACH/readers or extensions**.
- `evidence_export_rejects_path_inputs_without_touching_neighbor_files` constructs a neighboring private file with a synthetic sentinel and confirms numerous hostile table/session arguments are rejected without modifying that file. It checks the successful response has no `output_path` and does not call Google.
- `exports_private_bounded_json_and_markdown_evidence`, `evidence_samples_are_sorted_repeatable_and_hash_the_actual_query`, `aggregate_evidence_bundle_rejects_oversized_output_without_altering_tables`, and `scratchpad_evidence_export_matches_pinned_inputs_and_bounded_json_markdown` check the bounded structured/Markdown results and database preservation.
- Both core and native DuckDB jobs passed on [main CI run 40374986333](https://github.com/sguzman/synmetra/actions/runs/40374986333). These are synthetic local tests, **not** independent audit proof against all DuckDB/native vulnerabilities.

## What is *not* guaranteed

An analytics column can itself contain an access token, email, URL query secret, or other private value **if the caller placed it in the scratchpad**. The exporter currently does not classify or redact arbitrary row values automatically; selected sample rows are returned to the requesting trusted client. For sensitive analytics, export only authorized tables, consider `sample_rows_per_table=0` for counts without examples, and avoid forwarding MCP responses into public prompts, logs, CI artifacts or Git commits.

The current output limit and absence of disk-writing parameters prevent arbitrary **server file overwrites**, but cannot prevent the **client** from saving a response insecurely. Future per-principal session isolation, export authorization, redaction semantics and crash/temp-file cleanup require separate acceptance (T143, T148, T146). A passing path-fixture does not close T148's second permission/secret-disclosure gate.

See [SCRATCHPAD.md](SCRATCHPAD.md), [LOCAL_EXECUTION_THREAT_MODEL.md](LOCAL_EXECUTION_THREAT_MODEL.md), [SECURITY.md](SECURITY.md), and [progress.json](progress.json).
