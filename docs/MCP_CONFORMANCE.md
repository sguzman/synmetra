# 🔌 Native MCP cross-client conformance evidence

**Status:** passing offline Linux synthetic interop at source [f897156b7913ec04ef459c15772c2c2c47bec5ca](https://github.com/sguzman/synmetra/commit/f897156b7913ec04ef459c15772c2c2c47bec5ca); [core and DuckDB CI 40415340670](https://github.com/sguzman/synmetra/actions/runs/40415340670). This is scoped **local MCP stdio conformance**, not an Inspector, Codex, Claude Desktop, public HTTP, or live Google Analytics certification. See T033, T161, T172 and T185 for those separate acceptance gates.

## Client implementations under test

| Client | Implementation | Protocol negotiation | Discovery and call evidence |
| --- | --- | --- | --- |
| Rust MCP SDK client | `rmcp` `().serve(client_stream)` over an independent Tokio in-memory duplex stream | SDK's negotiated version | `in_memory_mcp_client_negotiates_and_calls_read_only_tools` initializes, lists 20 default read-only tools, verifies gated mutation absence, and checks an authorization-free error |
| Raw client A | Independently authored JSON-RPC newline frames in [ci/test-mcp-clients.sh](../ci/test-mcp-clients.sh), fed to actual installed `synmetra serve` | `2025-06-18` | Verifies native initialize response, 20 tool JSON schemas, `ga4_get_started` success envelope and `get_account_summaries` structured typed error |
| Raw client B | Same raw framing with separate client metadata, fresh process and isolated XDG config | `2025-11-25` | Repeats initialization, list, success and error assertions against the native executable |

The raw clients and Rust SDK exercise two **different client implementations**, while the raw profiles exercise two negotiated protocol versions. Each raw client creates a new process and private config directory, never contacts Google, clears ambient Google and MCP bearer overrides, uses `timeout`, checks JSON-RPC IDs/response counts, and requires no npm, Python, Node or `gcloud`. The client fixtures do **not** pretend a failed, uncredentialed Google request is an authorized success.

## Full 36-tool synthetic invocation pass (not an Inspector)

The optional bundled-DuckDB feature test `all_36_registered_mcp_tools_accept_synthetic_fixture_invocations` in `src/main.rs` loads [36 named JSON argument fixtures](../spec/fixtures/native-all-tool-smoke.v1.json), compares them with every advertised tool returned from a real `rmcp` client session, and invokes **all 36 routes** through the client, with an eight-second deadline per call. It verifies a typed structured `ok/data/meta` or `ok/error/meta` outcome, MCP tool-error status where appropriate, and absence of its synthetic Google bearer from response bodies.

A local, ephemeral `127.0.0.1` fake Google endpoint deliberately returns HTTP 403 rather than using a live account, while DuckDB tools run only in process-local scratchpad memory. The full feature-enabled test passed [DuckDB CI 40415340670](https://github.com/sguzman/synmetra/actions/runs/40415340670). This satisfies the **synthetic all-tool invocation** criterion of T172 but **not** its external MCP Inspector criterion: neither the official Inspector nor an authorized Google provider was invoked. That independent Inspector gate remains open.

## Contract checks and exclusions

- The initialize response must identify `synmetra` and advertise a tools capability; tool listing must expose exactly 20 default read-only tools and no annotation write.
- A local-only guidance tool must return `isError=false` and an `ok=true` structured data envelope.
- A Google account-read request with deliberately absent credentials must produce a properly flagged MCP tool error with structured `error.code` and `error.category`; protocol transport itself stays responsive.
- The wire output and diagnostics are checked for common reflected secret-field shapes. This is a targeted synthetic redaction check, **not** a historical Git secret scan or proof against all credential leakage.
- This suite is **not** an external Inspector run, an authorized GA4 report, a live Chrome/agent integration, a streaming HTTP validation or a comprehensive compliance claim covering every MCP method.

The test script runs in [the default core GitHub Actions workflow](../.github/workflows/ci.yml) after the installed native binary is built, while the independent Rust SDK client runs inside `cargo test --all-targets --locked`. Both steps passed the linked source SHA's core job. A future change to tool schemas, transport or host permissions must rerun both implementations before the conformance gate can be reaffirmed.
