# 🗺️ Synmetra exhaustive roadmap

**Status:** bootstrap planning, **0/36 runtime tools accepted**. This project is not yet an MCP server. **Source of truth:** [progress.json](progress.json). All percentages shown are task-check acceptance averages, not guesses of feature implementation.

**Overall:** `░░░░░░░░░░ 2.9%` · **+2.86 percentage points since initial empty repo (+0.22 points in this CI validation round)** · 6/227 accepted atomic tasks. **Production parity gate:** 36/36 documented tools, exact inputs, secure deployment, permitted live property tests and a native install.

### 📏 Progress conventions

- Every `Tnnn` is an atomic deliverable with two explicit acceptance checks in [progress.json](progress.json); both must be accepted with evidence for 100%. A source stub or draft is 0% unless a check has evidence.
- Phases and milestones average **atomic task** percentages, not pre-rounded visual bars. Overall averages all tasks including post-MVP work; v1 parity and post-MVP also tracked separately. The symbols `█` and `░` each represent 10 percentage points, rounded to the nearest block.
- On each stewardship round, report total and actively worked milestone/phase/task bars plus the **signed delta in percentage points** against the previous verified committed ledger; completed 100% tasks can be omitted in ongoing status updates.
- **No progress padding.** Upstream feature inventory, planning and brand assets count only for their specific documentation tasks; GA4 functionality starts at 0% until tests establish otherwise. Real Google access, credentials and live tests are separate explicit gates.

## 🧭 M0 — Reference baseline and project governance — `████░░░░░░ 35.3%`

**Scope:** MVP. **Acceptance gate:** Audited pinned feature-union inventory, color identity, public-safe policies and reproducible plan. Foundation is not runtime parity.

### 📚 P00 — Upstream contracts and provenance — `████████░░ 75.0%`

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
- 🧾 **T007 — Archive exact upstream tool schemas and signatures** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Verifiable pinned schema snapshots with tool argument parity tests
- 🔁 **T008 — Set up upstream drift tracking and version pin policy** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** CI alerts on name/signature deltas without silent breaking changes

### 🦀 P01 — Native scaffold and quality gates — `░░░░░░░░░░ 0.0%`

**Feature:** Rust foundation.

- 📦 **T009 — Initialize single-crate Rust executable synmetra** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Cargo metadata and CLI build on stable Rust x86-64 Linux
- 🔌 **T010 — Integrate rmcp server SDK foundation** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Protocol version pin plus compile-time registration tests
- 🌐 **T011 — Add reqwest rustls API client skeleton** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No OpenSSL or Python runtime dependency
- 🧪 **T012 — Add fmt clippy unit tests and CI jobs** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Green push checks with pinned toolchain and dependency cache
- 🧷 **T013 — Add lockfile dependency audits and license checks** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Dependencies auditable with deny or equivalent process
- 🧰 **T014 — Create deterministic offline fixtures and mock server** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No secrets or real analytics identifiers in fixtures
- 🧭 **T015 — Implement CLI config parsing and doctor subcommands** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Actionable diagnostics and typed failures without exposing secrets
- 🔏 **T016 — Threat-model local execution and its credential boundary** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Review accepted before live Google credentials
- 🗒️ **T017 — Create changelog and reproducible release policy** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Document semver and provenance gates


## 🔌 M1 — MCP protocol and transport fidelity — `░░░░░░░░░░ 0.0%`

**Scope:** MVP. **Acceptance gate:** Spec-conforming native MCP discovery and calls over stdio plus bounded Streamable HTTP; all transport safety tests pass.

### 📨 P02 — Stdio protocol — `░░░░░░░░░░ 0.0%`

**Feature:** MCP stdio.

