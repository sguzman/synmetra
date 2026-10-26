# 🔌 Other MCP agents: supported transport boundaries

Synmetra's MCP server is a native Rust executable with two experimental transports. The supported default for a local agent is **stdio**; optional Streamable HTTP is strictly loopback-only and authenticated. No browser extension, Python, npm, Node or public SaaS relay is required at runtime. All registered tools remain subject to the [36-tool native schema guide](NATIVE_TOOL_GUIDE.md) and [live-account acceptance limits](LIVE_ACCEPTANCE_MATRIX.md).

## Stdio (recommended for trusted local agents)

Configure the client to launch the absolute installed binary with executable `/home/YOUR_LOCAL_USER/.local/bin/synmetra` and argument `serve`. The literal path is a placeholder, **not** a user-specific path discovered by Synmetra. A compatible MCP client must send JSON-RPC initialization and `tools/list` over the child process's stdin/stdout. Stdout is reserved for protocol JSON; diagnostic messages use stderr. This child process is tied to one trusted local user and the identity available in that environment.

The default profile advertises 20 read-only tools, no mutation. With feature-enabled binary and explicit `--scratchpad`, 15 more local DuckDB tools are visible; that profile must not be exposed to multi-client HTTP. With explicit `--allow-write`, the annotation mutation name is advertised; actual Google API write requires edit-scoped consent/property role and client approval.

Avoid printing a Google bearer in client configurations. Prefer the private native Desktop OAuth file under XDG. The short-lived `SYNMETRA_GOOGLE_ACCESS_TOKEN` override exists for controlled local experiments but is not auto-refreshed; do not place it into tracked project JSON/TOML or shell history.

## Loopback Streamable HTTP (experimental)

Only a caller on the same trusted Linux host may connect to `http://127.0.0.1:44128/mcp` (or the explicitly selected nonzero `SYNMETRA_MCP_PORT`). The server is started with `synmetra serve --http-loopback` and rejects startup without `SYNMETRA_MCP_BEARER_TOKEN` set to at least 32 nonspace printable ASCII characters. Configure the client to send `Authorization: Bearer` with that independent inbound secret, and use a Host/Origin matching the bound loopback origin.

This inbound bearer is **not** the Google OAuth token. The server validates Host/Origin, request body/concurrency/time caps, and disallows arbitrary nonloopback binding. This is not a multi-tenant identity service, a TLS listener, or a safe reverse-proxy/public-port endpoint. Scratchpad plus HTTP is explicitly rejected until per-principal state is implemented and tested.

## Protocol and conformance acceptance

CI exercises native MCP stdio initialize, list and call, legacy/current HTTP initialization, bearer/Origin rejection and transport error framing using synthetic data. These fixtures do **not** certify Inspector, Claude Desktop or third-party agent compatibility; those require actual client integrations under T161/T172/T185 and cannot be credited from copied example configurations.

For tests, compare the client's advertised tool names with [tool-inventory.json](tool-inventory.json), and confirm that tool input schemas match the pinned [native snapshot](../spec/native/tool-schema.v1.json) for the selected profile. Do not claim provider or upstream payload parity from tool-name equality.

### Cancellation and remote side effects

A client may send MCP `notifications/cancelled` with the in-flight JSON-RPC request ID. Synmetra routes cancellation through the SDK request token and centrally drops the running tool future, so a stalled Google HTTP operation does not continue consuming the process after its caller abandons the request. The synthetic stdio regression sends a canceled historical report to a deliberately stalled local mock endpoint and verifies the upstream socket closes, the canceled response stays suppressed, and the same client can call a subsequent tool.

**Cancellation is not transactional rollback.** A Google annotation may already have been processed before its request is canceled; neither Synmetra nor the MCP protocol can undo that Google-side effect. Synmetra never automatically replays writes after uncertain outcomes. A DuckDB operation may have committed before the cancellation signal arrived; check session state rather than assuming an interrupted tool was undone. Cross-identity HTTP cancellation and client interoperability remain independently scoped acceptance tasks. See [the security threat model](LOCAL_EXECUTION_THREAT_MODEL.md) and [ROADMAP.md](ROADMAP.md).
