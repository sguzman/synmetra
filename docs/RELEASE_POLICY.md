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

The release workflow also invokes a SHA-pinned `actions/attest@v4` to produce a short-lived Sigstore-signed SLSA provenance statement for the **experimental archive**, and verifies it with GitHub CLI's `gh attestation verify` against the exact repository identity. This requires narrow GitHub Actions OIDC and attestation write permissions on that build job; no long-lived personal signing key is created. A passing signature binds the archive digest to its workflow identity, but **does not prove its dependencies are vulnerability-free or its output is functionally equivalent to Google Analytics**. No release tag is signed merely by attesting an experimental archive. A signed SLSA v1 archive attestation was generated and successfully verified in [experimental workflow 40359969759](https://github.com/sguzman/synmetra/actions/runs/40359969759). This verifies repository/workflow identity for that one archive digest, not signed tags or a permanent production release.

The [experimental SBOM workflow](SBOM.md) also generates a CycloneDX 1.5 **all-features Rust dependency inventory** and the native shared-library names of the **default** x86-64 binary, retaining separate source SHA and binary SHA-256. The generator deliberately does not claim to audit bundled DuckDB's C/C++ license obligations or prove full system-library redistribution compliance. Both CycloneDX SBOM creation and default-feature `ldd` inventory validation passed [workflow 40359969759](https://github.com/sguzman/synmetra/actions/runs/40359969759) and were uploaded in its expiring [artifact 12667318243](https://github.com/sguzman/synmetra/actions/runs/40359969759/artifacts/12667318243). Native DuckDB C++ license and fresh Arch install gates remain open.

Before experimental artifact signing/upload, [ci/scan-public-artifacts.sh](../ci/scan-public-artifacts.sh) requires the exact five-entry source archive manifest, forbids archive symlinks and scans extracted text, checksum, SBOM and linked-library JSON for high-confidence key/PEM signatures and absolute CI workspace paths. A redacted synthetic positive/negative self-test runs in core CI. This guards the **published artifact contents** against accidental known secret patterns but is not a comprehensive audit of ELF binary strings, obfuscated tokens, unrecognized sensitive account data or GitHub Actions logs. The separate native dependency license scan remains necessary.

Even a passing comparison does **not** establish byte-for-byte reproducibility across independent runners, LLVM environments, alternative linkers, Linux distributions or feature-enabled native DuckDB variants. A dedicated clean EndeavourOS package test, complete native dependency license review, and published stable release signatures remain separate unaccepted gates.

Do not publish a stable or production release until independent checks pass for upstream tool contract semantics, authorized Google login and reports, permissions and opt-in writes, client transport compatibility, DuckDB security, native Linux packaging and the release artifact lifecycle.



## Verified optional native ARM64 experimental CI artifact

[The ARM64 workflow](../.github/workflows/linux-arm64.yml) uses GitHub's native `ubuntu-24.04-arm` public runner rather than an emulated x86 cross-build. It pins Rust 1.99.1 and Cargo.lock, runs the native default-feature tests, builds a release ELF, asserts its AArch64 machine type, audits dynamic shared-library resolution, rejects absent native dependencies and unexpected OpenSSL/Python/Node linkage, and exercises the extracted executable offline with an isolated XDG configuration and no configured credentials. It generates a normalized archive with license and full source commit, verifies a SHA-256 sidecar, creates and independently verifies a repository-bound SLSA build attestation, then uploads a **30-day expiring GitHub Actions artifact**.

This is a separately optional **default-feature** architecture build; it does not include the bundled DuckDB native C/C++ feature, an ARM64 EndeavourOS installation, a permanent GitHub Release, or production parity certification. The native Linux ARM64 workflow **passed all steps** on source commit `a9fcc2316d736c0c54cfa369d4d9e5bad7b87519` in [Actions run 40400440461](https://github.com/sguzman/synmetra/actions/runs/40400440461), and published [experimental artifact 12688357449](https://github.com/sguzman/synmetra/actions/runs/40400440461/artifacts/12688357449) with `.tar.gz`, matching `.sha256`, and a source-linked `aarch64` runtime linkage text inventory. The run verified a repository-bound SLSA v1 attestation as well as source SHA, ELF architecture, native smoke, and shared-library resolution. The artifact expires under GitHub's 30-day retention: it is not a permanent GitHub Release or a `v1.0.0` production binary.

## Changelog rule

For each tagged version, add a dated entry in [CHANGELOG.md](../CHANGELOG.md) with changed features, breaking MCP contracts, upstream SHA changes, security work, test evidence and known limitations. Unreleased work stays under the Unreleased heading.