- 📥 **T018 — Support MCP initialize and capability negotiation** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Inspector negotiates compatible protocol version and identity
- 📋 **T019 — Support tools/list with exact declared JSON schemas** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All available tools discoverable by MCP clients
- ⚙️ **T020 — Support tools/call dispatch and structured content** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Call requests resolve to tools with valid response envelopes
- 📣 **T021 — Implement JSON-RPC notifications and cancellation** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Cancelled requests do not spawn orphan Google calls
- 🚧 **T022 — Separate stdout protocol from stderr diagnostics** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No banners or logging contaminate JSON-RPC stdout
- 📏 **T023 — Bound request sizes concurrency and timeouts** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Malformed and oversize frames rejected deterministically
- ⌨️ **T024 — Integrate Codex stdio configuration smoke** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Codex recognizes locally built synmetra binary

### 🌐 P03 — Streamable HTTP protocol — `░░░░░░░░░░ 0.0%`

**Feature:** MCP Streamable HTTP.

- 🌍 **T025 — Expose Streamable HTTP MCP endpoint** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Official MCP inspector connects and initializes
- 🏠 **T026 — Default bind to loopback only** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Non-loopback starts are rejected unless policy configured
- 🧿 **T027 — Validate Host Origin and DNS rebinding defenses** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Host header and Origin tests block inappropriate requests
- 🔐 **T028 — Add inbound bearer authorization policy** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Unauthenticated requests cannot enumerate tools on protected endpoints
- 🧭 **T029 — Serve protected MCP metadata and health endpoints** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Health is not publicly exposed under inbound auth
- 📡 **T030 — Support authenticated sessions and request routing** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Sessions terminate and cannot cross principal boundaries
- 🧊 **T031 — Enforce payload and connection resource limits** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Slow and oversized requests are bounded under load
- 🪪 **T032 — Support TLS behind secure deployment configurations** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Certificate and proxy trust rules reviewed and exercised
- 🧫 **T033 — Test MCP HTTP inspector interoperability** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** End-to-end tool invocation under loopback and protected remote modes

### 🧾 P04 — MCP schema semantics — `░░░░░░░░░░ 0.0%`

**Feature:** MCP contract.

- 📐 **T034 — Validate structured input against exported schemas** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Tool arguments validated with stable user-facing errors
- 🧩 **T035 — Declare read-only versus mutating tool annotations** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Annotation write differs accurately from read tools
- 🎛️ **T036 — Expose capability-profile filtered tool lists** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Default read-only excludes writes and scratchpad effects
- ♻️ **T037 — Honor deterministic pagination and tool list consistency** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Discovery pagination and list change tests pass
- ⚠️ **T038 — Normalize MCP protocol errors separately from GA failures** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Invalid tools are distinct from Google API errors
- 🧵 **T039 — Propagate request cancellation across auth and HTTP** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Interrupted calls free resources
- 🧪 **T040 — Run cross-client MCP conformance suite** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Protocol wire fixtures pass multiple clients


## 🔑 M2 — Google authentication and credential isolation — `░░░░░░░░░░ 0.0%`

**Scope:** MVP. **Acceptance gate:** Secure, least-privilege, non-Python auth across interactive, local and service configurations; no secret leaks.

### 🪪 P05 — Native OAuth login and operators — `░░░░░░░░░░ 0.0%`

**Feature:** Native OAuth.

- 🧭 **T041 — Implement auth login and auth status CLI** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** One-time login and status work without invoking Python or npm
- 🧰 **T042 — Implement auth doctor and login-command helper** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Operator gets actionable non-secret diagnostics
- 🌐 **T043 — Implement browser OAuth PKCE loopback flow** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** CSRF state, PKCE, timeout and redirect checks pass
- 🖥️ **T044 — Support headless copy-back OAuth setup** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Headless flow completes and rejects replayed callbacks
- 🔐 **T045 — Store tokens in GA4-specific user config storage** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Private file permissions and no shared ADC clobbering
- ♻️ **T046 — Implement expiry-aware access-token refresh** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Refresh honors expiry and avoids parallel stampedes
- 📛 **T047 — Validate requested OAuth scopes per capability** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Read-only uses analytics.readonly; writes demand analytics.edit
- 🔎 **T048 — Validate live token and quota-project configuration** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** API-enabled project and quota errors are distinguishable

