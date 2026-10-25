# 🦀 Native progress integrity

Synmetra contains a small native Rust maintenance binary called synmetra-progress. It is **not** the GA4 MCP server. Run it with:

    cargo run --bin synmetra-progress

It validates the pinned 36-tool union, all 227 atomic task IDs, evidence for every accepted check, the task percentage arithmetic, and README/roadmap progress bars. GitHub Actions runs its tests and the ledger checker for each push.

The baseline delta of +2.64 percentage points is documented in the initial bootstrap report. Future stewardship updates must compare the current committed ledger to the previous committed ledger and report a signed percentage-point delta in chat. This validator checks absolute totals, not historical deltas.

Never mark a task accepted solely because this validator passes: runtime tool parity still needs offline and authorized live Google acceptance evidence.
