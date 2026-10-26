# 🔁 Client compatibility contract and migration adapters

**Scope:** native Synmetra `0.0.1`, the pinned 36-tool union, and the source-reviewed response adapters. This is a **behavioral compatibility policy**, not a claim of executable SednaLabs parity, real Google authorization, or a stable `1.0` contract. [The JSON policy](../spec/compatibility/client-contract.v1.json) is validated in CI against the pinned tool inventory and the published native schema.

## Which MCP profile does a client actually see?

| Profile | Tool names | Activation | Guarantees and exclusions |
| --- | ---: | --- | --- |
| Read-only local stdio | 20 | `synmetra serve` | Google read/guidance tools; no annotation mutation and no DuckDB |
| Explicit write-capable stdio | 21 | `serve --allow-write` | Annotation *advertised*, not automatically authorized: separate Google edit scope, property role and client confirmation are required |
| Optional local scratchpad stdio | 35 | feature-enabled binary with `serve --scratchpad` | Adds 15 local DuckDB tools; only one trusted local stdio caller |
| Fully opt-in source test profile | 36 | both flags with a feature-enabled binary | Registry/parity fixture profile, **not** a default or publicly hosted service |

An inbound loopback HTTP bearer is **not** a Google OAuth access token. `--scratchpad --http-loopback` is intentionally refused until identity-bound sessions can be verified. These profiles must not be conflated with permission on a real GA4 account.

## Response compatibility decisions

| Client assumption | Current native behavior | Adaptation needed |
| --- | --- | --- |
| JSON-only MCP text clients | Every handled response supplies a JSON text content block | Parse the envelope; do not assume the text is raw Google JSON |
| Structured MCP clients | The same envelope is sent as `structuredContent` | Prefer the structured representation, but still inspect `ok` and `error` |
| Successful tool calls | `{ok:true,data,meta}` | Read `data` and preserve `meta`, including unknown/partial row totals |
| Tool-level failures | `{ok:false,error:{code,reason,message,category, status_code?},meta}` | Treat provider HTTP status as optional; do not confuse this with JSON-RPC transport errors |
| Google's ordinary read response | Provider-shaped response is nested under `data` by default | Unwrap `data` rather than assuming direct raw Google API JSON |
| SednaLabs-style tabular output | Explicitly opt in to supported `output_mode` (`rows`, `tuples`, `scalar`, `compact`), `summary_only`, `max_rows`, `max_cell_chars` | Check *each tool's advertised input schema* before supplying controls; not every route accepts every mode |
| Historical report pagination | Bounded `v1` cursor binds query and normalized property identity | Do not reuse across changed queries; Google data may change between pages |
| Realtime pagination | Cursor and positive offset deliberately rejected | Issue a new bounded realtime query; never pretend a snapshot cursor exists |
| Upstream elapsed duration | `elapsed_ms:0` with `elapsed_ms_measured:false` | Never treat it as measured zero latency |

### Error, policy and protocol boundaries

A rejected MCP method or malformed JSON-RPC frame is a **protocol error**; a valid tool call returning `ok:false` is an **application-level tool error**. A Google `401` or `403` is preserved as an upstream HTTP error (including `status_code`); Synmetra does not silently request `analytics.edit`, impersonate another user, or retry a mutation to bypass denial. A local DuckDB error is categorized separately from Google errors.

Source tests exercise all native `ApiError` variants, structured/text envelope equality, and redacted stable categories. The pinned SednaLabs source's `POLICY_DENIED` and timing/compact payload behavior still have documented differences in [RESPONSE_DIFFERENTIAL.md](RESPONSE_DIFFERENTIAL.md); matching a few shared fields does **not** establish full parity.

## Legacy client and version migration

Keep existing Google/SednaLabs MCP registrations in place while introducing Synmetra under a **distinct** tool-server name. Compare the actual `tools/list` schema with [NATIVE_TOOL_GUIDE.md](NATIVE_TOOL_GUIDE.md) and [the recorded native schema](../spec/native/tool-schema.v1.json). In particular, exact upstream input **names** are covered, but the raw pinned audit still records 201 shape differences across shared fields, and some scratchpad ingest output controls are deliberately rejected rather than silently dropped.

Preserve previously cached client output parsers until a synthetic side-by-side test validates the expected envelope, nested data, status, row metadata, and selected output mode. Do not reinterpret a provider-shaped report as automatically equivalent to SednaLabs compact Contract V1. The [migration guide](MIGRATION.md) and [canonical union decision](CANONICAL_CONTRACT_DECISION.md) enumerate remaining incompatibilities.

The current `0.0.1` line is **experimental**. Before `1.0`, any incompatible name, input, output, authentication or cursor change requires an explicit migration note and reviewed pinned schema update. Versioned client adapters may be extended, but silently deleting an upstream field or claiming untested provider behavior as equivalent is disallowed. Stable `v1.0.0` remains blocked until the independent acceptance gates in [ROADMAP.md](ROADMAP.md) pass.

## Verification and scope limits

Core and feature tests already verify native response envelopes, read-only defaults, explicit write/scratchpad gating, bounded projections and error categories. The compatibility policy's machine-readable fixture is checked by CI for the four profile counts and expected response/version rules. **Not accepted** merely from this documentation: pinned upstream *executable* output equivalence, live Google property-role and alpha API testing, Inspector/third-party client behavior, or hosted multi-user isolation. The respective gates remain T156–T165, T171–T172, T178–T186 and M9.
