# Experimental release and version policy

**State:** No production parity release is authorized. A CI artifact is not a signed or published GitHub Release, nor evidence of live Google certification.

## Semver and source policy

- The Rust crate is at `0.0.1`, an experimental version, not a full 36-tool parity guarantee.
- Release tags must identify an immutable commit, committed `Cargo.lock`, Rust 1.99.1 and pinned Google/SednaLabs schema fixtures. Do not release a mutable `main` reference.
- Before 1.0, changed MCP input/output or auth contracts require explicit migration notes. Never silently change behavior.
- Never include tokens, account IDs, property identifiers or private debugging text in public commits, notes and artifacts.

## Evidence-gated release sequence

The [Linux release workflow](../.github/workflows/linux-release.yml) builds a GNU/Linux x86-64 executable, packages a normalized tarball with the MIT license, install guide and source commit, verifies its SHA-256, extracts and executes it in an isolated environment with no external executable PATH, and publishes only an expiring GitHub Actions artifact. Artifact testing is independent of a live Google account.

The release workflow now tests whether **two independent Cargo target directories** produce identical SHA-256 hashes from the same pinned Rust 1.99.1 source and lockfile on one Ubuntu runner. It rejects hash mismatches before packaging. This is a narrower, falsifiable same-runner reproducibility check and must pass CI before acceptance is credited.

The release workflow also invokes a SHA-pinned `actions/attest@v4` to produce a short-lived Sigstore-signed SLSA provenance statement for the **experimental archive**, and verifies it with GitHub CLI's `gh attestation verify` against the exact repository identity. This requires narrow GitHub Actions OIDC and attestation write permissions on that build job; no long-lived personal signing key is created. A passing signature binds the archive digest to its workflow identity, but **does not prove its dependencies are vulnerability-free or its output is functionally equivalent to Google Analytics**. No release tag is signed merely by attesting an experimental archive. The attestation change remains unaccepted until its own CI run completes successfully.

The [experimental SBOM workflow](SBOM.md) also generates a CycloneDX 1.5 **all-features Rust dependency inventory** and the native shared-library names of the **default** x86-64 binary, retaining separate source SHA and binary SHA-256. The generator deliberately does not claim to audit bundled DuckDB's C/C++ license obligations or prove full system-library redistribution compliance. The SBOM/inventory creation remains unaccepted until its CI job passes.

Even a passing comparison does **not** establish byte-for-byte reproducibility across independent runners, LLVM environments, alternative linkers, Linux distributions or feature-enabled native DuckDB variants. A dedicated clean EndeavourOS package test, complete native dependency license review, and published stable release signatures remain separate unaccepted gates.

Do not publish a stable or production release until independent checks pass for upstream tool contract semantics, authorized Google login and reports, permissions and opt-in writes, client transport compatibility, DuckDB security, native Linux packaging and the release artifact lifecycle.

## Changelog rule

For each tagged version, add a dated entry in [CHANGELOG.md](../CHANGELOG.md) with changed features, breaking MCP contracts, upstream SHA changes, security work, test evidence and known limitations. Unreleased work stays under the Unreleased heading.
