# Synmetra 📐

**Native Rust Google Analytics 4 MCP server for AI agents.** Goal: the **complete feature union** of Google's official Python MCP server and SednaLabs' Rust implementation, with no Python or npm runtime dependency.

![Synmetra project color teal #28BDAD](assets/synmetra-color.svg)

**Project identity:** `#28BDAD` · CSS `oklch(72% 0.12 184)` — [OKLab color rationale](docs/COLOR.md).

> **Status: experimental native Rust MCP stdio server, not production-ready.** 17 read-only tool handlers and native OAuth code compile and pass synthetic tests. No authorized live Google account, end-to-end login, full upstream schema parity, Streamable HTTP or DuckDB scratchpad has been verified. The GitHub repository's owner remains its GitHub account holder; ChatGPT is acting as delegated development steward.

## 📊 Evidence-backed progress

**Overall:** `█░░░░░░░░░ 11.4%` · **overall delta: +11.45 percentage points** from initial 0% (**this implementation round: +8.59 pp**). **17/36 provisional handlers; 0/36 fully accepted upstream parity.** Real Google account acceptance and full tool schemas remain pending.

| Milestone | Progress | Gate |
| --- | --- | --- |
| 🧭 M0 Reference baseline and project governance | `█████░░░░░ 52.9%` | MVP |
| 🔌 M1 MCP protocol and transport fidelity | `█░░░░░░░░░ 10.9%` | MVP |
| 🔑 M2 Google authentication and credential isolation | `██░░░░░░░░ 17.3%` | MVP |
| 🏢 M3 Admin API information parity | `██░░░░░░░░ 21.9%` | MVP |
| 📊 M4 Reporting, preflight and Data API parity | `██░░░░░░░░ 18.3%` | MVP |
| ✍️ M5 Explicitly gated GA mutation parity | `░░░░░░░░░░ 0.0%` | MVP |
| 🗃️ M6 DuckDB scratchpad and advanced analytics parity | `░░░░░░░░░░ 0.0%` | MVP |
| 🧬 M7 Tool contract interoperability and parity audits | `░░░░░░░░░░ 0.0%` | MVP |
| 🧪 M8 Verification, resilience and security acceptance | `░░░░░░░░░░ 3.6%` | MVP |
| 🚀 M9 Linux release and Codex-ready inauguration | `░░░░░░░░░░ 0.0%` | MVP |
| 🔭 M10 Post-MVP extensions and sustained stewardship | `░░░░░░░░░░ 0.0%` | POST_MVP |

Detailed milestone, phase, feature and atomic-task progress: [ROADMAP](docs/ROADMAP.md). [GitHub milestone issue queue](docs/ISSUES.md) keeps the 11 tracking issues in sync. Machine-readable evidence and checks: [progress.json](docs/progress.json). Exact combined tool surface: [tool-inventory.json](docs/tool-inventory.json). Source/protocol audit: [UPSTREAM](docs/UPSTREAM.md).

## 🦀 Mission and boundaries

- **Single native Rust executable** `synmetra`, built for Linux / EndeavourOS and Codex first, supporting stdio and secure Streamable HTTP MCP.
- All **36 distinct MCP tools** remain the target: Google's **10** (nine read and one optional gated annotation write), SednaLabs' **35**, **nine shared**. Currently 17 have provisional read-only routes; 19 remain unimplemented and no route has passed full upstream parity/live GA acceptance.
- Comprehensive Google Analytics Admin + Data API read/report parity, alpha conversions and funnels, optional annotation creation.
- Full optional DuckDB scratchpad, safe SQL sandbox, lifecycle/quotas, evidence export and release/landing comparisons.
- Read-only by default, `analytics.readonly` scope, native OAuth/ADC/service account support, `analytics.edit` only on explicit opt-in; inbound HTTP identity and Google identity kept separate.
- Preserve upstream feature semantics and argument coverage, add no surprise telemetry, protect credentials and user analytics.
- No new runtime dependence on Python, node, npm or gcloud. Embedded DuckDB is a native database dependency and **not pure Rust**; see [architecture](docs/ARCHITECTURE.md).

## 📚 Canonical documents

[🗺️ Roadmap](docs/ROADMAP.md) · [📌 Upstream feature audit](docs/UPSTREAM.md) · [🏗️ Architecture](docs/ARCHITECTURE.md) · [🔐 Security](docs/SECURITY.md) · [🎨 Color](docs/COLOR.md) · [⚖️ Provenance](docs/PROVENANCE.md) · [📋 Governance](docs/GOVERNANCE.md).

**License:** MIT. The Google and SednaLabs reference repositories are Apache-2.0. This project independently implements their documented behavior; copying their code requires preserving their copyright/license notices and tracking provenance.

## 🧪 Native development (experimental)

Build and launch the MCP stdio server with `cargo run --bin synmetra -- serve`. For desktop OAuth, use `synmetra auth login --client-id-file /path/to/desktop-oauth.json`; credentials are isolated to Synmetra and refreshed natively, without Python or npm. See [native MCP status and security caveats](docs/NATIVE_MCP.md). A valid Google Cloud OAuth Desktop client and actual GA4 property permissions are required for live use. Do not expose a remote endpoint or treat this as production-ready software.