### 📂 P06 — ADC, service account and token policy — `░░░░░░░░░░ 0.0%`

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
- 🧮 **T055 — Support configured refresh-token credentials** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Explicit credentials refresh without CLI gcloud dependency
- 🏷️ **T056 — Support quota project and billing headers** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Expected Google quota routing preserved
- ⏳ **T057 — Guard clock skew and rotation races** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Expiring secrets renew safely during concurrent calls

### 🛡️ P07 — Inbound versus upstream trust — `░░░░░░░░░░ 0.0%`

**Feature:** Identity security.

- 🚪 **T058 — Separate inbound MCP identity from Google identity** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No identity-conflation in HTTP transport
- 🧾 **T059 — Define capability authorization matrices** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Google and local permissions checked per tool call
- 💼 **T060 — Avoid credential exposure in crash logs and traces** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Secret scanner fixtures remain clean
- 🔍 **T061 — Validate Google scope and role errors distinctly** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No broad-scope silent escalation on 403
- 🔐 **T062 — Protect local credential files from symlink attacks** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No world-readable files and atomic writes
- 🚫 **T063 — Block SSRF via URLs in config or request arguments** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Only pinned Google API hosts are contacted by default
- 🧪 **T064 — Exercise expired revoked and invalid credentials** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All modes fail safely with stable errors
- 📦 **T065 — Ensure no auth dependency on gcloud installation** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Native test environment lacks gcloud Python and npm
- 🧯 **T066 — Implement safe logout and credential revocation guidance** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** State cleared without touching unrelated Google apps


## 🏢 M3 — Admin API information parity — `░░░░░░░░░░ 0.0%`

**Scope:** MVP. **Acceptance gate:** All administrative read tools and metadata endpoints match pinned signatures and response behavior.

### 🗂️ P08 — Admin MCP tools — `░░░░░░░░░░ 0.0%`

**Feature:** Seven Admin read tools.

- 🧾 **T067 — Implement get_account_summaries** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for get_account_summaries
- 📃 **T068 — Implement get_account_data_sharing_settings** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for get_account_data_sharing_settings
- 🏠 **T069 — Implement get_property_details** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for get_property_details
- ⏳ **T070 — Implement get_property_data_retention_settings** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for get_property_data_retention_settings
- 🔗 **T071 — Implement list_google_ads_links** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for list_google_ads_links
- 📌 **T072 — Implement list_property_annotations** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for list_property_annotations
- 🧬 **T073 — Implement get_custom_dimensions_and_metrics** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Golden Admin API mock response and auth-denial case for get_custom_dimensions_and_metrics

### 🏷️ P09 — Administrative edge cases — `░░░░░░░░░░ 0.0%`

**Feature:** Admin fidelity.

- 📚 **T074 — Page through all account summaries and Google Ads links** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Multi-page provider fixtures return complete results
- 🗺️ **T075 — Normalize property and account resource names** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Numeric and canonical IDs round-trip
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


## 📊 M4 — Reporting, preflight and Data API parity — `░░░░░░░░░░ 0.0%`

**Scope:** MVP. **Acceptance gate:** All report tools return correct GA4 semantics, including documented alpha, pagination and response projections.

### 📈 P10 — Report MCP tools — `░░░░░░░░░░ 0.0%`

**Feature:** Eight report tools.

- 📊 **T083 — Implement run_report** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_report
- 🛒 **T084 — Implement run_conversions_report** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_conversions_report
- 🚦 **T085 — Implement run_funnel_report** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_funnel_report
- ⚡ **T086 — Implement run_realtime_report** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_realtime_report
- 🧩 **T087 — Implement run_pivot_report** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_pivot_report
- 📦 **T088 — Implement batch_run_reports** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for batch_run_reports
- 🔎 **T089 — Implement run_property_access_report** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_property_access_report
- 👥 **T090 — Implement run_account_access_report** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** MCP schema, Google request, golden output and error cases for run_account_access_report

