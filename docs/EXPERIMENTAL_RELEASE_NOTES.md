# Synmetra 0.0.1 — experimental development preview notes

**Status: not a published or production-supported GitHub Release.** The crate version is `0.0.1`; this is a source-tree preview accompanying the [Unreleased changelog](../CHANGELOG.md). There is no `v1.0.0` approval, no certified 36-tool live Google parity, and no authorization to expose the local HTTP transport publicly. Future tagged release notes require their own source SHA, published artifact digests, and accepted release gates.

## Source identity, licenses and provenance

- Project: [sguzman/synmetra](https://github.com/sguzman/synmetra), native Rust, edition 2021, project [MIT license](../LICENSE). Retain the source copyright notice.
- Tool contract references: [Google Analytics MCP](https://github.com/googleanalytics/google-analytics-mcp/tree/ce84226bf4cedea7f2910f5afb07a6b9138f6553) `ce84226bf4cedea7f2910f5afb07a6b9138f6553` (Apache-2.0), and [SednaLabs GA4 MCP](https://github.com/sednalabs/ga4-mcp/tree/039fd4620064fc4b76dc6bca641945aba52c14bc) `039fd4620064fc4b76dc6bca641945aba52c14bc` (Apache-2.0). The independently written Rust implementation is not endorsed by either upstream project.
- Reference schema snapshots, their Git hashes, and the retained SednaLabs license are recorded in [PROVENANCE.md](PROVENANCE.md), [schema-lock.tsv](../spec/upstream/schema-lock.tsv), and [licenses](../licenses/). Referencing a provider tool name is not evidence of equivalent live behavior.
- Dependency inventory: pinned `Cargo.lock`, optional native DuckDB 1.4.2 with bundled C/C++ engine, and [experimental all-feature Rust CycloneDX plus default-binary ELF inventory](SBOM.md). Cargo metadata licenses and allowed-license checks do **not** independently certify native DuckDB, system runtime, or all transitive redistributed code.
- Experimental provenance example: [Linux artifact workflow 40359969759](https://github.com/sguzman/synmetra/actions/runs/40359969759) produced a checked archive, SBOM, checksum and short-lived Sigstore/GitHub workflow attestation for source `0f5e9a81bfa3fd962a7d026a080a70b6cefd68b8`. This is a historical CI artifact, not a stable `0.0.1` tagged release, a permanent binary download, or an attestation for later `main` revisions.

## Available experimental capabilities

- Native Rust MCP stdio server with 20 read-only default names. Local authenticated Streamable HTTP is opt-in and fixed to loopback. `--allow-write` separately advertises one annotation mutation and does not itself grant Google edit scope or GA4 property role.
- Native Desktop OAuth with local loopback PKCE and private XDG credentials. Offline `auth status`, `auth doctor`, `--help` and `--version` do not require Python, Node, npm or gcloud.
- Provisional Google Analytics Data/Admin routes: historical, realtime, pivot, batch, compatibility, access audit and alpha-adjacent funnel/conversion. Requests are bounded, errors redacted, read-only retry budgets explicit, and full native schema inputs pinned.
- Optional `scratchpad` feature adds 15 DuckDB tools for local session management, bounded historical/realtime ingestion, restricted SELECT querying, analytical diagnostics and JSON/Markdown evidence export. The native C/C++ engine needs separate build/linkage/license review; scratchpad over HTTP is deliberately rejected.
- Offline QA covers canonical 36-name routing, response envelopes, mock Google API fixtures, hostile parameters, schema locks, opt-in write-denial controls and Linux build/startup smoke. These are **synthetic acceptance results only**.

## Known Google Analytics provider and alpha limitations

- **No authorized live test property or account is currently accepted.** Google API activation, OAuth consent, property role and quota-project authorization must be verified with a consenting operator and a private test property; no public repository artifact may contain tokens, account IDs or real visitor reports.
- Google Data **v1alpha** funnel/conversion behavior and eligibility remain unverified. A valid `analytics.readonly` scope does not automatically enable alpha methods, guarantee the relevant metric support, or establish complete GA4 report parity. The precise endpoint may reject an otherwise authenticated property; record denial rather than silently falling back.
- Google Admin/Data endpoints have distinct authorization requirements. Access audit reports may require Analytics Administrator privileges; annotation writes require `analytics.edit`, an edit-capable property role, `--allow-write` and explicit client approval. No permission escalation, impersonation or mutation replay may occur automatically.
- Upstream optional credentials, ADC, quota/billing and client-specific trust paths are **not** uniformly implemented. Unsupported fields should fail clearly rather than quietly expanding privileges.
- A pinned 36-name union and source-derived schema/input checks do not establish identical nested types, output projections, elapsed timing semantics, cursor semantics, quota headers, metadata, eligibility, or third-party MCP client behavior. See [CANONICAL_CONTRACT_DECISION.md](CANONICAL_CONTRACT_DECISION.md) and [LIVE_ACCEPTANCE_MATRIX.md](LIVE_ACCEPTANCE_MATRIX.md).

## Security, interoperability and distribution limitations

- The local bearer for loopback HTTP and Google bearer credentials are separate. There is no approved TLS remote service or per-principal DuckDB session isolation; do not reverse proxy this preview onto a public interface.
- Credential Unix permissions protect against other users, not hostile same-UID malware. Google API/tool results are untrusted data and should not be treated as instructions.
- DuckDB restricts SQL and external engine access and caps exported evidence, but this does not prove complete file/network sandboxing, fuzz coverage, crash cleanup, temp-file sanitation or third-party native dependency safety.
- The experimental x86-64 release smoke, source attestations and Rust-license scan are **not** a clean-install EndeavourOS proof, signed stable semver release, independently audited supply chain or ARM64 binary certification.
- Full MCP Inspector, Codex multi-tool, authorized Google login/query and live write tests are independent open gates. The [roadmap](ROADMAP.md) and [machine-readable ledger](progress.json) determine readiness, not these prose release notes.

## Evidence/packaging checklist for a future tagged release

A future release note must specify the **actual tagged source SHA**, compatible migration and breaking changes, source/upstream fixture SHAs, supported transports and auth surfaces, accepted Google/alpha behavior, exact artifact/checksum/SBOM/attestation links, third-party license notices, supported architectures/install/uninstall, security advisories and explicit unresolved defects. Do not promote this untagged preview to a stable version by copying its text. The [release policy](RELEASE_POLICY.md) blocks `v1.0.0` until the appropriate acceptance gates are met.
