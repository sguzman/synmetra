# 🔏 Local execution and credential-boundary threat model

**Status:** Pre-live review baseline, 2026-10-25. This is a technical control inventory, **not** a penetration-test result, production-hosting approval, or authorization for live Google credentials.

## Assets and threat actors

The assets are Synmetra's OAuth refresh/access tokens, inbound HTTP bearer, optional upstream token, Google Analytics property/account data, local DuckDB session data, evidence exports and the MCP response stream. Threat actors include hostile agent-generated tool arguments (including prompt injection), an unauthorized loopback caller, a forged OAuth callback, a malicious same-user process, hostile provider JSON or SQL, and compromised dependencies.

The process cannot isolate secrets from an adversary already executing as the same Unix account. File mode 0600 and private directories protect against other users, not same-user malware.

## Trust-boundary review

| Boundary | Existing controls | Residual / unaccepted |
| --- | --- | --- |
| MCP stdio to Rust tools | Typed registrations and separate default read-only, opt-in mutation and DuckDB profiles (src/main.rs); src/stdio_limit.rs bounds each line-delimited frame to 1 MiB before rmcp's line buffer via a 64 KiB backpressured pipe; CI tests oversized unterminated frames and clean stdout | Inherited OS identity and environment; tool arguments remain untrusted; cancellation of outbound calls tracked separately |
| Optional loopback HTTP | src/http.rs binds 127.0.0.1; bearer comparison is constant time; Host and Origin validated; request size 1 MiB, in-flight 16, body 10 s and handler 30 s caps; independent principal/session replay tests | No TLS, public binding or multi-tenant identity accepted; never port-forward or reverse-proxy |
| Browser OAuth callback | src/auth.rs uses localhost, random state and PKCE S256; read-only analytics scope by default; headless/port options do not need gcloud, Python or npm | Live user consent, expired/revoked credentials, clock skew and rotation races not accepted |
| Local credentials | XDG-scoped Synmetra file with private-file/unsafe symlink protections and scoped logout; diagnostic output excludes tokens | Disk credentials are not encrypted against the same Unix account; backups and compromised processes remain risks |
| Outbound Google API | src/google.rs uses HTTPS/rustls, fixed default Google hosts, no redirects, 8 s connect/20 s request limits, bounded provider bodies and read-only retries | Live scope, account role, quota and response parity not independently verified |
| Read access versus annotation writes | Annotation mutation is hidden under default profile and additionally requires edit-capable Google authentication | Per-identity policy, explicit human client confirmation and redacted write audit gate T119/T123 remain open |
| DuckDB scratchpad | src/scratchpad.rs has bounded sessions/data and restrictive SELECT grammar, with hostile SQL test fixtures | Native library dependency, parser bypass fuzzing, file/session isolation, evidence permissions and crash cleanup require further testing |
| Repository to release binary | Locked Rust dependencies, SHA-locked pinned upstream fixtures, reproducible progress validator, periodic contract drift watch and mock change detection | Dedicated vulnerability, secret and license scans, SBOM, signed releases and clean EndeavourOS installs remain open |

## Concrete misuse cases and required controls

1. An agent passes attacker-controlled filters, identifiers, SQL or output controls. Validate types, recursion, lengths and paging bounds before Google or DuckDB access; complete T188/T192 fuzz and injection acceptance.
2. Another local client tries replaying HTTP sessions or guessing access tokens. Enforce bearer, Host/Origin and session identity boundaries. Do not treat the inbound bearer as a Google credential. Complete T058/T059.
3. A forged browser callback or malicious credential file attempts to redirect token storage. Enforce state, PKCE, loopback and private-file checks; test revocation, refreshing, symlinks, race conditions, errors and logging under T057/T060/T064/T191.
4. A read-only user asks an AI client to write an annotation. Hide mutation unless explicitly enabled, verify analytics.edit and a sufficient property role, and provide confirmation/audit controls before live writes (T118/T119/T123).
5. A Google API sends oversized/malformed bodies or rate limits, or a DuckDB request uses an unsafe SQL function. Bound responses, retries, and queries; deny external SQL functions and loading paths. Finish T146–T149 and T175.
6. An accidental public commit, build log or release includes secrets or disallowed dependencies. Use only synthetic public fixtures and enforce T013/T177/T193/T216 before a production release.

## Security decision and acceptance scope

**Permitted:** credential-free test fixtures, synthetic loopback requests, source-schema audits, read-only local experimentation, and explicitly user-authorized future QA.

**Not accepted:** public HTTP hosting, multi-user delegation, production analytics export, safety against same-UID malware, complete upstream response parity, or a successful real Google OAuth/property session. A passing mocked test cannot satisfy an authorized live acceptance gate.

**Pre-live checkpoint:** reconcile outstanding credential, scope and identity controls in M2; perform authorized OAuth and property tests T178–T186 with redacted evidence. Browser consent is an explicit user action.

**Release checkpoint:** test or explicitly reject unresolved T013/T016, T119/T123, T142–T149, T175/T177, T187–T195 and M9 requirements. No v1.0.0 tag from synthetic-only success.

**Review outcome:** This document supplies a reviewable threat-model baseline and identified residual risks. Independent pre-live security acceptance is *pending* rather than claimed complete.
