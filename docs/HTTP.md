# 🌐 Local Streamable HTTP (experimental)

Synmetra supports optional Streamable HTTP at **127.0.0.1 only**, with the official Rust rmcp SDK's service and session manager. This does **not** make it a secure production-ready remote service.

Set an independent, randomly generated inbound MCP bearer token in the private environment (at least 32 ASCII nonspace characters) as `SYNMETRA_MCP_BEARER_TOKEN`; then run `synmetra serve --http-loopback`. Default port is 44128; `SYNMETRA_MCP_PORT` can select another nonzero port. Endpoint is `http://127.0.0.1:44128/mcp` by default. This bearer token authenticates MCP callers. It is separate from Google Analytics OAuth credentials, which remain read-only and isolated.

The HTTP middleware rejects missing/incorrect Authorization headers, non-loopback Host headers and foreign Origin headers; the server binds only to 127.0.0.1. Remote binding, TLS termination, multiuser OAuth, local transport rate limiting, and session isolation across different Google identities are **not implemented** and cannot be inferred from this local test. Do not expose this port outside the workstation, forward it publicly, or share its bearer token.

The stdio transport continues to work normally with `synmetra serve`. For local Codex, stdio remains preferred, avoiding the extra inbound token entirely.
