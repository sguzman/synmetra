# 🗺️ Synmetra exhaustive roadmap

**Status:** experimental Rust MCP stdio server, 20 read-only handlers plus one opt-in write handler implemented provisionally; **0/36 fully parity-accepted**. This project is an experimental MCP server, not a production-ready GA4 parity release. **Source of truth:** [progress.json](progress.json). All percentages shown are task-check acceptance averages, not guesses of feature implementation.

**Overall:** `█████░░░░░ 45.2%` · **+45.15 percentage points from initial empty repo; +0.44 pp from verified pinned upstream fixture hashes and parity reference tests** · 86/227 accepted atomic tasks. **Production parity gate:** 36/36 documented tools, exact inputs, secure deployment, permitted live property tests and a native install.

### 📏 Progress conventions

- Every `Tnnn` is an atomic deliverable with two explicit acceptance checks in [progress.json](progress.json); both must be accepted with evidence for 100%. A source stub or draft is 0% unless a check has evidence.
- Phases and milestones average **atomic task** percentages, not pre-rounded visual bars. Overall averages all tasks including post-MVP work; v1 parity and post-MVP also tracked separately. The symbols `█` and `░` each represent 10 percentage points, rounded to the nearest block.
- On each stewardship round, report total and actively worked milestone/phase/task bars plus the **signed delta in percentage points** against the previous verified committed ledger; completed 100% tasks can be omitted in ongoing status updates.
- **No progress padding.** Upstream feature inventory, planning and brand assets count only for their specific documentation tasks; GA4 functionality starts at 0% until tests establish otherwise. Real Google access, credentials and live tests are separate explicit gates.

## 🧭 M0 — Reference baseline and project governance — `████████░░ 82.4%`

**Scope:** MVP. **Acceptance gate:** Audited pinned feature-union inventory, color identity, public-safe policies and reproducible plan. Foundation is not runtime parity.

### 📚 P00 — Upstream contracts and provenance — `██████████ 100.0%`

**Feature:** Reference provenance.

- 📌 **T001 — Pin official and Rust upstream commits** — `██████████ 100.0%`
  - ✅ **Gate:** Record both source SHAs and reference date with traceable links · **Evidence:** docs/UPSTREAM.md; docs/UPSTREAM.md pinned references
- 🧮 **T002 — Audit 36-tool feature union and overlaps** — `██████████ 100.0%`
  - ✅ **Gate:** Verify 10 Google and 35 Rust names, 9 shared, 36 unique · **Evidence:** docs/tool-inventory.json; docs/tool-inventory.json checked 10+35-9=36
- ⚖️ **T003 — Document license and independent reimplementation policy** — `██████████ 100.0%`
  - ✅ **Gate:** MIT Synmetra license retained; Apache upstream attributions and boundaries explained · **Evidence:** docs/PROVENANCE.md; LICENSE and docs/PROVENANCE.md attribution policy
- 🎨 **T004 — Select an sRGB-safe OKLCH brand color** — `██████████ 100.0%`
  - ✅ **Gate:** Publish swatch with verified dark-on-accent contrast · **Evidence:** docs/COLOR.md and assets/synmetra-color.svg; docs/COLOR.md contrast calculations
- 🗺️ **T005 — Write complete milestone-phase-task ledger and roadmap** — `██████████ 100.0%`
  - ✅ **Gate:** Document consistent task gates, percentages, visual bars and baseline deltas · **Evidence:** docs/ROADMAP.md and docs/progress.json; README.md and docs/ROADMAP.md reference ledger
- 🛡️ **T006 — Document privacy, security and maintainer governance** — `██████████ 100.0%`
  - ✅ **Gate:** Explicit no-secret and read-only defaults and evidence policy · **Evidence:** docs/GOVERNANCE.md and docs/SECURITY.md; docs/GOVERNANCE.md and docs/SECURITY.md approved policies
- 🧾 **T007 — Archive exact upstream tool schemas and signatures** — `██████████ 100.0%`
  - ✅ **Gate:** Verifiable pinned schema snapshots with tool argument parity tests · **Evidence:** spec/upstream/sednalabs-tool-schema.v1.json pinned exact 35-tool upstream schema, spec/upstream/google-official-signatures.json pinned Google's 10 tool functions, licenses/SEDNALABS-APACHE-2.0.txt attribution, docs/PROVENANCE.md and docs/UPSTREAM.md; GitHub Actions 40211821382 passed pinned inventory integrity. Full runtime schema parity remains open.; spec/upstream/schema-lock.tsv pins immutable Git blob hashes for the original SednaLabs 35-tool JSON Schema and Contract V1 response schema, independently derived 10-signature Google JSON and merged 36-tool inventory; docs/PROVENANCE.md identifies original source commits and redistribution license. CI 40247973673 verified every fixture via git hash-object; src/main.rs every_registered_tool_preserves_pinned_required_mcp_arguments, full_capability_profile_matches_exact_pinned_36_tool_union and specific per-tool contract tests verify required arguments against these locked references. This archives/verifies fixtures, not full parameter/response parity.
- 🔁 **T008 — Set up upstream drift tracking and version pin policy** — `██████████ 100.0%`
  - ✅ **Gate:** CI alerts on name/signature deltas without silent breaking changes · **Evidence:** docs/UPSTREAM.md, docs/PROVENANCE.md, docs/tool-inventory.json and src/bin/synmetra-progress.rs: pinned source revisions and CI-checked 36-name union; GitHub Actions 40211821382 passed; upstream signature drift alert automation remains open.; .github/workflows/upstream-drift.yml now checks pinned upstream signature-bearing source blobs on weekly schedule, manual dispatch and changes to the watch/lock; spec/upstream/drift-lock.json pins SednaLabs schema snapshot blob 646c0baa and Google coordinator blob a1e7936a at separately documented source commits; drift produces a failed GitHub Actions job and review guidance without silently updating the baseline. docs/UPSTREAM.md documents the conservative source-change policy and its limitations. GitHub Actions 40241902469 verified both public current blobs against the pins; 40241902503 core and DuckDB passed. The watch detects file changes for review, not automatic parsing of Google's entire dynamic runtime schema.

### 🦀 P01 — Native scaffold and quality gates — `███████░░░ 66.7%`

**Feature:** Rust foundation.

- 📦 **T009 — Initialize single-crate Rust executable synmetra** — `██████████ 100.0%`
  - ✅ **Gate:** Cargo metadata and CLI build on stable Rust x86-64 Linux · **Evidence:** Cargo.toml; src/main.rs; Actions 40202015488 compiled Synmetra native Rust binary and passed unit tests; Cargo.toml; src/main.rs; Actions 40202015488 compiled Synmetra native Rust binary and passed unit tests
- 🔌 **T010 — Integrate rmcp server SDK foundation** — `██████████ 100.0%`
  - ✅ **Gate:** Protocol version pin plus compile-time registration tests · **Evidence:** src/main.rs rmcp 3.5.1 tool router and 17 registered MCP handlers; Actions 40202015488 passed; Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.
- 🌐 **T011 — Add reqwest rustls API client skeleton** — `██████████ 100.0%`
  - ✅ **Gate:** No OpenSSL or Python runtime dependency · **Evidence:** src/google.rs reqwest/rustls fixed-host Admin/Data HTTP transport; Actions 40202015488 passed; Cargo.toml uses reqwest default-features=false with rustls-tls, no Python/OpenSSL requirement. .github/workflows/ci.yml native runtime step runs locked cargo tree and fails on openssl/openssl-sys/native-tls/pyo3 dependencies, builds the synmetra executable, then fails if ldd shows libssl/libcrypto/libpython. GitHub Actions 40241002371 both core and DuckDB jobs passed, including this native dependency/linkage check. Acceptance covers the default native Linux API server binary and dependency graph, not a claim about every user-installed system library.
- 🧪 **T012 — Add fmt clippy unit tests and CI jobs** — `██████████ 100.0%`
  - ✅ **Gate:** Green push checks with pinned toolchain and dependency cache · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; rust-toolchain.toml pins Rust 1.99.1 (rustfmt, Clippy); both core and DuckDB CI jobs explicitly install and assert rustc 1.99.1, reuse Swatinem/rust-cache@v2, use Cargo.lock via --locked and enforce formatting/tests/strict Clippy/progress integrity. GitHub Actions run 40243957373 completed successfully for core and DuckDB. Synthetic and release-tag DuckDB strict Clippy remain separate verification goals.
- 🧷 **T013 — Add lockfile dependency audits and license checks** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Dependencies auditable with deny or equivalent process
- 🧰 **T014 — Create deterministic offline fixtures and mock server** — `██████████ 100.0%`
  - ✅ **Gate:** No secrets or real analytics identifiers in fixtures · **Evidence:** src/google.rs synthetic tokio TcpListener Google API mock tests; Actions 40202015488 passed without real credentials; src/google.rs synthetic tokio TcpListener Google API mock tests; Actions 40202015488 passed without real credentials
- 🧭 **T015 — Implement CLI config parsing and doctor subcommands** — `██████████ 100.0%`
  - ✅ **Gate:** Actionable diagnostics and typed failures without exposing secrets · **Evidence:** src/auth.rs implements native `auth doctor`, `auth command`, `auth status`, `auth login`, `auth logout`; public_status redacts token values and offline unit tests passed core run 40212316165; live Google authorization and full operator QA remain unverified.; src/auth.rs CLI offers private read-only auth doctor/status/command, typed malformed login validation and revocation guidance. GitHub Actions 40244436625 built the actual synmetra executable and passed Verify native auth doctor and credential-safe CLI failures, with isolated XDG_CONFIG_HOME, no configured credentials, synthetic override-token canary never disclosed, actionable Admin/Data API prerequisites, failed headless login without --client-id-file, and logout guidance. This accepts local CLI diagnostics, not an authorized live OAuth login.
- 🔏 **T016 — Threat-model local execution and its credential boundary** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Review accepted before live Google credentials
- 🗒️ **T017 — Create changelog and reproducible release policy** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Document semver and provenance gates


## 🔌 M1 — MCP protocol and transport fidelity — `██████░░░░ 60.9%`

**Scope:** MVP. **Acceptance gate:** Spec-conforming native MCP discovery and calls over stdio plus bounded Streamable HTTP; all transport safety tests pass.

### 📨 P02 — Stdio protocol — `█████░░░░░ 50.0%`

**Feature:** MCP stdio.

- 📥 **T018 — Support MCP initialize and capability negotiation** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Inspector negotiates compatible protocol version and identity
- 📋 **T019 — Support tools/list with exact declared JSON schemas** — `██████████ 100.0%`
  - ✅ **Gate:** All available tools discoverable by MCP clients · **Evidence:** src/main.rs typed schemars tool metadata; in-memory client lists 17 handlers in Actions 40202015488; Real binary stdio JSON-RPC tools/list issued after initialize and notifications/initialized returned the exact 20 read-only handlers, each with object inputSchema, including ga4_get_started and excluding create_property_annotation; .github/workflows/ci.yml real-process smoke in GitHub Actions run 40243631855 succeeded alongside existing in-memory MCP tool discovery and pinned field tests. Feature-enabled 36-name union remains separate.
