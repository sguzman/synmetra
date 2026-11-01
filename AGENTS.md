# Codex + Nomos repository instructions

This repository uses a modular Nomos rule library. Read `.nomos/INDEX.md` when first onboarding or when unfamiliar with the available modules. Consult the **applicable sections** of `.nomos/core/progress.md`, `.nomos/core/reporting.md`, and `.nomos/core/workflow.md` as needed for the current work. Do **not** dump entire large modules into context on every continuation turn; retain the already-established rules and reopen only relevant sections when the task or context changes.

Before specialist work, consult the relevant rules in `.nomos/gui/`, `.nomos/ports/`, `.nomos/forks/`, `.nomos/development/`, `.nomos/browser/`, `.nomos/quality/`, `.nomos/delivery/`, `.nomos/security/`, `.nomos/tarium/` or `.nomos/ontology/`, **only to the depth required by the affected surface**. Consult new modules if scope changes.

Never treat library presence as blanket rule activation. Apply the original canonical statement only to its stated scope, retain its conditions and exceptions, and distinguish preferences from mandates. Follow explicit owner/project instructions and do not invent work outside authorized goals.

Report actual verified progress, completion percent, percentage-point delta and relevant blockers, without inventing activity. Once authorized work is 100% verified complete, stop.

Selected historical/current prompts in `.nomos/prompts/current-trievium.json` are references, not automatic activation of 24 simultaneous instructions.

If a project already has `AGENTS.md`, merge these instructions without overwriting the original.

## Synmetra project scope

Synmetra is a **native Rust Google Analytics 4 MCP server** targeting the feature union of the pinned official Python and SednaLabs Rust reference servers. Follow the existing project specification; do not add unrelated features or declare full parity solely from tool names or synthetic tests.

- Use `docs/progress.json` as the accepted progress ledger and `docs/ROADMAP.md` as its human-readable roadmap. Preserve the documented evidence-backed percentage calculation; report verified progress and delta, with no padding. These files are large: inspect selected task/check objects or relevant roadmap headings with precise searches or scripts, rather than repeatedly loading entire documents into context.
- Treat `README.md`, `docs/UPSTREAM.md`, `docs/ARCHITECTURE.md`, `docs/CONTRACT.md`, `docs/SCHEMA_PARITY_AUDIT.md`, and source/spec fixtures as authoritative **task-specific references**, not a checklist to reread in full before every edit. For touched surfaces, consult relevant passages and dedicated acceptance/security documents.
- Apply `.nomos/development/rust.md`, `.nomos/ports/common.md`, `.nomos/quality/ci.md`, `.nomos/quality/testing.md`, and relevant security modules as scoped references. The Python-to-Rust module applies only to supported contracts actually being ported from the Python reference; do not invent a requirement for Python interpreter/runtime emulation or override Synmetra's authorized feature-union scope.
- Maintain read-only defaults and credential isolation. Do not use or commit personal Google tokens, live analytics, or private account data. Synthetic tests do **not** establish authorized live Google acceptance.
- Complete independently verifiable tasks using existing test/CI workflows, commit coherent changes to `main`, and pause for external authorization only at genuine live-access or deployment gates. No tag or release before existing acceptance gates are actually met.
- Existing repo-level policies, licenses, and security constraints remain in force. The Nomos reference library is not a replacement for the project's concrete specifications.

## Autonomous continuation and environment-blocker policy

The owner delegates ongoing iteration toward the **existing approved roadmap**. A subsequent message consisting only of `continue` authorizes the next useful development increment, not merely a recap. Use the available session to make sustained, independently verified progress. Do not stop after a short attempt because the first chosen test, dependency download, network request, or task is blocked. Do not invent new work to fill time or claim progress without evidence.

1. **Plan multiple viable tasks.** At each iteration, inspect the current tracked unfinished tasks and select a primary task plus at least one independently actionable fallback where practical. Prefer tasks with local acceptance evidence available now; do not repeatedly choose a known external-access blocker when other approved work exists.
2. **Treat environmental failures as scoped blockers.** If a required Rust toolchain is missing, dependencies are uncached, the network stalls, CI cannot be queried, or a command needs unavailable approval, diagnose and attempt safe bounded recovery once. Do not loop on the same unavailable resource, and do not terminate the whole iteration while another meaningful authorized task is feasible. Move to offline fixture/schema audits, implementation, test authoring, security review, documentation corrections, or other *pre-existing* roadmap tasks whose progress can be independently verified. A fallback is not license to fabricate acceptance.
3. **Preserve reproducibility.** Never silently relax `rust-toolchain.toml`, alter dependency pins or the lockfile solely to make unavailable tests appear green, bypass approval, or treat a build with an unpinned Rust toolchain as the pinned acceptance result. An installed alternate toolchain may be used for clearly labeled *diagnostics*, but not for acceptance that requires the pinned one. Record exact verification limitations.
4. **Use local and remote evidence correctly.** When GitHub CI queries fail, inspect local Git history and run checks that genuinely work offline; revisit CI only when reachable. Historical CI success does not accept later untested changes. If a check cannot run, leave its acceptance item unaccepted, but continue on independent items.
5. **Commit meaningful increments.** Commit coherent, locally supportable changes on `main` as permitted by the repo policy. Work on additional authorized unfinished tasks within the session after a commit, rather than ending merely because one increment is complete. Never change `docs/progress.json` without its specified supporting evidence and regenerate dependent roadmap artifacts only through established project workflows.
6. **Stop only for a real boundary.** Report a terminal blocker only after considering alternate unfinished authorized tasks and determining none can be advanced responsibly under the current permissions and tools. Explain what was attempted, what remains blocked, and the concrete next action required. If a long-running command is still active, report its status rather than spawning redundant work or claiming completion.

This policy governs how to persist through transient environmental blockers; it does not override credential, network, security, release, acceptance-evidence, or user-approval requirements elsewhere in this repository and Nomos.
