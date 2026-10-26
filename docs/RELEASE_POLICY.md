# Experimental release and version policy

**State:** No production parity release is authorized. A CI artifact is not a signed or published GitHub Release, nor evidence of live Google certification.

## Semver and source policy

- The Rust crate is at `0.0.1`, an experimental version, not a full 36-tool parity guarantee.
- Release tags must identify an immutable commit, committed `Cargo.lock`, Rust 1.99.1 and pinned Google/SednaLabs schema fixtures. Do not release a mutable `main` reference.
- Before 1.0, changed MCP input/output or auth contracts require explicit migration notes. Never silently change behavior.
- Never include tokens, account IDs, property identifiers or private debugging text in public commits, notes and artifacts.

## Evidence-gated release sequence

The [Linux release workflow](../.github/workflows/linux-release.yml) builds a GNU/Linux x86-64 executable, packages a normalized tarball with the MIT license, install guide and source commit, verifies its SHA-256, extracts and executes it in an isolated environment with no external executable PATH, and publishes only an expiring GitHub Actions artifact. Artifact testing is independent of a live Google account.

One runner's successful build does **not** establish byte-for-byte reproducibility across independent runners. That verification, a dedicated Arch package, software bill of materials, provenance attestations and release signatures are not yet certified. They remain tracked acceptance gates.

Do not publish a stable or production release until independent checks pass for upstream tool contract semantics, authorized Google login and reports, permissions and opt-in writes, client transport compatibility, DuckDB security, native Linux packaging and the release artifact lifecycle.

## Changelog rule

For each tagged version, add a dated entry in [CHANGELOG.md](../CHANGELOG.md) with changed features, breaking MCP contracts, upstream SHA changes, security work, test evidence and known limitations. Unreleased work stays under the Unreleased heading.