- ⚙️ **T020 — Support tools/call dispatch and structured content** — `██████████ 100.0%`
  - ✅ **Gate:** Call requests resolve to tools with valid response envelopes · **Evidence:** src/main.rs actual rmcp tools/call of get_account_summaries via in-memory client; Actions 40202015488 passed; Real binary stdio JSON-RPC tools/call dispatched ga4_get_started following initialize and tools/list and returned a successful MCP result with structuredContent.ok=true and the precise read-only analytics scope. Real-process CI run 40243631855 passed; existing in-memory rmcp call and structured envelope tests cover invalid/no-auth cases.
- 📣 **T021 — Implement JSON-RPC notifications and cancellation** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Cancelled requests do not spawn orphan Google calls
- 🚧 **T022 — Separate stdout protocol from stderr diagnostics** — `██████████ 100.0%`
  - ✅ **Gate:** No banners or logging contaminate JSON-RPC stdout · **Evidence:** src/main.rs stdio transport only on serve path; application diagnostic output uses stderr; Actions 40202015488 passed; Real native synmetra serve process captured stdout and stderr separately, enforced exactly three well-formed JSON-RPC result lines for initialize/list/call with no banner, and confirmed a synthetic SYNMETRA_GOOGLE_ACCESS_TOKEN canary never entered stdout. .github/workflows/ci.yml CI run 40243631855 core and DuckDB passed. Provider logs/stderr remain intentionally separate.
- 📏 **T023 — Bound request sizes concurrency and timeouts** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Malformed and oversize frames rejected deterministically
- ⌨️ **T024 — Integrate Codex stdio configuration smoke** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Codex recognizes locally built synmetra binary

### 🌐 P03 — Streamable HTTP protocol — `███████░░░ 72.2%`

**Feature:** MCP Streamable HTTP.

- 🌍 **T025 — Expose Streamable HTTP MCP endpoint** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Official MCP inspector connects and initializes
- 🏠 **T026 — Default bind to loopback only** — `██████████ 100.0%`
  - ✅ **Gate:** Non-loopback starts are rejected unless policy configured · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; src/http.rs serve_loopback binds exclusively to IPv4 127.0.0.1 using loopback_address; production_http_bind_is_always_ipv4_loopback asserts the invariant at multiple ports; .github/workflows/ci.yml launches an unsupported nonloopback override and requires CLI rejection without output. Synthetic wrong Host/Origin/unauthorized requests denied. GitHub Actions 40246360428 core and DuckDB jobs succeeded. Public/network bind is intentionally unsupported rather than enabled by policy.
- 🧿 **T027 — Validate Host Origin and DNS rebinding defenses** — `██████████ 100.0%`
  - ✅ **Gate:** Host header and Origin tests block inappropriate requests · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.
- 🔐 **T028 — Add inbound bearer authorization policy** — `██████████ 100.0%`
  - ✅ **Gate:** Unauthenticated requests cannot enumerate tools on protected endpoints · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.
- 🧭 **T029 — Serve protected MCP metadata and health endpoints** — `██████████ 100.0%`
  - ✅ **Gate:** Health is not publicly exposed under inbound auth · **Evidence:** src/http.rs adds /health and /metadata GET routes to the optional authenticated loopback MCP router. Health returns only an uptime-style status; metadata lists static server transport and auth policy without Google account or credential information. Both routes use require_local_auth, shared with /mcp.; src/http.rs metadata_and_health_are_authenticated_and_never_contain_credentials uses a real local HTTP listener to verify missing/wrong bearer and malicious Origin return 401, correct bearer returns JSON without tokens or private account/tool data. GitHub Actions 40246616329 core tests, fmt and Clippy plus native DuckDB job passed. No public or unauthenticated health surface is enabled.
- 📡 **T030 — Support authenticated sessions and request routing** — `██████████ 100.0%`
  - ✅ **Gate:** Sessions terminate and cannot cross principal boundaries · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; src/http.rs authenticated_http_session_lists_and_calls_tools_but_rejects_invalid_sessions verifies initialized session tokens, authenticated read-only MCP tool routing, invalid/foreign IDs rejected, HTTP session DELETE terminates and cannot be reused; separate_local_bearer_principals_cannot_replay_foreign_sessions runs two independently authenticated loopback servers and proves a valid token for server B cannot replay server A session, token A on B is denied, and sessions are distinct. GitHub Actions 40247495277 passed core+DuckDB, fmt and Clippy. A server is single-principal by design; this does not imply multi-tenant principal management.
- 🧊 **T031 — Enforce payload and connection resource limits** — `██████████ 100.0%`
  - ✅ **Gate:** Slow and oversized requests are bounded under load · **Evidence:** src/http.rs authenticated require_local_auth bounds requests to 1,048,576 bytes using Content-Length precheck plus axum::body::to_bytes hard limit, 16 concurrent in-flight handlers via Tokio semaphore, 10s body-read deadline and 30s handler deadline. Refuses unauthorized requests before consuming bodies. Implementation applies to protected /mcp, /health and /metadata.; src/http.rs authenticated_http_rejects_oversize_slow_and_overloaded_requests launches local TCP HTTP server: 1,048,577-byte body returns 413, saturated one-slot semaphore returns 503, incomplete slow Content-Length body exceeds injected 150ms deadline and receives 408. CI 40246890771 passed core tests, rustfmt, strict Clippy and DuckDB. Limits bound request processing, not an end-to-end guarantee for all HTTP response streaming sessions.
- 🪪 **T032 — Support TLS behind secure deployment configurations** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Certificate and proxy trust rules reviewed and exercised
- 🧫 **T033 — Test MCP HTTP inspector interoperability** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** End-to-end tool invocation under loopback and protected remote modes

### 🧾 P04 — MCP schema semantics — `██████░░░░ 57.1%`

**Feature:** MCP contract.

- 📐 **T034 — Validate structured input against exported schemas** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Tool arguments validated with stable user-facing errors
- 🧩 **T035 — Declare read-only versus mutating tool annotations** — `██████████ 100.0%`
  - ✅ **Gate:** Annotation write differs accurately from read tools · **Evidence:** src/main.rs: 21 #[tool] annotations including additive non-idempotent GA write; explicit in-memory metadata assertions. GitHub Actions run 40205211110 passed tests, strict rustfmt and Clippy.; src/main.rs: 21 #[tool] annotations including additive non-idempotent GA write; explicit in-memory metadata assertions. GitHub Actions run 40205211110 passed tests, strict rustfmt and Clippy.
- 🎛️ **T036 — Expose capability-profile filtered tool lists** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Default read-only excludes writes and scratchpad effects
- ♻️ **T037 — Honor deterministic pagination and tool list consistency** — `██████████ 100.0%`
  - ✅ **Gate:** Discovery pagination and list change tests pass · **Evidence:** src/http.rs authenticated_http_session_lists_and_calls_tools_but_rejects_invalid_sessions now repeats real MCP tools/list after initialization and asserts identical catalog ordering, no duplicates and no nextCursor when the complete 20-tool read-only catalog fits in a single response. No hidden multi-page cursor state is introduced.; src/http.rs performs two authenticated tools/list requests within one MCP session and verifies identical results/no spurious pagination. src/main.rs full_capability_profile_matches_exact_pinned_36_tool_union and scratchpad_tools_are_hidden_without_explicit_capability prove opt-in write/scratchpad profile differences and bounded complete inventories (20/35/36 tool names). GitHub Actions 40247290466 passed core and DuckDB. This accepts deterministic single-page discovery, not a fictitious paginated listing.
- ⚠️ **T038 — Normalize MCP protocol errors separately from GA failures** — `██████████ 100.0%`
  - ✅ **Gate:** Invalid tools are distinct from Google API errors · **Evidence:** src/http.rs json_rpc_protocol_errors_remain_separate_from_ga_validation_results runs a real authenticated MCP initialize, notifications/initialized, unknown JSON-RPC method and valid tools/call with invalid Google property. rmcp returns protocol method-not-found as JSON-RPC error -32601 while Synmetra returns a structured MCP tool result for GA argument validation. No provider call or Google credentials are required.; Synthetic loopback MCP HTTP test verifies unknown method returns top-level error.code=-32601 without result, but invalid run_report returns HTTP 200 JSON-RPC result with isError=true and structuredContent.error.category=validation and no top-level protocol error. Tests also enforce bearer token omission. GitHub Actions 40247087766 passed core tests, rustfmt, strict Clippy and native DuckDB.
- 🧵 **T039 — Propagate request cancellation across auth and HTTP** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Interrupted calls free resources
- 🧪 **T040 — Run cross-client MCP conformance suite** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Protocol wire fixtures pass multiple clients


## 🔑 M2 — Google authentication and credential isolation — `███░░░░░░░ 26.9%`

**Scope:** MVP. **Acceptance gate:** Secure, least-privilege, non-Python auth across interactive, local and service configurations; no secret leaks.

### 🪪 P05 — Native OAuth login and operators — `████░░░░░░ 43.8%`

**Feature:** Native OAuth.

- 🧭 **T041 — Implement auth login and auth status CLI** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** One-time login and status work without invoking Python or npm
- 🧰 **T042 — Implement auth doctor and login-command helper** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Operator gets actionable non-secret diagnostics
- 🌐 **T043 — Implement browser OAuth PKCE loopback flow** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** CSRF state, PKCE, timeout and redirect checks pass
- 🖥️ **T044 — Support headless copy-back OAuth setup** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Headless flow completes and rejects replayed callbacks
- 🔐 **T045 — Store tokens in GA4-specific user config storage** — `██████████ 100.0%`
  - ✅ **Gate:** Private file permissions and no shared ADC clobbering · **Evidence:** src/auth.rs isolated XDG credential path and atomic mode-0600 persistence; Actions 40202015488 compiled; file-mode tests pending; Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.
- ♻️ **T046 — Implement expiry-aware access-token refresh** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Refresh honors expiry and avoids parallel stampedes
- 📛 **T047 — Validate requested OAuth scopes per capability** — `██████████ 100.0%`
  - ✅ **Gate:** Read-only uses analytics.readonly; writes demand analytics.edit · **Evidence:** src/auth.rs authorization_url() requests only analytics.readonly; scope tests pass Actions 40202015488; write scope policy pending; src/auth.rs now requires explicit auth login --client-id-file PATH --allow-edit to request both analytics.readonly and analytics.edit, verifies both returned scopes, persists granted scope in mode-0600 private storage and keeps normal login readonly; tests pkce_authorization_is_readonly_and_state_bound and edit_scopes_require_explicit_consent_in_authorization_url passed core/feature CI 40222541416. Google's permissions remain independently enforced and live consent is unverified.
- 🔎 **T048 — Validate live token and quota-project configuration** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** API-enabled project and quota errors are distinguishable

### 📂 P06 — ADC, service account and token policy — `█░░░░░░░░░ 5.6%`

