# 🌐 Local Streamable HTTP (experimental)

Synmetra supports optional Streamable HTTP at **127.0.0.1 only**, with the official Rust rmcp SDK's service and session manager. This does **not** make it a secure production-ready remote service.

Set an independent, randomly generated inbound MCP bearer token in the private environment (at least 32 ASCII nonspace characters) as `SYNMETRA_MCP_BEARER_TOKEN`; then run `synmetra serve --http-loopback`. Default port is 44128; `SYNMETRA_MCP_PORT` can select another nonzero port. Endpoint is `http://127.0.0.1:44128/mcp` by default. This bearer token authenticates MCP callers. It is separate from Google Analytics OAuth credentials, which remain read-only and isolated.

The HTTP middleware rejects missing/incorrect Authorization headers, non-loopback Host headers and foreign Origin headers; the server binds only to 127.0.0.1. Remote binding, TLS termination, multiuser OAuth, local transport rate limiting, and session isolation across different Google identities are **not implemented** and cannot be inferred from this local test. Do not expose this port outside the workstation, forward it publicly, or share its bearer token.

The stdio transport continues to work normally with `synmetra serve`. For local Codex, stdio remains preferred, avoiding the extra inbound token entirely.

## Session compatibility and loopback binding

For MCP clients negotiating a legacy protocol before `2026-07-28`, Synmetra now uses rmcp's local session manager. An authenticated initialization returns `Mcp-Session-Id`; the caller must include that ID (and the same inbound bearer) on its subsequent `/mcp` requests. An authenticated HTTP `DELETE` terminates a session and its former ID cannot resume it. Missing or incorrect bearer tokens and mismatched Origin/Host headers are rejected before reaching the MCP service. The current `2026-07-28` protocol is stateless by its transport rules and does not issue legacy session headers; absence of a header there is expected.

The binary's production bind helper constructs only `127.0.0.1:<port>`. There is no public host override; attempts to pass `--host 0.0.0.0` or similar unsupported flags fail at CLI parsing. This is still an experimental **single-principal** local service: session IDs do not establish isolation between multiple Google identities, and session quotas, lifetime limits, rate limiting, and remote access remain outside the verified scope.
