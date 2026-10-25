# 🗂️ Synmetra GitHub issue index

Canonical task checklists and acceptance gates live in [docs/progress.json](progress.json) and [ROADMAP.md](ROADMAP.md). Issue checkboxes mirror evidence accepted by the ledger; changing a checkbox alone does not count as progress. Every milestone issue contains all phase and atomic task names.

- 🧭 [M0 · Reference baseline and project governance](https://github.com/sguzman/synmetra/issues/1) — 2 phases · 17 tasks · MVP
- 🔌 [M1 · MCP protocol and transport fidelity](https://github.com/sguzman/synmetra/issues/2) — 3 phases · 23 tasks · MVP
- 🔑 [M2 · Google authentication and credential isolation](https://github.com/sguzman/synmetra/issues/3) — 3 phases · 26 tasks · MVP
- 🏢 [M3 · Admin API information parity](https://github.com/sguzman/synmetra/issues/4) — 2 phases · 16 tasks · MVP
- 📊 [M4 · Reporting, preflight and Data API parity](https://github.com/sguzman/synmetra/issues/5) — 3 phases · 30 tasks · MVP
- ✍️ [M5 · Explicitly gated GA mutation parity](https://github.com/sguzman/synmetra/issues/6) — 2 phases · 13 tasks · MVP
- 🗃️ [M6 · DuckDB scratchpad and advanced analytics parity](https://github.com/sguzman/synmetra/issues/7) — 4 phases · 25 tasks · MVP
- 🧬 [M7 · Tool contract interoperability and parity audits](https://github.com/sguzman/synmetra/issues/8) — 3 phases · 17 tasks · MVP
- 🧪 [M8 · Verification, resilience and security acceptance](https://github.com/sguzman/synmetra/issues/9) — 3 phases · 28 tasks · MVP
- 🚀 [M9 · Linux release and Codex-ready inauguration](https://github.com/sguzman/synmetra/issues/10) — 3 phases · 22 tasks · MVP
- 🔭 [M10 · Post-MVP extensions and sustained stewardship](https://github.com/sguzman/synmetra/issues/11) — 2 phases · 10 tasks · POST_MVP

## 🔁 Daily stewardship protocol

1. Read the current head and existing progress ledger; avoid overwriting concurrent commits.
2. Select unaccepted atomic checks and perform implementation, testing and review autonomously.
3. Only credit verified checks with file/commit/CI or scoped live-run evidence; update the ledger first.
4. Update roadmap bars, README summary and GitHub tracking checkbox state to match.
5. Report **overall percent plus signed percentage-point delta**, and progress bars for every in-progress milestone, phase and task touched; omit completed 100% work from ongoing progress updates.
6. Preserve secret-free fixtures, original MIT licensing, Rust-native architecture and separate security/live gates.
