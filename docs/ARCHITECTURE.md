# 🏗️ Native Rust architecture and compatibility boundaries

## 🎯 Runtime modules (planned, not implemented)

`synmetra` standalone Rust crate, with cohesive modules rather than a monorepo:

1. **CLI/config:** `synmetra serve --transport stdio|http`, `synmetra auth login|status|doctor|logout`, `synmetra tools`, safe config files and support commands.
2. **MCP:** `rmcp` + Tokio for discovery, per-tool JSON schemas, annotation hints, cancellation, output envelopes, and stdio/Streamable HTTP.
3. **Google HTTP:** `reqwest` + `rustls` + `serde` for Admin v1beta/v1alpha and Data v1beta/v1alpha; a central bounded client handles retry, quota, metadata and pagination. Do not make Google client calls directly from MCP handlers.
4. **Auth:** native OAuth PKCE loopback, headless assisted login, refresh-token lifecycle, explicit isolated ADC, service-account JWT, upstream bearer-per-request; per-request scopes and role checks. Avoid gcloud as an installer/runtime prerequisite.
5. **Contracts:** schema compatibility layer for the 36-name union, provider request parsing and Contract V1 output projections. Shared names must support meaningful inputs from both pinned implementations.
6. **Scratchpad:** embedded DuckDB Rust bindings, per-session limits, read-only SQL policy, disabled extension loading and external data access, metered ingestion and bounded evidence exports. **DuckDB embeds native C/C++ code**; the user-facing server is Rust, not a promise of pure-Rust transitive code.
7. **Safety:** read-only profile by default, opt-in `--allow-write` for annotations, explicit scratchpad profile; config-bound HTTP/host/CIDR/TLS protection. Inbound MCP identity is independent from Google upstream identity.
8. **Tests:** fixture-driven mock Google endpoints, schema snapshots, inspector/Codex integration, native Linux release smoke and optional explicitly authorized live GA property gates.

## 🧭 Proposed module layout

`src/{main,cli,config,mcp,auth,google,admin,reports,contracts,policy,scratchpad,errors}.rs` (modules or subdirectories only when needed), `tests/{contract,google_mock,security,live_ga}.rs`, `docs/progress.json`, and source-pinned schemas in `spec/`. Preserve a single Cargo package while scope remains manageable.

## 🔐 Deployment profiles

| Profile | Google scope | Tools | Inbound exposure |
| --- | --- | --- | --- |
| `read_only` (default) | `analytics.readonly` | 20 GA/auth/preflight read tools (not scratchpad), subject to discovery policy | stdio or protected loopback HTTP |
| `scratchpad` | `analytics.readonly` | Default reads plus 15 local state tools | isolated per-session; prefer local |
| `read_only --allow-write` | explicit `analytics.edit` | Includes `create_property_annotation` | only when operator confirms policy |

**Tool counting:** read-only shared GA/admin/report/preflight helpers total 20: 3 setup + 7 Admin + 8 reports + 2 preflight. 20 + 15 scratchpad + 1 gated write = 36 distinct names. The word `read_only` describes Google mutability; scratchpad still writes locally.

## ⚖️ Acceptance and licensing

The existing Synmetra repository is MIT licensed. Both reference repos are Apache-2.0; behavioral parity is an independent Rust implementation with clearly attributed ideas. Vendored or adapted code, if any, requires SPDX/copyright notices and an inventory. No trademark affiliation with Google. Never store Google tokens, real GA property IDs, private logs or user conversation fragments in public source, issues or CI artifacts.
