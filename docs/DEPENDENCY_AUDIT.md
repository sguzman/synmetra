# 🧷 Rust dependency, license and advisory audit

Synmetra's dependency policy is in the root [deny.toml](../deny.toml). The independent [dependency-audit workflow](../.github/workflows/dependency-audit.yml) uses the SHA-pinned cargo-deny GitHub Action (v2.1.1) to examine the locked Cargo graph with **all features**, including optional DuckDB bindings. It tests license allowlists, non-crates.io sources, explicit disallowed crate policies and RustSec advisories. No npm, Python or gcloud runtime is required for this audit.

## Evidence limits

- **License pass** means cargo-deny found a valid allowed license path for each analyzed Rust crate in the selected target graph. It is not an attorney's review and does not replace shipping license notices, source-offer obligations, or independent review of the bundled DuckDB **native C/C++** engine and system libraries (T150, T200, T216).
- **Advisory checks** report new RustSec database findings on weekly schedules and workflow dispatches. The advisory job is intentionally marked nonblocking to keep newly published findings from causing unrelated source-only pushes to fail. A warning/failure is *still an unresolved security finding*, not evidence of a safe release. Do not declare T177 fully accepted until its findings have been reviewed.
- **Source checks** reject unexpected external Cargo Git/registry dependencies by default. The pinned `Cargo.lock` continues to be the authoritative dependency version manifest, with optional features explicitly expanded.
- **Bans** reject wildcard Cargo dependency declarations; duplicate crate versions are visible warnings rather than unreviewed automatic blocking.
- Real secrets and project-private analytics are never intended to be stored in the public repository. The separate [source heuristic scanner](../ci/scan-secrets.sh) runs on every mainline push and blocks high-confidence GitHub/OpenAI/Google-style key or PEM patterns without printing matched values. It is not a comprehensive secret scanner: unrecognized/obfuscated credentials, files outside the checked worktree, history and connected services remain outside its coverage. Neither workflow is counted as proof of zero credential disclosures until independent scanner acceptance (T177).
- Reproducible binary hashes, code signing, and SBOM/release provenance remain separate open requirements (T195, T216).

The first license audit found the narrow `webpki-roots 1.0.11` `CDLA-Permissive-2.0` trust-anchor data license. The policy now allows that permissive **data license for that exact crate/version only**, instead of broadly permitting it for all dependencies. The audit remains unaccepted until a new passing workflow run verifies the exception.

CI workflow action uses an immutable upstream commit so a moving marketplace tag cannot silently change the audit executor. The audit uses GitHub-hosted Ubuntu runners rather than a personally installed Arch environment; a clean EndeavourOS install remains T217.
