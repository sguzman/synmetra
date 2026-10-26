# 🧱 Canonical union contract and compatibility policy (T164)

**Decision status:** source-backed native-contract baseline; **not** a declaration of upstream behavioral parity, Google account authorization, or production readiness. Every public MCP input field from the pinned SednaLabs and official Google signatures is represented in the current **fully opt-in, feature-enabled native router**, but several accepted input shapes still have intentionally unsupported semantics or response differences.

## Sources and contract choice

The canonical **name and input parameter set** is the union of Google's ten pinned official function signatures (`ce84226b`) and SednaLabs' 35 pinned MCP JSON Schemas (`039fd462`), comprising 36 unique tools. Prefer the SednaLabs MCP JSON Schema field names, required sets, and explicitly documented defaults for shared tools; preserve Google-only inputs and annotations, including the explicitly gated `create_property_annotation`. Never silently remove a field to make a smaller implementation appear compatible. The default runtime still advertises only the permitted 20 read-only tools; opt-in scratchpad adds 15 and opt-in write adds one. The full 36-name feature profile is a test-only union, not the default.

The native schema source of truth is [spec/native/tool-schema.v1.json](../spec/native/tool-schema.v1.json), captured/checked by the Rust `full_capability_profile_matches_exact_pinned_36_tool_union` test. A separate [machine-readable native-versus-upstream diff](../spec/audits/native-upstream-diff.v1.json) records each raw property difference and the exact source blob IDs. Its current *top-level* result:

| Comparison | Current result |
| --- | ---: |
| Native MCP tools | 36 |
| Pinned SednaLabs tools | 35 |
| Pinned Google function signature names | 10 |
| Missing SednaLabs top-level input names | **0** |
| Missing official Google signature parameter names | **0** |
| Required-field set mismatches against SednaLabs | **0** |
| Native-only extra input names | 2 (`keep_empty_rows` on `run_report` and `preview_report_request`) |
| Raw field-shape differences across shared fields | **202** |

The raw differences include **182 description differences**, 19 differently named `$ref` fields, 18 `anyOf` differences, 10 `items` differences, 9 default differences, 8 type differences. The two previously differing `uint32` versus `uint64` ingest-limit formats now match upstream. These categories overlap, so totals must not be summed as distinct fields. Raw JSON Schema comparison does **not dereference or normalize** nested definitions. An input `$ref` rename can be semantically equivalent; a default/type change can also be meaningful. Neither case can be called accepted without its own tests and decision.

## Behavioral precedence and known incompatibilities

1. **MCP transport / errors:** preserve JSON-RPC protocol errors independently from tool-level `ok/data/meta` and `ok/error/meta` envelopes. Native responses provide both structured JSON and text content. `meta.elapsed_ms=0` is an explicit *unmeasured* sentinel, never a promise of instantaneous execution.
2. **Google provider boundary:** accept pinned snake_case input names, normalize Google camelCase requests only within the fixed Google API boundary, and reject arbitrary resource identifiers/URLs before outgoing requests. A valid input schema is not permission to a real property or v1alpha endpoint.
3. **Pagination:** historical report cursor tokens are query/resource-bound and bounded, but are not snapshots of Google data. Realtime has no provider cursor/offset semantics; a positive offset or nonempty cursor fails explicitly.
4. **Scratchpad metadata and diagnostics:** local DuckDB table metadata projection, release diagnostics, and landing parameter shift reports support bounded output controls without altering default no-control response shapes. Local sessions are in-memory and stdio-only, not per-HTTP-principal.
5. **Scratchpad ingestion remains one page:** optional `max_rows` and `max_cell_chars` are bounded (1–1000, 1–2048). Native ingestion accepts upstream-width `u64` `limit` inputs but rejects values above 1,000 **locally before sending Google requests**. Nonempty `cursor`, `summary_only=true`, and non-`rows` `output_mode` are deliberately **rejected**, not ignored. If requested `limit` exceeds that one-page maximum, an explicit `pagination.complete=false` warning is returned. Multi-page GA ingestion, true cursor parity, richer modes and independent authorized-property checks remain unaccepted.
6. **Writes and secrets:** annotation creation is hidden by default and requires independent editing scope and property role, explicit server `--allow-write`, and client confirmation. Only synthetic fixtures are in public CI; never include real Google tokens/analytics in contract evidence.

## Versioning and acceptance

This baseline is **experimental 0.0.1**. Treat new required fields, removed fields, altered defaults, changed output envelope, or silent acceptance of currently rejected controls as incompatible until a reviewed version decision and tests exist. Optional additional fields may be backward compatible only if old synthetic client behavior remains unchanged. Keep raw upstream snapshots immutable; edit the native adapter, not source fixtures, to repair parity.

The first T164 gate is a concrete, reviewed canonical policy and schema-diff artifact. **The second T164 acceptance gate and T165 remain open** until all documented input *behaviors*, nested type/default semantics and response projections have independently verified compatibility, not just field-name presence. Live Google, release portability, DuckDB isolation and external MCP client QA retain their own gates.