### 🧭 P11 — Preflight MCP tools — `░░░░░░░░░░ 0.0%`

**Feature:** Preflight and metadata.

- 🧭 **T091 — Implement preview_report_request** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Validate its contract and unsupported metrics without wasteful report requests
- ✅ **T092 — Implement check_report_compatibility** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Validate its contract and unsupported metrics without wasteful report requests
- 🧬 **T093 — Cache and refresh Data API metadata safely** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Metadata cache keys include property and caller
- 🔤 **T094 — Support flexible dimension and metric filters** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** FilterExpression nested JSON and shorthand cases validated
- 📅 **T095 — Validate date ranges and reserved dates** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Multiple ranges and dates behave as Google expects
- 🧮 **T096 — Implement metric and dimension compatibility checks** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Invalid combinations handled before expensive calls
- 📝 **T097 — Normalize input values without silent coercion** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Lossless valid requests and typed invalid results

### 📐 P12 — Data request fidelity and limits — `░░░░░░░░░░ 0.0%`

**Feature:** Reporting correctness.

- 🧮 **T098 — Support metric aggregates and row counts** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Totals minimum maximum and aggregate fields preserved
- 📎 **T099 — Support pagination offsets and opaque cursors** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Continuation is stable with bounded row limits
- ⚡ **T100 — Honor realtime non-paginated semantics** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No fake continuation token when GA realtime lacks offset
- 🗂️ **T101 — Support pivot report ordering and groups** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Nested pivots and row groups validated by fixtures
- 📦 **T102 — Enforce batch report maximums and isolation** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Five-report constraint and distinct report results preserved
- 🧪 **T103 — Support funnel steps segments trended and breakdown** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Complex v1alpha funnel shapes round-trip
- 🛒 **T104 — Support conversion and attribution reporting v1alpha** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Eligible and ineligible properties/errors tested
- 💵 **T105 — Preserve currency timezone and numeric precision** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Typed metrics and currency metadata retained
- 📊 **T106 — Support output modes rows tuples scalar compact** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All compatible result projections are predictable
- ✂️ **T107 — Support row cell and request-size clipping** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Limits protect context without hiding truncation metadata
- 📋 **T108 — Support summary-only responses and total known flags** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Counts and partial results are never misrepresented
- 🌍 **T109 — Handle property-specific API permission and quota errors** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** 429 and 403 retry/backoff rules explicit
- 🧱 **T110 — Bound oversized filter recursion and JSON arguments** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Worst-case requests rejected before Google API
- ♻️ **T111 — Make repeated report retries safe and idempotent** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Retry-After respected with bounded jitter
- 🧾 **T112 — Preserve GA response quota metadata and warnings** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Quotas accessible without leaking tokens


## ✍️ M5 — Explicitly gated GA mutation parity — `░░░░░░░░░░ 0.0%`

**Scope:** MVP. **Acceptance gate:** One official write tool operates with analytics.edit and explicit opt-in; ordinary mode cannot mutate GA.

### 📝 P13 — Annotation creation parity — `░░░░░░░░░░ 0.0%`

**Feature:** One opt-in Google write tool.

- 🖊️ **T113 — Implement create_property_annotation** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Full official title description dates and color schema contract tested
- 📅 **T114 — Validate annotation date and range order** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Reject malformed dates and reversed ranges before Google call
- 🎨 **T115 — Reject reserved ORANGE and invalid enum colors** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Only supported annotation colors submitted to Google
- 📝 **T116 — Validate maximum title and description lengths** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** 60-character title and 150-character description bounds respected
- 🔐 **T117 — Require explicit allow-write startup capability** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Default tools/list and tools/call cannot write
- 🪪 **T118 — Demand analytics.edit scope and sufficient property role** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Clear denial without escalating OAuth permissions
- ✋ **T119 — Require user confirmation at client boundary when supported** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** High-impact writes annotated and host approval guidance explicit
- 🧪 **T120 — Test write denial and optional live opt-in acceptance** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No writes in automated public CI