**Feature:** Google identities.

- 🔑 **T049 — Support explicit Google application credentials JSON** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Credential paths resolved without stdout token leakage
- 🪄 **T050 — Support ADC user credentials as opt-in** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Existing ADC loads without modifying global credentials
- 🤖 **T051 — Support service-account JWT token exchange** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Service identity token renews and errors correctly
- 🔗 **T052 — Support delegated and impersonated identities where valid** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Bound and documented Google subject/scopes
- 📨 **T053 — Support request-header upstream bearer tokens** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Client token is never persisted in server config
- 🔀 **T054 — Support request-header-or-config only on loopback** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Hosted fallback to server-held identity fails closed
- 🧮 **T055 — Support configured refresh-token credentials** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Explicit credentials refresh without CLI gcloud dependency
- 🏷️ **T056 — Support quota project and billing headers** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Expected Google quota routing preserved
- ⏳ **T057 — Guard clock skew and rotation races** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Expiring secrets renew safely during concurrent calls

### 🛡️ P07 — Inbound versus upstream trust — `███░░░░░░░ 33.3%`

**Feature:** Identity security.

- 🚪 **T058 — Separate inbound MCP identity from Google identity** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No identity-conflation in HTTP transport
- 🧾 **T059 — Define capability authorization matrices** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Google and local permissions checked per tool call
- 💼 **T060 — Avoid credential exposure in crash logs and traces** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Secret scanner fixtures remain clean
- 🔍 **T061 — Validate Google scope and role errors distinctly** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No broad-scope silent escalation on 403
- 🔐 **T062 — Protect local credential files from symlink attacks** — `██████████ 100.0%`
  - ✅ **Gate:** No world-readable files and atomic writes · **Evidence:** src/auth.rs private_file/read_credentials/persist_credentials reject symlinks and world-readable files; CI 40202015488 compilation; security tests pending; Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.
- 🚫 **T063 — Block SSRF via URLs in config or request arguments** — `██████████ 100.0%`
  - ✅ **Gate:** Only pinned Google API hosts are contacted by default · **Evidence:** src/google.rs hardcoded HTTPS Google API origins; property/account resource name sanitation; CI 40202015488 tests and fixed-host synthetic requests; src/google.rs GoogleClient::new pins analyticsadmin.googleapis.com and analyticsdata.googleapis.com via HTTPS with redirect policy none; production_google_hosts_are_pinned_and_http_redirects_never_followed verifies both hosts, rejects URL/authority/path injection in property/account identifiers, and checks an isolated 302 response is terminal rather than redirecting authorization headers. Only test-local with_endpoints can substitute loopback origins. GitHub Actions 40240180552 core Rust fmt, tests, Clippy, ledger and native DuckDB job passed; actual live Google identity/access remains a separate acceptance gate.
- 🧪 **T064 — Exercise expired revoked and invalid credentials** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All modes fail safely with stable errors
- 📦 **T065 — Ensure no auth dependency on gcloud installation** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Native test environment lacks gcloud Python and npm
- 🧯 **T066 — Implement safe logout and credential revocation guidance** — `██████████ 100.0%`
  - ✅ **Gate:** State cleared without touching unrelated Google apps · **Evidence:** src/auth.rs logout removes only Synmetra credential file after private-file check; Actions 40202015488 compiled; src/auth.rs clear_credentials_at deletes only private Synmetra credentials in a non-symlinked private auth directory, treats a missing file idempotently, refuses symlinked credentials/directories or world-readable credentials, and distinguishes unexpected filesystem errors. Unit test logout_removes_only_private_synmetra_credentials_without_touching_neighbors verifies removal, idempotence, no unrelated Google app data changes, and adversarial symlink/insecure-file rejection. Native CLI auth logout provides explicit Google-account revocation guidance without claiming server-side revocation. GitHub Actions 40244722883 core and DuckDB passed; no live Google session was accessed.


## 🏢 M3 — Admin API information parity — `██████░░░░ 56.2%`

**Scope:** MVP. **Acceptance gate:** All administrative read tools and metadata endpoints match pinned signatures and response behavior.

### 🗂️ P08 — Admin MCP tools — `██████████ 100.0%`

**Feature:** Seven Admin read tools.

