# Synmetra 📐

**Native Rust Google Analytics 4 MCP server for AI agents.** Goal: the **complete feature union** of Google's official Python MCP server and SednaLabs' Rust implementation, with no Python or npm runtime dependency.

![Synmetra project color teal #28BDAD](assets/synmetra-color.svg)

**Project identity:** `#28BDAD` · CSS `oklch(72% 0.12 184)` — [OKLab color rationale](docs/COLOR.md).

> **Status: experimental native Rust MCP stdio server, not production-ready.** 20 read-only tool handlers and one optional annotation write and native OAuth code compile and pass synthetic tests. No authorized live Google account, end-to-end login, or full upstream schema parity has been verified. Authenticated loopback HTTP passed a synthetic initialization test; optional DuckDB scratchpad builds and passes synthetic native tests, but live GA ingestion and full upstream parity remain unverified. The GitHub repository's owner remains its GitHub account holder; ChatGPT is acting as delegated development steward.

## 📊 Evidence-backed progress

**Overall:** `███░░░░░░░ 29.7%` · **overall delta: +29.74 percentage points** from initial 0% (**latest accepted round: +0.67 pp**). **35/35 SednaLabs tool names implemented provisionally plus 1 opt-in annotation handler; 0/36 fully accepted upstream parity.** Real Google account acceptance and full tool schemas remain pending.

| Milestone | Progress | Gate |
| --- | --- | --- |
| 🧭 M0 Reference baseline and project governance | `███████░░░ 67.7%` | MVP |
| 🔌 M1 MCP protocol and transport fidelity | `███░░░░░░░ 32.6%` | MVP |
| 🔑 M2 Google authentication and credential isolation | `██░░░░░░░░ 23.1%` | MVP |
| 🏢 M3 Admin API information parity | `███░░░░░░░ 34.4%` | MVP |
| 📊 M4 Reporting, preflight and Data API parity | `███░░░░░░░ 31.7%` | MVP |
| ✍️ M5 Explicitly gated GA mutation parity | `██████░░░░ 61.5%` | MVP |
| 🗃️ M6 DuckDB scratchpad and advanced analytics parity | `████░░░░░░ 40.0%` | MVP |
| 🧬 M7 Tool contract interoperability and parity audits | `████░░░░░░ 35.3%` | MVP |
| 🧪 M8 Verification, resilience and security acceptance | `█░░░░░░░░░ 12.5%` | MVP |
| 🚀 M9 Linux release and Codex-ready inauguration | `░░░░░░░░░░ 0.0%` | MVP |
| 🔭 M10 Post-MVP extensions and sustained stewardship | `░░░░░░░░░░ 0.0%` | POST_MVP |

The [pinned native input-schema parity audit](docs/SCHEMA_PARITY_AUDIT.md) documents measured argument/schema gaps from an earlier feature-enabled CI run; these gaps are tracked separately from the 36-name inventory and are not yet all resolved.

Detailed milestone, phase, feature and atomic-task progress: [ROADMAP](docs/ROADMAP.md). [GitHub milestone issue queue](docs/ISSUES.md) keeps the 11 tracking issues in sync. Machine-readable evidence and checks: [progress.json](docs/progress.json). Exact combined tool surface: [tool-inventory.json](docs/tool-inventory.json). Source/protocol audit: [UPSTREAM](docs/UPSTREAM.md).

## 🦀 Mission and boundaries

- **Single native Rust executable** `synmetra`, built for Linux / EndeavourOS and Codex first, supporting stdio and secure Streamable HTTP MCP.
- All **36 distinct MCP tools** remain the target: Google's **10** (nine read and one optional gated annotation write), SednaLabs' **35**, **nine shared**. Currently 20 have provisional read-only routes and 1 has a gated mutation route; 7 optional scratchpad lifecycle handlers are implemented provisionally and 8 scratchpad analysis handlers are not implemented. No route has passed full upstream parity/live GA acceptance.
- Comprehensive Google Analytics Admin + Data API read/report parity, alpha conversions and funnels, optional annotation creation.
- Full optional DuckDB scratchpad, safe SQL sandbox, lifecycle/quotas, evidence export and release/landing comparisons.
- Read-only by default, `analytics.readonly` scope, native Desktop OAuth and short-lived bearer-token support. ADC and service-account support remain planned. `analytics.edit` requires explicit opt-in plus externally authorized credentials; inbound HTTP identity and Google identity are kept separate.
- Preserve upstream feature semantics and argument coverage, add no surprise telemetry, protect credentials and user analytics.
- No new runtime dependence on Python, node, npm or gcloud. Embedded DuckDB is a native database dependency and **not pure Rust**; see [architecture](docs/ARCHITECTURE.md).

## 📚 Canonical documents

[🗺️ Roadmap](docs/ROADMAP.md) · [📌 Upstream feature audit](docs/UPSTREAM.md) · [🏗️ Architecture](docs/ARCHITECTURE.md) · [🔐 Security](docs/SECURITY.md) · [🎨 Color](docs/COLOR.md) · [⚖️ Provenance](docs/PROVENANCE.md) · [📋 Governance](docs/GOVERNANCE.md).

**License:** MIT. The Google and SednaLabs reference repositories are Apache-2.0. This project independently implements their documented behavior; copying their code requires preserving their copyright/license notices and tracking provenance.

## 🧪 Native development (experimental)

Build and launch the MCP stdio server with `cargo run --bin synmetra -- serve`. For desktop OAuth, use `synmetra auth login --client-id-file /path/to/desktop-oauth.json`; credentials are isolated to Synmetra and refreshed natively, without Python or npm. See [native MCP status and security caveats](docs/NATIVE_MCP.md). A valid Google Cloud OAuth Desktop client and actual GA4 property permissions are required for live use. Do not expose a remote endpoint or treat this as production-ready software.

**Optional annotation writes:** `synmetra serve --allow-write` exposes `create_property_annotation` only when explicitly requested. The native OAuth flow currently requests read-only access; an authorized `analytics.edit` token and Google property permission are still required. [Write-gating and validation](docs/ANNOTATIONS.md). Remote public deployment is not implemented.

**Optional scratchpad (experimental):** `cargo build --features scratchpad` and `synmetra serve --scratchpad` expose seven provisional in-memory DuckDB lifecycle operations. They are blocked on Streamable HTTP because per-client isolation is unfinished. Feature-enabled native compilation and contract verification remain pending. [Scratchpad limitations](docs/SCRATCHPAD.md) · [Upstream schema gaps](docs/SCHEMA_GAPS.md).

**Native DuckDB feature testing:** The initial seven lifecycle tool implementations passed an optional bundled-DuckDB feature test in [GitHub Actions run 40206168306](https://github.com/sguzman/synmetra/actions/runs/40206168306). Later schema refinements and full strict Clippy/Google integration are not yet accepted. The 7 lifecycle tasks remain partial (50%) rather than falsely marking full upstream parity.

**Latest native scratchpad:** Twelve provisional local DuckDB MCP tools are available only with `--features scratchpad` and `serve --scratchpad`, including historical and realtime GA4 ingestion. Sessions are memory-only and inaccessible through Synmetra's HTTP transport. See [scratchpad policy](docs/SCRATCHPAD.md). No live Google authorization has been tested.

**Tool-name implementation coverage:** 35/35 of the pinned SednaLabs tools now have native Rust handlers. Fifteen DuckDB tools require the optional feature and explicit scratchpad capability. All 36 union tools still require full input/output behavioral acceptance; none is certified 100% upstream parity complete.

For implementation-versus-parity semantics, see the [MCP Contract V1 compatibility policy](docs/CONTRACT.md). It documents dual text/structured results, explicit elapsed-time uncertainty, and intentionally disabled write/scratchpad capabilities.