### 🚦 P14 — Write safety policies — `░░░░░░░░░░ 0.0%`

**Feature:** Write gating.

- 🏷️ **T121 — Annotate mutating MCP tools correctly** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** ReadOnly and idempotence hints match effects
- 🛡️ **T122 — Keep write tool hidden under read-only profiles** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Bypass attempts fail regardless of tool visibility
- 🧾 **T123 — Produce redacted audit events for writes** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Timestamp and action identifiable but no secrets
- ♻️ **T124 — Handle Google transient failures without blind retries** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No automatic duplicate annotation creation
- 🧯 **T125 — Support reversible config opt-out without token replacement** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Disable write capability at runtime restart


## 🗃️ M6 — DuckDB scratchpad and advanced analytics parity — `░░░░░░░░░░ 0.0%`

**Scope:** MVP. **Acceptance gate:** All 15 local scratchpad tools and policy limits operate without unsafe SQL or credential disclosure.

### 🗄️ P15 — Scratchpad lifecycle MCP tools — `░░░░░░░░░░ 0.0%`

**Feature:** Seven session tools.

- 🧱 **T126 — Implement scratchpad_open_session** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_open_session
- 🔒 **T127 — Implement scratchpad_close_session** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_close_session
- 📋 **T128 — Implement scratchpad_list_sessions** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_list_sessions
- 📏 **T129 — Implement scratchpad_get_runtime_limits** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_get_runtime_limits
- 🎛️ **T130 — Implement scratchpad_set_runtime_limits** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_set_runtime_limits
- 🗂️ **T131 — Implement scratchpad_list_tables** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_list_tables
- 🧹 **T132 — Implement scratchpad_drop_table** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Isolated session lifecycle plus quota failure fixtures for scratchpad_drop_table

### 📥 P16 — Scratchpad ingest and query MCP tools — `░░░░░░░░░░ 0.0%`

**Feature:** Five ingest/query tools.

- 📊 **T133 — Implement scratchpad_ingest_report** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Synthetic GA payload and DuckDB query conformance for scratchpad_ingest_report
- ⚡ **T134 — Implement scratchpad_ingest_realtime_report** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Synthetic GA payload and DuckDB query conformance for scratchpad_ingest_realtime_report
- 🔎 **T135 — Implement scratchpad_query** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Synthetic GA payload and DuckDB query conformance for scratchpad_query
- 🧬 **T136 — Implement scratchpad_describe_table** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Synthetic GA payload and DuckDB query conformance for scratchpad_describe_table
- 🧮 **T137 — Implement scratchpad_summarize_table** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Synthetic GA payload and DuckDB query conformance for scratchpad_summarize_table

### 📉 P17 — Scratchpad analysis MCP tools — `░░░░░░░░░░ 0.0%`

**Feature:** Three analytic tools.

- 📉 **T138 — Implement scratchpad_release_regression_report** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Evidence fixture and schema/output equivalence for scratchpad_release_regression_report
- 🔀 **T139 — Implement scratchpad_landing_param_shift_report** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Evidence fixture and schema/output equivalence for scratchpad_landing_param_shift_report
- 📤 **T140 — Implement scratchpad_export_evidence_bundle** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Evidence fixture and schema/output equivalence for scratchpad_export_evidence_bundle

### 🔐 P18 — DuckDB security and resource controls — `░░░░░░░░░░ 0.0%`

**Feature:** Scratchpad sandbox.

