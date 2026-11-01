# Codex + Nomos repository instructions

This repository uses a modular Nomos rule library. Start each task by reading `.nomos/INDEX.md` and available `.nomos/core/progress.md`, `.nomos/core/reporting.md`, and `.nomos/core/workflow.md`.

Before specialist work, read relevant modules in `.nomos/gui/`, `.nomos/ports/`, `.nomos/forks/`, `.nomos/development/`, `.nomos/browser/`, `.nomos/quality/`, `.nomos/delivery/`, `.nomos/security/`, `.nomos/tarium/` or `.nomos/ontology/`. Consult new modules if scope changes.

Never treat library presence as blanket rule activation. Apply the original canonical statement only to its stated scope, retain its conditions and exceptions, and distinguish preferences from mandates. Follow explicit owner/project instructions and do not invent work outside authorized goals.

Report actual verified progress, completion percent, percentage-point delta and relevant blockers, without inventing activity. Once authorized work is 100% verified complete, stop.

Selected historical/current prompts in `.nomos/prompts/current-trievium.json` are references, not automatic activation of 24 simultaneous instructions.

If a project already has `AGENTS.md`, merge these instructions without overwriting the original.

## Synmetra project scope

Synmetra is a **native Rust Google Analytics 4 MCP server** targeting the feature union of the pinned official Python and SednaLabs Rust reference servers. Follow the existing project specification; do not add unrelated features or declare full parity solely from tool names or synthetic tests.

- Use `docs/progress.json` as the accepted progress ledger and `docs/ROADMAP.md` as its human-readable roadmap. Preserve the documented evidence-backed percentage calculation; report verified progress and delta, with no padding.
- Start with `README.md`, `docs/UPSTREAM.md`, `docs/ARCHITECTURE.md`, `docs/CONTRACT.md`, `docs/SCHEMA_PARITY_AUDIT.md`, and relevant source/spec fixtures. For touched surfaces, check their dedicated acceptance and security documents.
- Apply `.nomos/development/rust.md`, `.nomos/ports/common.md`, `.nomos/quality/ci.md`, `.nomos/quality/testing.md`, and relevant security modules as scoped references. The Python-to-Rust module applies only to supported contracts actually being ported from the Python reference; do not invent a requirement for Python interpreter/runtime emulation or override Synmetra's authorized feature-union scope.
- Maintain read-only defaults and credential isolation. Do not use or commit personal Google tokens, live analytics, or private account data. Synthetic tests do **not** establish authorized live Google acceptance.
- Complete independently verifiable tasks using existing test/CI workflows, commit coherent changes to `main`, and pause for external authorization only at genuine live-access or deployment gates. No tag or release before existing acceptance gates are actually met.
- Existing repo-level policies, licenses, and security constraints remain in force. The Nomos reference library is not a replacement for the project's concrete specifications.
