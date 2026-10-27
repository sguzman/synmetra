# 🔒 CI artifact privacy and publication gates

**Scope:** experimental, synthetic-only Synmetra CI outputs. No Google Analytics account, OAuth authorization, property ID, visitor record, or production secret is required by any artifact-producing workflow. This is a bounded disclosure-control policy, **not** a claim that regex scanning prevents all secrets or that actual authorized GA data has been processed.

## Current public artifact surfaces

| Workflow | Upload | Required checks before upload |
| --- | --- | --- |
| [Experimental Linux x86-64 release](../.github/workflows/linux-release.yml) | Native executable tarball, checksum, Rust SBOM, native runtime inventory | Reproducibility/checksum, archive manifest, private file/symlink denial, release documentation and SBOM/runtime text signature scan, binary printable-string signature scan |
| [Native Linux aarch64 release](../.github/workflows/linux-arm64.yml) | Native executable tarball, checksum, native linkage inventory | Native ARM smoke and checksum, exact archive manifest, linkage/checksum and extracted documentation text scan, binary printable-string signature scan |
| [Pinned SednaLabs Contract V1 golden capture](../.github/workflows/pinned-upstream-golden.yml) | Four synthetic generated JSON Contract V1 cases | Exact pinned upstream revision, Rust-executed no-credentials fixtures, JSON structure assertions, pre-upload high-confidence text signature scan |

All three upload paths use [ci/scan-public-artifacts.sh](../ci/scan-public-artifacts.sh) before `actions/upload-artifact`. The default native [core CI](../.github/workflows/ci.yml) runs its local negative tests using deliberately constructed synthetic token-shaped canaries: safe archive accepted; malicious manifest, README, SBOM, ARM64 linkage, executable strings, and standalone JSON rejected with **no matched contents printed**. Passing that test establishes the scanner's behavior, not the contents of every future asset.

The scanner rejects high-confidence GitHub, Google and OpenAI key-shaped material, private-key boundary markers in text files, and absolute CI checkout paths in candidate text assets. To avoid false positives from normal TLS/PEM parser libraries, the executable scan checks only key-shaped bearer tokens in printable strings, not the generic PEM header. Archive manifests are strictly allowlisted, extracted files cannot be symlinks, and each scanner input must exist. No wildcard arbitrary directory publishing is authorized.

## Boundaries and incident handling

- CI must use synthetic data and clear ambient Google/MCP bearer overrides in protocol tests. Do not upload account/visitor information, OAuth client JSON, refresh tokens, request headers, private issue reports, or live Google output.
- Scan failures must report **only the error category**, not the matched text, path or credential itself. Leave the upload step unexecuted after any failed guard.
- The Rust binary may contain legitimate system/library metadata; an isolated review of binary secrets and privacy remains required before production. Regex signatures cannot recognize arbitrary API keys, encrypted secrets, nonstandard personal analytics or sensitive semantic data.
- GitHub Actions logs, external attestation providers, historical/external artifacts, unrecognized binary encodings and authorized live-provider outputs are **not exhaustively scanned** by this package guard. The [current/reachable-history repository scanner](../ci/scan-secrets.sh) is a separate control; it does not replace artifact review.
- A production or live-Google release requires explicit safe-account permission, an independently reviewed T193 sign-off, and the separate T178–T186 and M9 gates. Never treat a green synthetic artifact job as proof of real user-data safety.

A reviewer should verify the three workflow paths, scan step ordering, negative-test results, exact source SHA and uploaded manifest before claiming the corresponding artifact's specific disclosure checks passed.