- 🚪 **T141 — Gate scratchpad behind explicit capability profile** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Default read-only process has no scratchpad tools
- 🧩 **T142 — Enforce per-session quotas and retention** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Memory disk time rows and session count bounded
- 🗃️ **T143 — Isolate scratchpad database files per identity** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Cross-user table read attempts fail
- 🪬 **T144 — Implement SQL SELECT-only parser policy** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** DML DDL COPY INSTALL LOAD ATTACH and external scans rejected
- 🌍 **T145 — Disable remote extensions and filesystem reads** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No network or arbitrary path escape through DuckDB
- 🧯 **T146 — Handle unexpected termination and session cleanup** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Leftover temp files sanitized on restart
- 📏 **T147 — Provide deterministic truncation and evidence limits** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Bundle size and context limits enforced
- 🔏 **T148 — Guard exported evidence bundle permissions** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Exports do not overwrite arbitrary paths or expose secrets
- 🧪 **T149 — Run embedded DuckDB fuzz and hostile SQL fixtures** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Policy bypass corpus fails safely
- 🦀 **T150 — Document native DuckDB library boundary honestly** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Dependency license architecture and portability reviewed


## 🧬 M7 — Tool contract interoperability and parity audits — `░░░░░░░░░░ 0.0%`

**Scope:** MVP. **Acceptance gate:** All 36 tool schemas and response mappings are independently snapshot-tested against both pinned upstreams.

### 🧰 P19 — Setup and authentication MCP tools — `░░░░░░░░░░ 0.0%`

**Feature:** Three auth/setup tools.

- 🧭 **T151 — Implement ga4_get_started** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Native CLI guidance and MCP schema parity for ga4_get_started
- 🩺 **T152 — Implement ga4_auth_status** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Native CLI guidance and MCP schema parity for ga4_auth_status
- 🗝️ **T153 — Implement ga4_auth_login_command** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Native CLI guidance and MCP schema parity for ga4_auth_login_command

### 📜 P20 — Schema and error contract — `░░░░░░░░░░ 0.0%`

**Feature:** Contracts and schemas.

- 🪪 **T154 — Generate exact 36-name tool inventory from server** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** No missing or unexpected tool names in union gate
- 📑 **T155 — Compare required fields defaults descriptions and schemas** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Diff against both upstream pinned snapshots reviewed
- 🧳 **T156 — Implement Contract V1 ok/data/meta responses** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All Rust parity tools produce validated success envelope
- ⚠️ **T157 — Implement Contract V1 ok/error/meta responses** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Errors have stable status code reason and context fields
- ⚖️ **T158 — Preserve official response semantics where conflicting** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Document compatibility choice and adapters
- 📏 **T159 — Enforce stable cursor and truncation metadata** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All tabular tools report known/unknown totals honestly
- 📂 **T160 — Publish schema snapshot with generated reference SHA** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Tool schema diff is reviewed during every upstream update
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


## 🧪 M8 — Verification, resilience and security acceptance — `░░░░░░░░░░ 1.8%`

**Scope:** MVP. **Acceptance gate:** Full offline and authorized live verification including malformed inputs, MCP interoperability and threat model.

### 🧫 P22 — Deterministic tests — `█░░░░░░░░░ 5.0%`

**Feature:** Offline QA.

- 🧪 **T168 — Create offline GA Admin and Data endpoint mocks** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Representative success empty error and pagination fixtures
- 📦 **T169 — Exercise all 36 tool names through MCP dispatch** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Tool-by-tool tests verify tool catalog and call behavior
- 🧿 **T170 — Test JSON schema rejection and nested depth caps** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Adversarial payload corpus is bounded and rejected
- 📚 **T171 — Differentially compare upstream response projections** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Normalized golden fixtures generated from pinned baseline
- 🪟 **T172 — Exercise stdio tools under an MCP inspector** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Every advertised tool can be called with synthetic fixtures
- 🛰️ **T173 — Exercise protected Streamable HTTP end-to-end** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Valid and invalid sessions checked over loopback
- 🧹 **T174 — Enforce schema/roadmap ledger consistency in CI** — `█████░░░░░ 50.0%`
  - ✅ **Gate:** Progress bars and evidence checks regenerated reproducibly
- ⏱️ **T175 — Validate timeouts retries backoff and rate limits** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** Offline deterministic clock and throttled 429 tests
- 🦀 **T176 — Pass rustfmt clippy and tests on clean environment** — `░░░░░░░░░░ 0.0%`
  - ✅ **Gate:** All-target strict checks green in CI
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
