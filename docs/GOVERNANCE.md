# 📋 Synmetra stewardship contract

- **Repository ownership:** the GitHub owner retains actual control; ChatGPT is delegated planning/development stewardship, not literal GitHub account ownership.
- **Branch:** work directly on `main` unless expressly instructed otherwise; prefer atomic commits, no PR ceremony. Before pushing, inspect head and use a compare-and-swap/expected SHA to avoid overwriting concurrent changes. Never force-push by default.
- **Scope:** full feature union of the pinned official and Rust GA4 MCP servers is the MVP. Defer enhancements beyond this to M10. Do not claim 100% for code that is untested or a service that has not been live verified.
- **Roles:** assistant/director owns planning, research, decomposition, implementation, review, GitHub documentation, CI inspection, QA and evidence ledger; user supplies goals and any auth approval requiring their identity. Do not require routine manual QA from user.
- **Progress format:** every round report **overall** `████░░░░░░ 40%` and **signed Δ percentage points**, then milestone/phase/task visual bars for in-progress items worked on; omit completed items from ongoing updates. Source of truth `docs/progress.json` and every success claim needs specific committed evidence. Only real accepted checks change percentages; no speculative progress.
- **Security:** read-only Google mode by default. Never publish user secrets, venting, credentials, identity-specific data, transcript content, private GA property IDs or token-bearing QA logs. No Python/npm runtime. Preserve unrelated system tools and sessions.
- **Acceptance:** mock tests, upstream parity diff, live GA test property, security gate and clean-release smoke are distinct; do not mark runtime complete from schema parity alone.