- 🧾 **T067 — Implement get_account_summaries** — `██████████ 100.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for get_account_summaries · **Evidence:** src/main.rs implemented get_account_summaries MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs admin_handlers_preserve_golden_google_json_and_reject_403_without_leakage exercises get_account_summaries over isolated loopback HTTP with canonical mock Admin API JSON and HTTP 403 denial; verifies exact method/path, MCP structured response and stable GOOGLE_HTTP_ERROR/status 403 without the fixture bearer token. GitHub Actions 40227511758 core passed tests, fmt and strict Clippy. These synthetic fixtures do not certify authorized live Google or full upstream schema parity.
- 📃 **T068 — Implement get_account_data_sharing_settings** — `██████████ 100.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for get_account_data_sharing_settings · **Evidence:** src/main.rs implemented get_account_data_sharing_settings MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs admin_handlers_preserve_golden_google_json_and_reject_403_without_leakage exercises get_account_data_sharing_settings over isolated loopback HTTP with canonical mock Admin API JSON and HTTP 403 denial; verifies exact method/path, MCP structured response and stable GOOGLE_HTTP_ERROR/status 403 without the fixture bearer token. GitHub Actions 40227511758 core passed tests, fmt and strict Clippy. These synthetic fixtures do not certify authorized live Google or full upstream schema parity.
- 🏠 **T069 — Implement get_property_details** — `██████████ 100.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for get_property_details · **Evidence:** src/main.rs implemented get_property_details MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs admin_handlers_preserve_golden_google_json_and_reject_403_without_leakage exercises get_property_details over isolated loopback HTTP with canonical mock Admin API JSON and HTTP 403 denial; verifies exact method/path, MCP structured response and stable GOOGLE_HTTP_ERROR/status 403 without the fixture bearer token. GitHub Actions 40227511758 core passed tests, fmt and strict Clippy. These synthetic fixtures do not certify authorized live Google or full upstream schema parity.
- ⏳ **T070 — Implement get_property_data_retention_settings** — `██████████ 100.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for get_property_data_retention_settings · **Evidence:** src/main.rs implemented get_property_data_retention_settings MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs admin_handlers_preserve_golden_google_json_and_reject_403_without_leakage exercises get_property_data_retention_settings over isolated loopback HTTP with canonical mock Admin API JSON and HTTP 403 denial; verifies exact method/path, MCP structured response and stable GOOGLE_HTTP_ERROR/status 403 without the fixture bearer token. GitHub Actions 40227511758 core passed tests, fmt and strict Clippy. These synthetic fixtures do not certify authorized live Google or full upstream schema parity.
- 🔗 **T071 — Implement list_google_ads_links** — `██████████ 100.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for list_google_ads_links · **Evidence:** src/main.rs implemented list_google_ads_links MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs admin_handlers_preserve_golden_google_json_and_reject_403_without_leakage exercises list_google_ads_links over isolated loopback HTTP with canonical mock Admin API JSON and HTTP 403 denial; verifies exact method/path, MCP structured response and stable GOOGLE_HTTP_ERROR/status 403 without the fixture bearer token. GitHub Actions 40227511758 core passed tests, fmt and strict Clippy. These synthetic fixtures do not certify authorized live Google or full upstream schema parity.
- 📌 **T072 — Implement list_property_annotations** — `██████████ 100.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for list_property_annotations · **Evidence:** src/main.rs implemented list_property_annotations MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs admin_handlers_preserve_golden_google_json_and_reject_403_without_leakage exercises list_property_annotations over isolated loopback HTTP with canonical mock Admin API JSON and HTTP 403 denial; verifies exact method/path, MCP structured response and stable GOOGLE_HTTP_ERROR/status 403 without the fixture bearer token. GitHub Actions 40227511758 core passed tests, fmt and strict Clippy. These synthetic fixtures do not certify authorized live Google or full upstream schema parity.
- 🧬 **T073 — Implement get_custom_dimensions_and_metrics** — `██████████ 100.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for get_custom_dimensions_and_metrics · **Evidence:** src/main.rs implemented get_custom_dimensions_and_metrics MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs admin_handlers_preserve_golden_google_json_and_reject_403_without_leakage exercises get_custom_dimensions_and_metrics over isolated loopback HTTP with canonical mock Admin API JSON and HTTP 403 denial; verifies exact method/path, MCP structured response and stable GOOGLE_HTTP_ERROR/status 403 without the fixture bearer token. GitHub Actions 40227511758 core passed tests, fmt and strict Clippy. These synthetic fixtures do not certify authorized live Google or full upstream schema parity.

### 🏷️ P09 — Administrative edge cases — `██░░░░░░░░ 22.2%`

**Feature:** Admin fidelity.

- 📚 **T074 — Page through all account summaries and Google Ads links** — `██████████ 100.0%`
  - ✅ **Gate:** Multi-page provider fixtures return complete results · **Evidence:** src/google.rs: GoogleClient::collection caps accumulated results at 10,000 items / 16 MiB, rejects repeated or oversized cursors, and aggregates account summaries/Ads links in mocked multi-page fixtures. Tests account_pagination_collects_pages_and_rejects_replayed_tokens and account_collection_rejects_unbounded_item_count passed GitHub Actions core run 40218035242, 2026-10-25.; src/google.rs: GoogleClient::collection caps accumulated results at 10,000 items / 16 MiB, rejects repeated or oversized cursors, and aggregates account summaries/Ads links in mocked multi-page fixtures. Tests account_pagination_collects_pages_and_rejects_replayed_tokens and account_collection_rejects_unbounded_item_count passed GitHub Actions core run 40218035242, 2026-10-25.
- 🗺️ **T075 — Normalize property and account resource names** — `██████████ 100.0%`
  - ✅ **Gate:** Numeric and canonical IDs round-trip · **Evidence:** src/google.rs: scoped_name, property_name, account_name restrict and normalize positive resource IDs. Test rejects_property_path_and_zero passed GitHub Actions core 40218035242; complete account ID equivalence and live boundary QA remain open.; src/google.rs property_name and account_name normalize both positive bare numeric IDs and canonical properties/ or accounts/ resource names, rejecting path traversal, zero and malformed input. Test rejects_property_path_and_zero passed GitHub Actions core run 40218415190, 2026-10-25.
- 🧱 **T076 — Handle GA4 property account hierarchy and time zones** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Correct property selection and permissions
- ⏳ **T077 — Preserve property data retention and reset semantics** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Sparse optional values and defaults tested
- 🧩 **T078 — Support custom dimension and metric catalogs** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Custom-scoped names and reserved field handling correct
- 🏷️ **T079 — Support annotation listing and date intervals** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All pages and annotation color enums round-trip
- 🪪 **T080 — Handle GA Admin API beta versus alpha resources** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Version-specific host and payload shapes pinned
- 🚷 **T081 — Represent missing account permissions without data leaks** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Partial access errors are explicit and scoped
- 🔄 **T082 — Refresh account metadata safely on repeated calls** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No stale cross-principal cache confusion


## 📊 M4 — Reporting, preflight and Data API parity — `█████████░ 91.7%`

**Scope:** MVP. **Acceptance gate:** All report tools return correct GA4 semantics, including documented alpha, pagination and response projections.

### 📈 P10 — Report MCP tools — `██████████ 100.0%`

**Feature:** Eight report tools.

- 📊 **T083 — Implement run_report** — `██████████ 100.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_report · **Evidence:** src/main.rs implemented run_report MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs core_reporting_inputs_cover_pinned_contract_without_untracked_fields validates declared pinned argument names/required sets and deliberate keep_empty_rows union; historical_report_opt_in_projection_preserves_metadata_and_previews_offline and historical_cursor_pages_remain_query_bound_and_never_repeat_rows validate Google request paths, golden row/metadata and cursor behavior; historical_and_realtime_report_denials_are_redacted_and_stable verifies HTTP 403/429 errors without bearer leaks. GitHub Actions 40234385460 core passed tests, fmt, strict Clippy, ledger. This does not certify live Google or every upstream field representation.
- 🛒 **T084 — Implement run_conversions_report** — `██████████ 100.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_conversions_report · **Evidence:** src/main.rs implemented run_conversions_report MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs conversion_spec_schema_advertises_pinned_action_and_attribution_fields and core_reporting_inputs_cover_pinned_contract_without_untracked_fields test typed conversion_spec/action and full pinned field inventory, required fields and quota default; conversion_report_opt_in_projection_uses_alpha_endpoint_and_bounded_response verifies the v1alpha request and golden projected response; conversion_report_denial_and_invalid_spec_fail_without_sensitive_output verifies offline malformed-action rejection and redacted HTTP 403. GitHub Actions 40234385460 core passed. Upstream field-format exactness and authorized live GA remain distinct gates.
- 🚦 **T085 — Implement run_funnel_report** — `██████████ 100.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_funnel_report · **Evidence:** src/main.rs implemented run_funnel_report MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs run_funnel_report forwards pinned optional arguments with typed step, breakdown, next-action and visualization schemas; src/google.rs funnel_request/funnel_request_options enforce bounded, read-only Google v1alpha request normalization including open/trended settings and timing. funnel_subreports_are_projected_independently_with_open_funnel_options covers local golden table/visualization, quota, summary/row limits; funnel_invalid_arguments_and_google_permission_denials_are_redacted confirms invalid preflight and HTTP 403 redaction; typed funnel schema assertion confirms declared fields. GitHub Actions 40236619627 core Rust tests, fmt, strict Clippy, ledger passed. No claim of authorized live Google or complete SednaLabs projection parity.
- ⚡ **T086 — Implement run_realtime_report** — `██████████ 100.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_realtime_report · **Evidence:** src/main.rs implemented run_realtime_report MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs core_reporting_inputs_cover_pinned_contract_without_untracked_fields and realtime_official_optional_arguments_are_declared_and_ordering_is_forwarded verify MCP field inventories and ordering; realtime_mcp_retains_provider_quota_without_fabricating_a_next_cursor verifies golden Google response, quota and no fabricated cursor; realtime_nonzero_offset_has_stable_preflight_error plus historical_and_realtime_report_denials_are_redacted_and_stable verify unsupported offset and HTTP 403/429 category/status without secrets. GitHub Actions 40234385460 core passed. Official live Google and all-tool contract parity are separately unaccepted.
- 🧩 **T087 — Implement run_pivot_report** — `██████████ 100.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_pivot_report · **Evidence:** src/main.rs implemented run_pivot_report MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs pivot_input_fields_and_required_sets_match_pinned_contract compares all pinned input fields, required set and quota default to the native MCP router; pivot_report_projection_keeps_pivot_header_metadata_without_fake_cursor uses synthetic Google v1beta pivot HTTP response to verify request routing, opt-in rows, true totals, original pivotHeaders under meta.ga4, and nonfabricated next_cursor; core_preflight_and_batch_reject_invalid_input_before_google_calls tests missing pivots; pivot_report_provider_denial_excludes_http_body_and_bearer verifies stable redacted Google HTTP 403. GitHub Actions 40235171004 core passed tests, fmt and strict Clippy. Real Google acceptance, pivot-axis pagination and all-upstream response parity remain open.
- 📦 **T088 — Implement batch_run_reports** — `██████████ 100.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for batch_run_reports · **Evidence:** src/main.rs implemented batch_run_reports MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs batch_input_contract_requires_only_property_and_report_requests verifies the exact pinned outer MCP properties/required fields; BatchReportItemArgs safely normalizes snake_case request items to Google camelCase; batch_reports_normalize_separate_requests_and_preserve_distinct_results mock confirms distinct golden results; batch_report_five_item_boundary_is_enforced_before_google_credentials tests one-to-five bounds; batch_report_permission_and_quota_denials_do_not_leak_provider_details validates HTTP 403/429, stable errors and secret redaction; src/google.rs batch_reports_reject_missing_or_extra_provider_results rejects inconsistent Google success arrays. GitHub Actions 40234680391 core passed tests, formatting, strict Clippy. This does not imply 36-tool parity or authorized live Google tests.
- 🔎 **T089 — Implement run_property_access_report** — `██████████ 100.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_property_access_report · **Evidence:** src/main.rs implemented run_property_access_report MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs admin_access_mcp_inputs_cover_pinned_fields_and_required_sets verifies complete SednaLabs property access MCP input inventory, required set, and uint64 limit. admin_access_reports_project_named_fields_and_redact_google_denials uses local Admin API HTTP mocks for both resource types to test POST routing, GA dimensionName/metricName fields, stable golden rows, bounded totals and v1 continuation, and redacted HTTP 403 provider errors. src/tabular.rs admin_access_headers_and_resource_boundaries_have_distinct_cursors verifies account/property cursor fingerprints cannot be reused across resource types. GitHub Actions 40235752963 core passed formatting, tests, strict Clippy. Admin privileges and live Google acceptance remain separate.
- 👥 **T090 — Implement run_account_access_report** — `██████████ 100.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_account_access_report · **Evidence:** src/main.rs implemented run_account_access_report MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/main.rs admin_access_mcp_inputs_cover_pinned_fields_and_required_sets verifies complete SednaLabs account access MCP input inventory, required set, and uint64 limit. admin_access_reports_project_named_fields_and_redact_google_denials uses local Admin API HTTP mocks for both resource types to test POST routing, GA dimensionName/metricName fields, stable golden rows, bounded totals and v1 continuation, and redacted HTTP 403 provider errors. src/tabular.rs admin_access_headers_and_resource_boundaries_have_distinct_cursors verifies account/property cursor fingerprints cannot be reused across resource types. GitHub Actions 40235752963 core passed formatting, tests, strict Clippy. Admin privileges and live Google acceptance remain separate.

### 🧭 P11 — Preflight MCP tools — `██████░░░░ 64.3%`

**Feature:** Preflight and metadata.

- 🧭 **T091 — Implement preview_report_request** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Validate its contract and unsupported metrics without wasteful report requests
- ✅ **T092 — Implement check_report_compatibility** — `██████████ 100.0%`
  - ✅ **Gate:** Validate its contract and unsupported metrics without wasteful report requests · **Evidence:** src/main.rs implemented check_report_compatibility MCP handler plus src/google.rs request route/validation; Rust unit and in-memory MCP tests passed Actions 40202015488; full upstream schema/live GA parity remains open; src/google.rs compatibility_request builds the dedicated Google Data API checkCompatibility body without inventing report date ranges or requiring metrics; property 0 is allowed only for checkCompatibility. src/main.rs compatibility_probe_uses_property_zero_without_a_report_call mock verifies correct POST endpoint, empty inventory request, compatibility reason metadata and secret-free response, plus offline rejection of invalid fields; src/google.rs compatibility_requests_allow_inventory_or_metric_only_without_report_dates tests dimension-only, metric-only, inventory and invalid values. GitHub Actions 40232788668 core Rust tests, fmt and Clippy passed. This does not certify authorized live property-specific metadata or all upstream enriched response adapters.
- 🧬 **T093 — Cache and refresh Data API metadata safely** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Metadata cache keys include property and caller
- 🔤 **T094 — Support flexible dimension and metric filters** — `██████████ 100.0%`
  - ✅ **Gate:** FilterExpression nested JSON and shorthand cases validated · **Evidence:** src/google.rs: structured nested dimensionFilter and metricFilter pass unchanged through report_request; bounded recursive validation and valid nested AND/NOT fixture passed 2026-10-25 GitHub Actions core job 121238774791 (run 40216536190). Shorthand filters and live GA4 contract parity still unverified.; src/google.rs; GitHub Actions core job 121240177425 passed nested and shorthand FilterExpression fixtures, 2026-10-25.
- 📅 **T095 — Validate date ranges and reserved dates** — `██████████ 100.0%`
  - ✅ **Gate:** Multiple ranges and dates behave as Google expects · **Evidence:** src/google.rs: normalize_date_ranges validates ISO calendar dates and relative date forms, rejects reversed, unknown or invalid ranges and canonicalizes snake_case, used by historical, funnel and Admin report requests. Synthetic tests date_ranges_preserve_relative_dates_and_normalize_snake_case and date_ranges_reject_invalid_dates_and_unknown_fields passed GitHub Actions core run 40212316165; provider-wide acceptance remains open.; src/google.rs four_historical_date_ranges_round_trip_and_a_fifth_is_rejected verifies four independently named/relative/absolute/leap-day intervals normalize and round-trip correctly, fifth range and invalid/reversed/duplicate-alias/empty-name inputs fail with INVALID_REQUEST. Earlier date_ranges_preserve_relative_dates_and_normalize_snake_case and date_ranges_reject_invalid_dates_and_unknown_fields extend the cases; GitHub Actions 40228846409 core and DuckDB jobs passed. Authorized live Google Data API validation remains a separate acceptance gate.
- 🧮 **T096 — Implement metric and dimension compatibility checks** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Invalid combinations handled before expensive calls
- 📝 **T097 — Normalize input values without silent coercion** — `██████████ 100.0%`
  - ✅ **Gate:** Lossless valid requests and typed invalid results · **Evidence:** src/google.rs: canonical normalization of dates and typed invalid-request rejection before Google API calls; synthetic unit tests passed GitHub Actions core 40212316165. Full filter and pivot coercion and live API parity remain open.; src/google.rs normalized_report_body_preserves_valid_inputs_and_rejects_ambiguous_values round-trips relative and leap-day dates, nested dimension filters, exact signed-64 metric values, ordering, dimensions and decimal string limits without silent coercion; invalid overflow, non-finite numbers, nonnumeric filter inputs and ambiguous date aliases yield typed INVALID_REQUEST. numeric_metric_filter_preserves_i64_edges_and_rejects_overflow adds signed boundary coverage. GitHub Actions 40237203370 passed core Rust tests, fmt, strict Clippy and native DuckDB. This does not certify Google's semantic validity for arbitrary field names.

### 📐 P12 — Data request fidelity and limits — `██████████ 100.0%`

**Feature:** Reporting correctness.

- 🧮 **T098 — Support metric aggregates and row counts** — `██████████ 100.0%`
  - ✅ **Gate:** Totals minimum maximum and aggregate fields preserved · **Evidence:** src/tabular.rs project explicitly preserves provider totals, minimums, maximums, aggregates and propertyQuota unmodified in meta.ga4, while retaining row_count_total and row_count_total_known and keeping ordinary rows separate from aggregate rows. Default native run_report continues returning provider-shaped JSON. GitHub Actions 40236868925 full core+DuckDB CI passed.; src/tabular.rs provider_aggregate_rows_survive_all_projection_modes_without_recomputation checks all four output modes and summary-only against fixed non-integer revenue strings and original GA totals/minimums/maximums/aggregates, truthful total=18 and returned=1, and excludes fabricated aggregates when absent. CI 40236868925 passed Rust tests, canonical fmt, strict Clippy, ledger and native DuckDB job. Live Google and complete response parity remain independent.
- 📎 **T099 — Support pagination offsets and opaque cursors** — `██████████ 100.0%`
  - ✅ **Gate:** Continuation is stable with bounded row limits · **Evidence:** src/tabular.rs implements bounded SHA-256 query-bound v1 continuation cursor codec, normalized property, canonical report query without pagination parameters, offset bound of 10 million, and cursor+offset conflict/mismatch rejection; src/main.rs uses it in run_report and offline preview, realtime cursor remains explicitly rejected. Tests historical_cursor_round_trip_enforces_query_hash_and_bounded_offset passed GitHub Actions 40231599082 core Rust CI.; src/main.rs historical_cursor_pages_remain_query_bound_and_never_repeat_rows runs sequential local mock Google pages, verifies the first report returns v1 next_cursor, the next request sends Google offset=1, second report returns a distinct row with no further token, and changed query with reused cursor fails before network access. Core GitHub Actions 40231599082 passed formatting, tests, strict Clippy, ledger. This does not guarantee snapshot isolation if Google data changes, nor establish full SednaLabs parity or live Google acceptance.
- ⚡ **T100 — Honor realtime non-paginated semantics** — `██████████ 100.0%`
  - ✅ **Gate:** No fake continuation token when GA realtime lacks offset · **Evidence:** src/main.rs RealtimeArgs supports ordering and official offset input; realtime_report_request rejects nonzero offset with a stable UNSUPPORTED_REALTIME_OFFSET before contacting Google because the current Google Data API v1beta RunRealtimeReportRequest has no offset field. Test realtime_nonzero_offset_has_stable_preflight_error passed GitHub Actions 40227043192 core Rust checks.; src/main.rs realtime_mcp_retains_provider_quota_without_fabricating_a_next_cursor uses local synthetic Google HTTP response of rowCount=154 and limit=1; tests verify both data and meta have no fabricated next_cursor while retaining original rowCount. GitHub Actions 40227043192 core job passed. This task is narrow: general historical report cursors and live GA parity remain separate.
- 🗂️ **T101 — Support pivot report ordering and groups** — `██████████ 100.0%`
  - ✅ **Gate:** Nested pivots and row groups validated by fixtures · **Evidence:** src/google.rs normalize_pivots now enforces 1–4 Google v1beta pivot axes, nonempty requested dimension fieldNames, required positive per-axis limits and bounded product <=250,000; accepts numeric/string offset/limit and normalizes to Google int64 strings, rejects invalid dimension/metric orderings and conflicting sort selectors. src/main.rs run_pivot_report uses it before Google calls; tests multi_axis_pivot_ordering_and_cardinality_are_bounded passed Actions 40238367789 full CI.; src/tabular.rs multi_axis_pivot_groups_preserve_headers_and_never_claim_flat_cursors uses grouped synthetic country/browser rows with two pivotHeaders, verifies all row values, original pivot header grouping, true totals, truncated bounds and no fabricated flat cursor. Existing pivot_report_projection_keeps_pivot_header_metadata_without_fake_cursor exercises read-only Google v1beta HTTP routing. GitHub Actions 40238367789 core+DuckDB passed. Live Google acceptance remains separate.
- 📦 **T102 — Enforce batch report maximums and isolation** — `██████████ 100.0%`
  - ✅ **Gate:** Five-report constraint and distinct report results preserved · **Evidence:** src/google.rs data_operation rejects batches with 0 or >5 requests; Actions 40202015488 passed guards; src/main.rs BatchReportItemArgs typed snake_case-to-Google normalization and batch_reports_normalize_separate_requests_and_preserve_distinct_results mock HTTP test preserve distinct first/second reports without cross-contamination; batch_report_five_item_boundary_is_enforced_before_google_credentials proves empty and six requests fail with INVALID_REQUEST while exactly five proceed to missing credentials; google.rs rejects_invalid_batch_without_http_request guards counts. GitHub Actions 40228564460 core tests, formatting and Clippy passed. Full SednaLabs batch output controls and live Google parity remain open on T088/T155 follow-up.
- 🧪 **T103 — Support funnel steps segments trended and breakdown** — `██████████ 100.0%`
  - ✅ **Gate:** Complex v1alpha funnel shapes round-trip · **Evidence:** src/google.rs funnel_request and funnel_request_options normalize v1alpha event or filter expression steps, prior-step sequencing/duration, up to four segments, breakdown and next-action controls, open/trended visualization, quota flag and bounded dimension filtering; src/main.rs run_funnel_report calls the v1alpha Google endpoint. Complex funnel fixture and API mock tests run in GitHub Actions 40239266063 with both core and DuckDB jobs passing.; src/google.rs complex_funnel_steps_segments_breakdown_and_trended_options_round_trip, funnel_steps_preserve_sequencing_durations_and_bounded_breakdown_limits, and funnel_nullable_optionals_and_unknown_fields_have_explicit_semantics verify multi-step v1alpha JSON round-trip, null handling, aliases, invalid first-step sequencing, malformed durations, and conflicting or unknown fields. GitHub Actions 40239266063 passed. This gate does not certify live property eligibility or all SednaLabs response semantics.
- 🛒 **T104 — Support conversion and attribution reporting v1alpha** — `██████████ 100.0%`
  - ✅ **Gate:** Eligible and ineligible properties/errors tested · **Evidence:** src/google.rs conversion_spec normalizes and bounds supported v1alpha attribution models and conversion metrics; src/main.rs run_conversions_report calls the Google v1alpha runReport endpoint with conversionSpec and optional bounded tabular projection/cursor. Test conversion_report_opt_in_projection_uses_alpha_endpoint_and_bounded_response asserts a synthetic eligible response and exact endpoint. GitHub Actions 40240559437 core CI passed; native DuckDB job was still in progress at acceptance time.; src/main.rs conversion_eligibility_and_permission_errors_remain_typed_and_private tests simulated provider-specific 400 eligibility and 403 permission denials, preserving status and upstream category while redacting token and private error body; conversion_report_denial_and_invalid_spec_fail_without_sensitive_output checks validation failure before network and access denial. Mock eligible v1alpha 200 response tested separately. GitHub Actions 40240559437 core Rust tests, rustfmt and Clippy passed. No live Google eligibility or account authorization claim; separate M8 live tests remain open.
- 💵 **T105 — Preserve currency timezone and numeric precision** — `██████████ 100.0%`
  - ✅ **Gate:** Typed metrics and currency metadata retained · **Evidence:** src/main.rs forwards caller-supplied currency_code as Google currencyCode in native standard, conversion and pivot requests, and forwards time_zone to Admin access report requests; src/google.rs access_report_request validates IANA-style zone syntax and report_currency_request_remains_exact_and_numeric_filter_is_not_rounded checks currency request mapping. src/tabular.rs retains provider metadata including currencyCode/timeZone and exact TYPE_CURRENCY column header type. CI 40237637016 passed core+native DuckDB.; src/tabular.rs numeric_metric_precision_and_currency_metadata_survive_text_clipping reproduces loss of precision with long decimal metric strings under max_cell_chars, then verifies metric values are now kept byte-for-byte while Unicode dimension labels are clipped and meta.ga4.metadata retains currencyCode MXN and timeZone America/Mexico_City; meta.columns[*].ga4_type preserves TYPE_CURRENCY. src/google.rs access_reports_use_admin_field_names_and_reject_invalid_ranges tests forwarded timeZone. GitHub Actions 40237637016 passed fmt, Rust tests, strict Clippy, progress validation and DuckDB. No licensed live GA property validation is inferred.
- 📊 **T106 — Support output modes rows tuples scalar compact** — `██████████ 100.0%`
  - ✅ **Gate:** All compatible result projections are predictable · **Evidence:** src/tabular.rs OutputMode and project implement rows/tuples/scalar/compact plus summary_only and shared typed columns; src/main.rs report-MCP handlers opt into projection while preserving official provider JSON by default. all_compatible_ga_report_mcp_tools_advertise_four_tabular_output_modes verifies all seven core/advanced/Admin reporting MCP schema entries expose output_mode, with valid four and rejected unknown enum labels. GitHub Actions 40238367789 passed core+native.; src/tabular.rs projections_keep_true_counts_and_upstream_quota, provider_aggregate_rows_survive_all_projection_modes_without_recomputation and bounded_projection_limits_are_explicit_in_all_tabular_modes assert predictable mode-specific row/tuple/scalar/compact data, counts and unmodified provider aggregate/quota metadata. Funnel projection tests verify two independent subreports. GitHub Actions 40238367789 full CI passed; this is not 36-tool upstream response parity.
- ✂️ **T107 — Support row cell and request-size clipping** — `██████████ 100.0%`
  - ✅ **Gate:** Limits protect context without hiding truncation metadata · **Evidence:** src/tabular.rs Controls validate max_rows 1–1000 and max_cell_chars 1–2048, bounds Google report limit, clips Unicode dimension values while preserving numeric metric strings, and exposes row_count_total, row_count_returned, truncated, cell_clipping metadata. src/google.rs report_request and GoogleClient::validate_body enforce the entire 64-KiB request preflight and send caps the upstream response at 4 MiB. GitHub Actions 40238367789 core+DuckDB passed.; src/tabular.rs bounded_projection_limits_are_explicit_in_all_tabular_modes verifies row and clip caps, returned=2 of total=7, clipped_cells=2, truncation and unmodified provider data across four output modes and summary_only. numeric_metric_precision_and_currency_metadata_survive_text_clipping tests large exact metrics are never silently shortened; src/google.rs historical_report_request_rejects_oversized_options_and_deep_filters checks large nested request denial. GitHub Actions 40238367789 full CI passed. Other-tool source-specific projections remain separate.
- 📋 **T108 — Support summary-only responses and total known flags** — `██████████ 100.0%`
  - ✅ **Gate:** Counts and partial results are never misrepresented · **Evidence:** src/tabular.rs provides opt-in summary_only responses with data=null, explicit row_count_total_known flag, truthful row_count_returned/truncated and no fabricated next_cursor; src/main.rs wires this to run_report/run_realtime_report and preview. Tests projections_keep_true_counts_and_upstream_quota, summary_and_unicode_clipping_preserve_counts_without_row_payload, unknown_total_is_marked_unknown_and_metric_only_reports_are_valid plus mock integration passed GitHub Actions 40230044045 full CI. Remaining all-tool output controls and stable historical cursors are open.; src/tabular.rs historical_last_page_is_not_marked_truncated_and_accepts_string_totals fixes the verified last-page offset bug, distinguishes string provider rowCount from unknown totals, keeps returned-count honest and suppresses continuation only at true final page. summary_and_unicode_clipping_preserve_counts_without_row_payload verifies summary_only data null with accurate metadata, unknown_total_is_marked_unknown_and_metric_only_reports_are_valid marks uncertainty, and historical_cursor_pages_remain_query_bound_and_never_repeat_rows checks nonduplicating pages. GitHub Actions 40237203370 passed core+DuckDB. All 36 tools' complete Contract V1 projections remain a separate gate. · **Evidence (partial):** src/tabular.rs provides opt-in summary_only responses with data=null, explicit row_count_total_known flag, truthful row_count_returned/truncated and no fabricated next_cursor; src/main.rs wires this to run_report/run_realtime_report and preview. Tests projections_keep_true_counts_and_upstream_quota, summary_and_unicode_clipping_preserve_counts_without_row_payload, unknown_total_is_marked_unknown_and_metric_only_reports_are_valid plus mock integration passed GitHub Actions 40230044045 full CI. Remaining all-tool output controls and stable historical cursors are open.
- 🌍 **T109 — Handle property-specific API permission and quota errors** — `██████████ 100.0%`
  - ✅ **Gate:** 429 and 403 retry/backoff rules explicit · **Evidence:** src/google.rs readonly_retry_delay rejects 403 and ordinary 4xx, supports bounded idempotent 429/500/502/503/504 retries; src/main.rs batch_report_permission_and_quota_denials_do_not_leak_provider_details and historical_and_realtime_report_denials_are_redacted_and_stable verify redacted 403/429 HTTP codes. docs/NATIVE_MCP.md explicitly states rules. GitHub Actions 40240180552 full core and DuckDB CI passed.; src/google.rs readonly_retry_policy_honors_backoff_limits_and_never_retries_forbidden checks 403 terminal, numeric Retry-After minimum and excessive/malformed Retry-After terminal; readonly_report_retry_is_bounded_and_never_replays_writes validates 429 then 200, and annotation_writes_do_not_retry_429_500_or_503_responses verifies no write replay. Does not claim live Google quota validation.
- 🧱 **T110 — Bound oversized filter recursion and JSON arguments** — `██████████ 100.0%`
  - ✅ **Gate:** Worst-case requests rejected before Google API · **Evidence:** src/google.rs validate_bounded_filter: 16 KiB encoded bound, max depth 12, max 256 nodes, 64 children, and 1024-byte strings. Test nested_ga_filter_preserved_and_excessive_structures_rejected passed 2026-10-25 GitHub Actions core job 121238774791 (run 40216536190), alongside default Rust MCP unit tests and Clippy. More hostile corpus and live API acceptance pending.; src/google.rs report_request validates the entire assembled body (64 KiB cap), plus existing bounded filter depth/nodes/breadth limits; src/main.rs offline_preview_rejects_oversized_report_options_with_a_typed_error confirms malformed options yield INVALID_REQUEST without requiring Google credentials or network access; google.rs historical_report_request_rejects_oversized_options_and_deep_filters verifies massive orderBy and recursive/wide filters. GitHub Actions 40228133861 core tests, rustfmt and Clippy passed. Independent live-Google and all-upstream schema acceptance remain separate.
- ♻️ **T111 — Make repeated report retries safe and idempotent** — `██████████ 100.0%`
  - ✅ **Gate:** Retry-After respected with bounded jitter · **Evidence:** src/google.rs send_readonly supports at most three attempts for clonable idempotent GA read requests, while send uses exactly one attempt for annotation mutations. readonly_retry_delay uses 200/400ms exponential backoff, 0–100ms jitter, numeric Retry-After minimum up to 5 seconds, with over-budget delays ending in an HTTP error. docs/NATIVE_MCP.md documents limitations. Full GitHub Actions 40240180552 green.; src/google.rs readonly_report_retry_is_bounded_and_never_replays_writes confirms an authenticated historical report recovers from HTTP 429 Retry-After:0 using exactly a second successful mock request; readonly_retry_policy_honors_backoff_limits_and_never_retries_forbidden tests bounded jitter, long delay skip and HTTP 403 no retry. Annotation tests guard non-idempotent writes. Full CI 40240180552 passed.
- 🧾 **T112 — Preserve GA response quota metadata and warnings** — `██████████ 100.0%`
  - ✅ **Gate:** Quotas accessible without leaking tokens · **Evidence:** src/main.rs result preserves the provider JSON data without dropping propertyQuota and never echoes the access token; realtime_mcp_retains_provider_quota_without_fabricating_a_next_cursor verifies quota remaining=9 and absence of the mock bearer token in MCP structured output. CI 40227043192 core passed. Structured warning metadata and full cross-tool quota/error behavior remain unaccepted.; src/main.rs default provider-shaped result retains Google propertyQuota; src/tabular.rs opt-in output retains provider propertyQuota and warnings inside meta.ga4, with source token excluded from structured/text content. Tests realtime_mcp_retains_provider_quota_without_fabricating_a_next_cursor, historical_report_opt_in_projection_preserves_metadata_and_previews_offline, projections_keep_true_counts_and_upstream_quota confirm quota counts and warnings. GitHub Actions 40230044045 both core and DuckDB jobs passed. This is metadata pass-through acceptance, not a claim of all 36 upstream response projections. · **Evidence (quota only):** src/main.rs result preserves the provider JSON data without dropping propertyQuota and never echoes the access token; realtime_mcp_retains_provider_quota_without_fabricating_a_next_cursor verifies quota remaining=9 and absence of the mock bearer token in MCP structured output. CI 40227043192 core passed. Structured warning metadata and full cross-tool quota/error behavior remain unaccepted.


## ✍️ M5 — Explicitly gated GA mutation parity — `███████░░░ 73.1%`

**Scope:** MVP. **Acceptance gate:** One official write tool operates with analytics.edit and explicit opt-in; ordinary mode cannot mutate GA.

### 📝 P13 — Annotation creation parity — `████████░░ 75.0%`

**Feature:** One opt-in Google write tool.

- 🖊️ **T113 — Implement create_property_annotation** — `██████████ 100.0%`
  - ✅ **Gate:** Full official title description dates and color schema contract tested · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; src/main.rs AnnotationColorInput exposes six valid user annotation colors, excluding reserved ORANGE. annotation_schema_and_payload_cover_default_color_and_single_or_range_dates validates required/optional MCP inputs, default BLUE, exact GA Admin reportingDataAnnotation single-date versus ranged-date objects, and description. src/google.rs annotation_payload enforces title/description length, valid date order and supported colors. GitHub Actions 40241002371 core + DuckDB jobs passed. Live write approval remains a separate M8 gate.
- 📅 **T114 — Validate annotation date and range order** — `██████████ 100.0%`
  - ✅ **Gate:** Reject malformed dates and reversed ranges before Google call · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.
- 🎨 **T115 — Reject reserved ORANGE and invalid enum colors** — `██████████ 100.0%`
  - ✅ **Gate:** Only supported annotation colors submitted to Google · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.
- 📝 **T116 — Validate maximum title and description lengths** — `██████████ 100.0%`
  - ✅ **Gate:** 60-character title and 150-character description bounds respected · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.
- 🔐 **T117 — Require explicit allow-write startup capability** — `██████████ 100.0%`
  - ✅ **Gate:** Default tools/list and tools/call cannot write · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.
- 🪪 **T118 — Demand analytics.edit scope and sufficient property role** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Clear denial without escalating OAuth permissions
- ✋ **T119 — Require user confirmation at client boundary when supported** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** High-impact writes annotated and host approval guidance explicit
- 🧪 **T120 — Test write denial and optional live opt-in acceptance** — `██████████ 100.0%`
  - ✅ **Gate:** No writes in automated public CI · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; .github/workflows/ci.yml has contents:read and runs only cargo fmt, tests, Clippy, release checks and progress validator: no Google credentials/secrets, no allow-write live server, and no production POSTs. src/main.rs disables mutations by default; annotation mock tests use localhost Google endpoints, not live Google. GitHub Actions 40241002371 full core + DuckDB jobs passed. Authorized optional live annotations remain scoped to M8 T184.

### 🚦 P14 — Write safety policies — `███████░░░ 70.0%`

**Feature:** Write gating.

- 🏷️ **T121 — Annotate mutating MCP tools correctly** — `██████████ 100.0%`
  - ✅ **Gate:** ReadOnly and idempotence hints match effects · **Evidence:** src/main.rs: 21 #[tool] annotations including additive non-idempotent GA write; explicit in-memory metadata assertions. GitHub Actions run 40205211110 passed tests, strict rustfmt and Clippy.; src/main.rs: 21 #[tool] annotations including additive non-idempotent GA write; explicit in-memory metadata assertions. GitHub Actions run 40205211110 passed tests, strict rustfmt and Clippy.
- 🛡️ **T122 — Keep write tool hidden under read-only profiles** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Bypass attempts fail regardless of tool visibility
- 🧾 **T123 — Produce redacted audit events for writes** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Timestamp and action identifiable but no secrets
- ♻️ **T124 — Handle Google transient failures without blind retries** — `██████████ 100.0%`
  - ✅ **Gate:** No automatic duplicate annotation creation · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; src/google.rs create_annotation invokes self.send once, never an automatic retry. Test annotation_writes_do_not_retry_429_500_or_503_responses uses a single local loopback request with Retry-After: 1 for each transient status and checks immediate error and no leaked token, while annotation_post_targets_admin_v1alpha_without_retry verifies one successful POST. GitHub Actions 40239266063 and 40239414715 both passed core and DuckDB jobs. This does not claim future retry/backoff policy for read-only reporting (T109/T111).
- 🧯 **T125 — Support reversible config opt-out without token replacement** — `██████████ 100.0%`
  - ✅ **Gate:** Disable write capability at runtime restart · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.


## 🗃️ M6 — DuckDB scratchpad and advanced analytics parity — `████░░░░░░ 40.0%`

**Scope:** MVP. **Acceptance gate:** All 15 local scratchpad tools and policy limits operate without unsafe SQL or credential disclosure.

### 🗄️ P15 — Scratchpad lifecycle MCP tools — `█████░░░░░ 50.0%`

**Feature:** Seven session tools.

- 🧱 **T126 — Implement scratchpad_open_session** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_open_session
- 🔒 **T127 — Implement scratchpad_close_session** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_close_session
- 📋 **T128 — Implement scratchpad_list_sessions** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_list_sessions
- 📏 **T129 — Implement scratchpad_get_runtime_limits** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_get_runtime_limits
- 🎛️ **T130 — Implement scratchpad_set_runtime_limits** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_set_runtime_limits
- 🗂️ **T131 — Implement scratchpad_list_tables** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_list_tables
- 🧹 **T132 — Implement scratchpad_drop_table** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_drop_table

### 📥 P16 — Scratchpad ingest and query MCP tools — `█████░░░░░ 50.0%`

**Feature:** Five ingest/query tools.

- 📊 **T133 — Implement scratchpad_ingest_report** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Synthetic GA payload and DuckDB query conformance for scratchpad_ingest_report
- ⚡ **T134 — Implement scratchpad_ingest_realtime_report** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Synthetic GA payload and DuckDB query conformance for scratchpad_ingest_realtime_report
- 🔎 **T135 — Implement scratchpad_query** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Synthetic GA payload and DuckDB query conformance for scratchpad_query
- 🧬 **T136 — Implement scratchpad_describe_table** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Synthetic GA payload and DuckDB query conformance for scratchpad_describe_table
- 🧮 **T137 — Implement scratchpad_summarize_table** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Synthetic GA payload and DuckDB query conformance for scratchpad_summarize_table

### 📉 P17 — Scratchpad analysis MCP tools — `███░░░░░░░ 33.3%`

**Feature:** Three analytic tools.

- 📉 **T138 — Implement scratchpad_release_regression_report** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Evidence fixture and schema/output equivalence for scratchpad_release_regression_report
- 🔀 **T139 — Implement scratchpad_landing_param_shift_report** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Evidence fixture and schema/output equivalence for scratchpad_landing_param_shift_report
- 📤 **T140 — Implement scratchpad_export_evidence_bundle** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Evidence fixture and schema/output equivalence for scratchpad_export_evidence_bundle

### 🔐 P18 — DuckDB security and resource controls — `███░░░░░░░ 30.0%`

**Feature:** Scratchpad sandbox.

- 🚪 **T141 — Gate scratchpad behind explicit capability profile** — `██████████ 100.0%`
  - ✅ **Gate:** Default read-only process has no scratchpad tools · **Evidence:** src/main.rs and src/scratchpad.rs; GitHub Actions run 40206168306 successfully completed bundled-DuckDB feature tests (initial seven-route implementation); full strict-Clippy/updated-parameter parity and live GA tests remain separate acceptance gates.; src/main.rs and src/scratchpad.rs; GitHub Actions run 40206168306 successfully completed bundled-DuckDB feature tests (initial seven-route implementation); full strict-Clippy/updated-parameter parity and live GA tests remain separate acceptance gates.
- 🧩 **T142 — Enforce per-session quotas and retention** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Memory disk time rows and session count bounded
- 🗃️ **T143 — Isolate scratchpad database files per identity** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Cross-user table read attempts fail
- 🪬 **T144 — Implement SQL SELECT-only parser policy** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** DML DDL COPY INSTALL LOAD ATTACH and external scans rejected
- 🌍 **T145 — Disable remote extensions and filesystem reads** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** No network or arbitrary path escape through DuckDB
- 🧯 **T146 — Handle unexpected termination and session cleanup** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Leftover temp files sanitized on restart
- 📏 **T147 — Provide deterministic truncation and evidence limits** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Bundle size and context limits enforced
- 🔏 **T148 — Guard exported evidence bundle permissions** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Exports do not overwrite arbitrary paths or expose secrets
- 🧪 **T149 — Run embedded DuckDB fuzz and hostile SQL fixtures** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Policy bypass corpus fails safely
- 🦀 **T150 — Document native DuckDB library boundary honestly** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Dependency license architecture and portability reviewed


## 🧬 M7 — Tool contract interoperability and parity audits — `████░░░░░░ 38.2%`

**Scope:** MVP. **Acceptance gate:** All 36 tool schemas and response mappings are independently snapshot-tested against both pinned upstreams.

### 🧰 P19 — Setup and authentication MCP tools — `████████░░ 83.3%`

**Feature:** Three auth/setup tools.

- 🧭 **T151 — Implement ga4_get_started** — `██████████ 100.0%`
  - ✅ **Gate:** Native CLI guidance and MCP schema parity for ga4_get_started · **Evidence:** src/main.rs implements ga4_get_started with native synmetra auth login --client-id-file guidance, explicit read-only analytics scope, API prerequisites and no Python/npm runtime requirements; test get_started_matches_pinned_empty_input_schema_and_native_login_guidance passed GitHub Actions 40225106072 core job.; src/main.rs native getting-started test verifies registered MCP input properties match pinned SednaLabs ga4_get_started no-argument schema and response supplies copyable native read-only OAuth guidance without secrets. src/auth.rs implements the referenced native login command. GitHub Actions 40225106072 passed fmt, Rust tests, strict Clippy and progress validation. Other upstream feature/response acceptance remains open.
- 🩺 **T152 — Implement ga4_auth_status** — `██████████ 100.0%`
  - ✅ **Gate:** Native CLI guidance and MCP schema parity for ga4_auth_status · **Evidence:** src/main.rs AuthStatusArgs and opt-in ga4_auth_status verify_token request with bounded Google account lookup and credential-free status; native schema and explicit-opt-in tests passed GitHub Actions 40223667547 core job (fmt, tests, strict Clippy). Live Google verification and exact upstream response parity remain open.; src/main.rs enforces ga4_auth_status verify_token type/default/description/optional required status against pinned SednaLabs inputSchema and tests opt-out plus explicitly authorized mock Google account-summary verification without exposing credential/account details; src/auth.rs native auth status guidance. GitHub Actions 40224853636 core job passed Rust tests, formatting, Clippy and ledger. This task does not certify entire upstream response parity or live Google acceptance.
- 🗝️ **T153 — Implement ga4_auth_login_command** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Native CLI guidance and MCP schema parity for ga4_auth_login_command · **Evidence (implementation only):** src/main.rs AuthLoginCommandArgs advertises all six pinned optional inputs, compares input schema types/defaults/descriptions/formats to pinned SednaLabs, POSIX-quotes generated commands, and rejects unsupported shared ADC/quota-project options explicitly; src/auth.rs implements CLI --account OAuth login hint, --callback-port, --headless with bounds/duplication tests. GitHub Actions 40225871288 core job passed fmt, tests, strict Clippy, ledger. Runtime parity for shared ADC/quota-project and live auth still pending.

### 📜 P20 — Schema and error contract — `████░░░░░░ 44.4%`

**Feature:** Contracts and schemas.

- 🪪 **T154 — Generate exact 36-name tool inventory from server** — `██████████ 100.0%`
  - ✅ **Gate:** No missing or unexpected tool names in union gate · **Evidence:** src/main.rs full_capability_profile_matches_exact_pinned_36_tool_union compares Rust MCP router registry to docs/tool-inventory.json, both additive write opt-in and scratchpad enabled; feature-enabled cargo test passed GitHub Actions 40224261534 duckdb job. Tool existence is NOT functional parity.
- 📑 **T155 — Compare required fields defaults descriptions and schemas** — `██████████ 100.0%`
  - ✅ **Gate:** Diff against both upstream pinned snapshots reviewed · **Evidence:** spec/audits/native-tool-input-schemas-40224261534.json captures 36 native router schemas from actual feature-enabled passing GitHub Actions 40224261534 (source commit 43a69ecc); spec/audits/input-schema-differences-40224261534.json records machine-readable property names, required sets, and selected raw schema attributes. docs/SCHEMA_PARITY_AUDIT.md explains scope, provenance and remediation categories.; docs/SCHEMA_PARITY_AUDIT.md and spec/audits/input-schema-differences-40224261534.json compare native schema baseline with all 35 pinned SednaLabs JSON schemas and 10 official Google extracted signatures; report preserves all detected differences and explicitly identifies conflicting defaults/feature union and snapshot age. Comparison acceptance does not mean gaps resolved or 36-tool parity complete.
- 🧳 **T156 — Implement Contract V1 ok/data/meta responses** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** All Rust parity tools produce validated success envelope
- ⚠️ **T157 — Implement Contract V1 ok/error/meta responses** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Errors have stable status code reason and context fields
- ⚖️ **T158 — Preserve official response semantics where conflicting** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Document compatibility choice and adapters
- 📏 **T159 — Enforce stable cursor and truncation metadata** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All tabular tools report known/unknown totals honestly
- 📂 **T160 — Publish schema snapshot with generated reference SHA** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Tool schema diff is reviewed during every upstream update · **Evidence:** spec/upstream/schema-lock.tsv published with Git object SHA-1 digests and upstream source commit identifiers, verified on each CI run using the native git hash-object command in .github/workflows/ci.yml. CI 40247973673 passed lock integrity without Python/npm. Full semantic schema diff/review on upstream version bumps remains open.
- 🤝 **T161 — Test alternative MCP clients and structured output** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Codex and inspector consume every major response type
- 🧯 **T162 — Avoid false confidence from mocked-only parity** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Document live coverage by tool and account eligibility

### 🔀 P21 — Feature-union conflict resolution — `░░░░░░░░░░ 0.0%`

**Feature:** Compatibility governance.

- 🔍 **T163 — Audit every parameter-level difference between sources** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Diff includes nullable fields limits aliases and errors
- 🧱 **T164 — Choose a stable canonical contract without dropping capabilities** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All documented upstream inputs remain expressible
- ⚙️ **T165 — Version compatibility adapters and response projections** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Legacy tool expectations and version policy documented
- 📚 **T166 — Keep per-tool source-to-Rust test reference links** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Each one of 36 tools has provenance and acceptance evidence
- 🕵️ **T167 — Detect new upstream tools and changes in CI** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Changes open maintenance work rather than silently inflate parity


## 🧪 M8 — Verification, resilience and security acceptance — `██░░░░░░░░ 17.9%`

**Scope:** MVP. **Acceptance gate:** Full offline and authorized live verification including malformed inputs, MCP interoperability and threat model.

### 🧫 P22 — Deterministic tests — `█████░░░░░ 50.0%`

**Feature:** Offline QA.

- 🧪 **T168 — Create offline GA Admin and Data endpoint mocks** — `██████████ 100.0%`
  - ✅ **Gate:** Representative success empty error and pagination fixtures · **Evidence:** src/google.rs deterministic HTTP mock harness and numerous no-secrets synthetic route/auth tests; Actions 40202015488 passed; src/google.rs in-memory Admin/Data server tests cover successful, empty, invalid response/replayed token and oversized collection cases; GitHub Actions core 40218035242 passed the mock suite.
- 📦 **T169 — Exercise all 36 tool names through MCP dispatch** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Tool-by-tool tests verify tool catalog and call behavior
- 🧿 **T170 — Test JSON schema rejection and nested depth caps** — `██████████ 100.0%`
  - ✅ **Gate:** Adversarial payload corpus is bounded and rejected · **Evidence:** src/google.rs: exhaustive bounded filter tests reject excess nesting, too many nodes, long strings, oversize JSON, and invalid filter shapes; GitHub Actions core 40218035242. Full adversarial fuzz corpus remains unaccepted.; src/google.rs adversarial_report_filter_corpus_is_bounded_before_google_network_access tests deterministic invalid JSON and filter corpus: empty/primitive roots, empty nested filters, breadth 65, 1025-byte strings, 17-kibibyte serialized inputs, 129-byte keys, over 256 nested nodes, and depth over 12. Both bounded filter validation and report_request reject the corpus with INVALID_REQUEST before Google credentials/network access. Supplements existing historic/complex filter tests; GitHub Actions 40240559437 core fmt, Rust tests, Clippy succeeded. This is deterministic corpus coverage, not a stochastic fuzzing or live Google claim.
- 📚 **T171 — Differentially compare upstream response projections** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Normalized golden fixtures generated from pinned baseline
- 🪟 **T172 — Exercise stdio tools under an MCP inspector** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Every advertised tool can be called with synthetic fixtures
- 🛰️ **T173 — Exercise protected Streamable HTTP end-to-end** — `██████████ 100.0%`
  - ✅ **Gate:** Valid and invalid sessions checked over loopback · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; src/http.rs authenticated_http_session_lists_and_calls_tools_but_rejects_invalid_sessions verifies a real Axum loopback HTTP server with rmcp legacy sessions, returned Mcp-Session-Id, initialized notification, authenticated tools/list (20 default read-only tools), successful ga4_get_started tools/call and structured payload, and rejection of unknown session, wrong bearer and wrong origin. mcp_json_response handles legacy SSE transport safely. GitHub Actions 40245367526 core and DuckDB passed. No live Google property was accessed; session termination/lifetime and modern stateless performance are separate hardening gates.
- 🧹 **T174 — Enforce schema/roadmap ledger consistency in CI** — `██████████ 100.0%`
  - ✅ **Gate:** Progress bars and evidence checks regenerated reproducibly · **Evidence:** src/bin/synmetra-progress.rs and .github/workflows/ci.yml, GitHub Actions run 40199550979 (2026-10-25) passed Rust tests and verified 36-tool union, 227 task evidence states, and README/roadmap percentages; src/bin/synmetra-progress.rs regenerate_bars derives each task, phase, milestone and overall display bar directly from two evidence-backed acceptance checks, rejects inconsistent evidence, preserves editorial prose and is idempotent in bar_regeneration_is_idempotent_and_preserves_editorial_text. The opt-in --write-bars command updates README/ROADMAP; .github/workflows/ci.yml rebuilds the real 227-task bars and requires git diff --exit-code on both files, alongside normal inventory/evidence audits. GitHub Actions 40241555014 full core and DuckDB passed including actual no-diff regeneration.
- ⏱️ **T175 — Validate timeouts retries backoff and rate limits** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Offline deterministic clock and throttled 429 tests
- 🦀 **T176 — Pass rustfmt clippy and tests on clean environment** — `██████████ 100.0%`
  - ✅ **Gate:** All-target strict checks green in CI · **Evidence:** Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.; Source: src/main.rs, src/google.rs, src/http.rs, src/auth.rs, docs/ANNOTATIONS.md, Cargo.lock and .github/workflows/ci.yml. GitHub Actions run 40204312090 passed 21-route gated annotation build, mock tests, locked fmt/Clippy; run 40204566566 passed authenticated Streamable HTTP initialization and read-only discovery tests.
- 🔐 **T177 — Run dependency secret and license scanners** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No accidental credentials or prohibited licenses in repository

### 🔬 P23 — Authorized live Google acceptance — `░░░░░░░░░░ 0.0%`

**Feature:** Live GA gates.

- 🧑 **T178 — Verify native browser user OAuth on Linux** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Authenticated operator with a safe Google test identity
- 🏢 **T179 — Confirm account and property discovery against real GA** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** At least one explicit test property readable
- 📊 **T180 — Confirm standard historical and realtime reports** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Compare key totals to GA console with date/timezone recorded
- 🧪 **T181 — Confirm funnel and conversion API eligibility behavior** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Expected success or documented Google restriction captured
- 📈 **T182 — Confirm pivot batch and property access reports** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Authorized scoped live evidence and no sensitive data committed
- 🗃️ **T183 — Confirm scratchpad ingest query and export lifecycle** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Synthetic or sanitized GA evidence kept private
- ✍️ **T184 — Opt-in annotation write and disable workflow** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Test property only after explicit operator authorization
- 🧬 **T185 — Smoke test Codex multi-tool use with local MCP** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Successful live analysis query with no manual QA burden
- 🗄️ **T186 — Record redacted reproducible live test ledger** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All outcomes scoped with date and source SHA

### 🛡️ P24 — Robustness and operational performance — `░░░░░░░░░░ 0.0%`

**Feature:** Reliability and security.

- 🔒 **T187 — Threat-model inbound Google and DuckDB trust boundaries** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Documented review of MCP prompt injection and auth vectors
- 🧪 **T188 — Fuzz resource names filters cursors and JSON parsing** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Fuzz corpus with reproducible seeds and crash guard
- ⏱️ **T189 — Benchmark startup latency peak RSS and tool calls** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Repeatable Linux measurements under no-cache and warm-cache
- 🧰 **T190 — Handle network outage and Google API failure modes** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Actionable status without unsafe retries
- ♻️ **T191 — Test OAuth refresh and cancellation under concurrency** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No token duplication or orphan request leaks
- 🚧 **T192 — Test untrusted MCP arguments for injection and SSRF** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No arbitrary networking or destructive file access
- 📃 **T193 — Redact logs telemetry and error reports by default** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No account data or tokens in CI artifacts
- 🛑 **T194 — Document safe remote deployment rejection gates** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No unauthenticated public shared-credential service
- 🦺 **T195 — Exercise build reproducibility and supply-chain pinning** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Signed provenance and dependency audit evidence


## 🚀 M9 — Linux release and Codex-ready inauguration — `░░░░░░░░░░ 0.0%`

**Scope:** MVP. **Acceptance gate:** A documented stable native binary with 36-tool verified parity and release artifacts; no Python/npm runtime.

### 📦 P25 — Packaging and installation — `░░░░░░░░░░ 0.0%`

**Feature:** Native packaging.

- 🏗️ **T196 — Build reproducible x86-64 Linux binary** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Artifact installs without Python npm or node
- 🦀 **T197 — Provide rustup and cargo-based source installation** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Native build documented for Arch and other Linux
- 📦 **T198 — Provide Arch PKGBUILD or local packaging recipe** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Pacman-managed or local user installation path tested
- 🧷 **T199 — Offer optional ARM64 Linux release binary** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Aarch64 CI artifact smoke test and linked library audit
- 📏 **T200 — Document DuckDB native shared/static requirements** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Fresh-system runtime prerequisites verified
- 🖋️ **T201 — Publish install/uninstall and upgrades guide** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No writes outside approved local paths
- 🐚 **T202 — Provide Fish-friendly CLI configuration examples** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No npm Python or bash-only assumptions

### 📖 P26 — Documentation and agent onboarding — `░░░░░░░░░░ 0.0%`

**Feature:** Operator docs.

- 📋 **T203 — Write complete CLI help config and environment reference** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All flags options and permissions documented
- 🔌 **T204 — Publish tested Codex global MCP registration** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** One-line setup verified with official Codex
- 🧠 **T205 — Document other MCP agents using stdio and HTTP** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Secure example configurations and token separation
- 🔑 **T206 — Write Google Cloud setup without mandatory gcloud** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Project API activation and scopes clearly explained
- 🔍 **T207 — Document per-tool parameter guides for all 36 tools** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Each tool includes input output limits and examples
- 🧪 **T208 — Publish anonymized smoke scenarios and QA limits** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Operator can diagnose errors without pasting secrets
- 📗 **T209 — Publish migrations from official and SednaLabs servers** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Tool schema and behavior differences listed
- 🛡️ **T210 — Publish responsible security disclosure policy** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Safe vulnerability contact with no leaked reports

### 🏁 P27 — Release acceptance and semantic versions — `░░░░░░░░░░ 0.0%`

**Feature:** v1.0 GA4 parity.

- ✅ **T211 — Complete all MVP atomic acceptance gates** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No tool parity acceptance based on mocks alone
- 📊 **T212 — Regenerate all progress percentages from evidence** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Zero uncited 100 percent tasks
- 🏷️ **T213 — Tag v1.0.0 only after parity and live QA** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Semver milestone traceable to test artifacts
- 📦 **T214 — Publish checksummed Linux release assets** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Checksums and source commit visible
- 🧾 **T215 — Publish provenance and release notes** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Attributed dependencies and known GA alpha limitations
- 🔏 **T216 — Publish SBOM and binary dependency inventory** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Explicit native runtime dependencies and license compliance
- 🩺 **T217 — Run clean-install and uninstall QA on EndeavourOS** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Does not disturb other installed tools


## 🔭 M10 — Post-MVP extensions and sustained stewardship — `░░░░░░░░░░ 0.0%`

**Scope:** POST_MVP. **Acceptance gate:** Changes beyond upstream feature union remain separate and do not delay v1.0 parity.

### 🌱 P28 — Lifecycle maintenance — `░░░░░░░░░░ 0.0%`

**Feature:** Upstream stewardship.

- 🔄 **T218 — Track upstream Google and Rust tool drift monthly** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Pinned bump proposes documented schema differences
- 🧬 **T219 — Add new Google Admin and Data API endpoints deliberately** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No accidental broad write surface
- 🗞️ **T220 — Maintain alpha API eligibility and deprecation matrix** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Conversion and funnel contract changes communicated
- 🏷️ **T221 — Publish clear deprecation and semver compatibility rules** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Existing agent configs survive non-major updates
- 🔬 **T222 — Maintain long-term performance and privacy regression suite** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Releases reject regressions against previous stable versions

### 📡 P29 — Optional analytics enhancements — `░░░░░░░░░░ 0.0%`

**Feature:** Optional extensions.

- 📊 **T223 — Add optional cross-property aggregate summary tool** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No unconsented aggregation across Google identities
- 💡 **T224 — Add privacy-preserving analysis templates for small sites** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No exported visitor-level personal information
- 🧠 **T225 — Add semantic agent analysis prompts as optional presets** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Prompts contain no private site or visitor details
- 🔗 **T226 — Add optional secure hosted self-service endpoint** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Threat model authorization TLS and rate limiting required
- 🌉 **T227 — Evaluate Chatarium agent harness/A2A interoperability** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Do not add protocol dependencies before core parity


## 🧰 Complete MCP tool acceptance matrix

See [machine-readable 36-tool inventory](tool-inventory.json), [upstream reference audit](UPSTREAM.md) and [architecture](ARCHITECTURE.md). Every one of the 36 named tools has a dedicated task in milestones M3–M7; shared tools must satisfy both upstream contracts. No runtime parity is claimed until every tool is integration-tested.
