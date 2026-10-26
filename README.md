# Synmetra 📐

**Native Rust Google Analytics 4 MCP server for AI agents.** Goal: the **complete feature union** of Google's official Python MCP server and SednaLabs' Rust implementation, with no Python or npm runtime dependency.

![Synmetra project color teal #28BDAD](assets/synmetra-color.svg)

**Project identity:** `#28BDAD` · CSS `oklch(72% 0.12 184)` — [OKLab color rationale](docs/COLOR.md).

> **Status: experimental native Rust MCP stdio server, not production-ready.** 20 read-only tool handlers and one optional annotation write and native OAuth code compile and pass synthetic tests. No authorized live Google account, end-to-end login, or full upstream schema parity has been verified. Authenticated loopback HTTP passed a synthetic initialization test; optional DuckDB scratchpad builds and passes synthetic native tests, but live GA ingestion and full upstream parity remain unverified. The GitHub repository's owner remains its GitHub account holder; ChatGPT is acting as delegated development steward.

## 📊 Evidence-backed progress

**Overall:** `███████░░░ 73.6%` · **overall delta: +73.57 percentage points** from initial 0% (**latest accepted round: +0.22 pp**). **35/35 SednaLabs tool names implemented provisionally plus 1 opt-in annotation handler; 0/36 fully accepted upstream parity.** Real Google account acceptance and full tool schemas remain pending.

| Milestone | Progress | Gate |
| --- | --- | --- |
| 🧭 M0 Reference baseline and project governance | `██████████ 97.1%` | MVP |
| 🔌 M1 MCP protocol and transport fidelity | `████████░░ 80.4%` | MVP |
| 🔑 M2 Google authentication and credential isolation | `█████░░░░░ 53.8%` | MVP |
| 🏢 M3 Admin API information parity | `██████████ 100.0%` | MVP |
| 📊 M4 Reporting, preflight and Data API parity | `██████████ 100.0%` | MVP |
| ✍️ M5 Explicitly gated GA mutation parity | `████████░░ 84.6%` | MVP |
| 🗃️ M6 DuckDB scratchpad and advanced analytics parity | `████████░░ 78.0%` | MVP |
| 🧬 M7 Tool contract interoperability and parity audits | `███████░░░ 73.5%` | MVP |
| 🧪 M8 Verification, resilience and security acceptance | `█████░░░░░ 51.8%` | MVP |
| 🚀 M9 Linux release and Codex-ready inauguration | `███████░░░ 65.9%` | MVP |
| 🔭 M10 Post-MVP extensions and sustained stewardship | `░░░░░░░░░░ 0.0%` | POST_MVP |

The [pinned upstream parameter-union audit](docs/UPSTREAM_PARAMETER_AUDIT.md) preserves all source fields and their known compatibility limits. The [pinned native input-schema parity audit](docs/SCHEMA_PARITY_AUDIT.md) documents measured argument/schema gaps from an earlier feature-enabled CI run; these gaps are tracked separately from the 36-name inventory and are not yet all resolved.

Detailed milestone, phase, feature and atomic-task progress: [ROADMAP](docs/ROADMAP.md). [GitHub milestone issue queue](docs/ISSUES.md) keeps the 11 tracking issues in sync. Machine-readable evidence and checks: [progress.json](docs/progress.json). Exact combined tool surface: [tool-inventory.json](docs/tool-inventory.json). Source/protocol audit: [UPSTREAM](docs/UPSTREAM.md).

## 🦀 Mission and boundaries

- **Single native Rust executable** `synmetra`, built for Linux / EndeavourOS and Codex first, supporting stdio and secure Streamable HTTP MCP.
- All **36 distinct MCP tools** remain the target: Google's **10** (nine read and one optional gated annotation write), SednaLabs' **35**, **nine shared**. Currently 20 default read-only tool names, 15 opt-in scratchpad tool names and 1 explicitly gated annotation mutation tool have provisional native Rust handlers. Each remains subject to parameter/response parity and authorized live acceptance. No route has passed full upstream parity/live GA acceptance.
- Comprehensive Google Analytics Admin + Data API read/report parity, alpha conversions and funnels, optional annotation creation.
- Full optional DuckDB scratchpad, safe SQL sandbox, lifecycle/quotas, evidence export and release/landing comparisons.
- Read-only by default, `analytics.readonly` scope, native Desktop OAuth and short-lived bearer-token support. ADC and service-account support remain planned. `analytics.edit` requires explicit opt-in plus externally authorized credentials; inbound HTTP identity and Google identity are kept separate.
- Preserve upstream feature semantics and argument coverage, add no surprise telemetry, protect credentials and user analytics.
- No new runtime dependence on Python, node, npm or gcloud. Embedded DuckDB is a native database dependency and **not pure Rust**; see [architecture](docs/ARCHITECTURE.md).

## 📚 Canonical documents

[🦀 Linux installation](docs/INSTALL.md) · [⌨️ CLI and Fish](docs/CLI.md) · [🔑 Cloud setup](docs/GOOGLE_CLOUD.md) · [🧪 Smoke QA](docs/SMOKE_SCENARIOS.md) · [⏱️ Offline performance](docs/BENCHMARK.md) · [📚 36-tool native guide](docs/NATIVE_TOOL_GUIDE.md) · [🔬 Source differential](docs/RESPONSE_DIFFERENTIAL.md) · [🧱 Canonical union policy](docs/CANONICAL_CONTRACT_DECISION.md) · [🧪 Payload differential](docs/PAYLOAD_DIFFERENTIAL.md) · [🤖 Codex registration](docs/CODEX.md) · [🔌 MCP clients](docs/MCP_CLIENTS.md) · [🔀 Upstream migration](docs/MIGRATION.md) · [📦 Release evidence](docs/RELEASE_POLICY.md) · [🗺️ Roadmap](docs/ROADMAP.md) · [📌 Upstream feature audit](docs/UPSTREAM.md) · [🏗️ Architecture](docs/ARCHITECTURE.md) · [🔐 Security](docs/SECURITY.md) · [🧷 Dependency audit](docs/DEPENDENCY_AUDIT.md) · [📋 SBOM inventory](docs/SBOM.md) · [🎨 Color](docs/COLOR.md) · [⚖️ Provenance](docs/PROVENANCE.md) · [📋 Governance](docs/GOVERNANCE.md).

**License:** MIT. The Google and SednaLabs reference repositories are Apache-2.0. This project independently implements their documented behavior; copying their code requires preserving their copyright/license notices and tracking provenance.

## 🧪 Native development (experimental)

Build and launch the MCP stdio server with `cargo run --bin synmetra -- serve`. For desktop OAuth, use `synmetra auth login --client-id-file /path/to/desktop-oauth.json`; credentials are isolated to Synmetra and refreshed natively, without Python or npm. See [native MCP status and security caveats](docs/NATIVE_MCP.md). A valid Google Cloud OAuth Desktop client and actual GA4 property permissions are required for live use. Do not expose a remote endpoint or treat this as production-ready software.

**Optional annotation writes:** `synmetra serve --allow-write` exposes `create_property_annotation` only when explicitly requested. The native OAuth flow currently requests read-only access; an authorized `analytics.edit` token and Google property permission are still required. [Write-gating and validation](docs/ANNOTATIONS.md). Remote public deployment is not implemented.

**Native Linux install:** [EndeavourOS/Arch and other Linux source installation, verification, upgrade and uninstall](docs/INSTALL.md). The default binary is read-only and does not require Python, Node, npm or gcloud. Release packaging and authorized live GA testing remain separate gates.

**Optional scratchpad (experimental):** `cargo build --features scratchpad` and `synmetra serve --scratchpad` expose 15 provisional in-memory DuckDB tools, including lifecycle, ingestion and analysis. They are blocked on Streamable HTTP because per-client identity isolation is unfinished. Feature-enabled native tests have run; full security, upstream parity, and live Google acceptance remain pending. [Scratchpad limitations](docs/SCRATCHPAD.md) · [Upstream schema gaps](docs/SCHEMA_GAPS.md).

**Native DuckDB feature testing:** Feature-enabled bundled-DuckDB tests passed in [GitHub Actions run 40269115096](https://github.com/sguzman/synmetra/actions/runs/40269115096) (the full workflow's core formatting gate failed on this SHA and was repaired separately). Strict feature Clippy is configured as a release-tag gate, not a default-branch certification. A passed feature fixture test is not full upstream contract or live GA parity.

**Latest native scratchpad:** Fifteen provisional local DuckDB MCP tools are available only with `--features scratchpad` and `serve --scratchpad`, including historical and realtime GA4 ingestion. Sessions are memory-only and inaccessible through Synmetra's HTTP transport. See [scratchpad policy](docs/SCRATCHPAD.md). No live Google authorization has been tested.

**Tool-name implementation coverage:** 35/35 of the pinned SednaLabs tools now have native Rust handlers. Fifteen DuckDB tools require the optional feature and explicit scratchpad capability. All 36 union tools still require full input/output behavioral acceptance; none is certified 100% upstream parity complete.

For implementation-versus-parity semantics, see the [MCP Contract V1 compatibility policy](docs/CONTRACT.md). It documents dual text/structured results, explicit elapsed-time uncertainty, and intentionally disabled write/scratchpad capabilities.
