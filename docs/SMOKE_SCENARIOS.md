# 🧪 Synthetic smoke scenarios and QA limitations

**Purpose:** let a developer or agent diagnose Synmetra without requesting the operator's credentials, property IDs, screenshots of private analytics, terminal secrets, or any manual QA that can instead run in CI. These are diagnostic scenarios, **not** a full upstream contract or real Google acceptance certificate.

## Automated offline scenarios

| Scenario | Command/test or CI evidence | Expected safe result | What it does **not** prove |
| --- | --- | --- | --- |
| Read-only help | `synmetra --help`, `synmetra serve --help`, `synmetra auth --help` | Offline, no native external command dependency, no secrets | A real Google account can log in |
| Setup diagnosis | `synmetra auth doctor` and `auth status` with temporary `XDG_CONFIG_HOME` | No token values or OAuth client secrets on stdout | Google-side scopes/roles are valid |
| Rejected OAuth refresh | `rejected_and_malformed_oauth_refreshes_preserve_private_credentials`, `missing_or_corrupt_local_credentials_do_not_trigger_network_refresh` | Mock 400/401, malformed/empty tokens and absent/corrupt private credentials return redacted errors without token rotation or unintended network calls | Live Google revocation, quota-project and service-account identity modes |
| Native installation | CI `cargo install --path . --locked --debug --bin synmetra --root TMPDIR` then uninstall | Installed program works with `PATH=/nonexistent`; only designated root is removed | Full EndeavourOS/makepkg clean-machine installation |
| MCP stdio | CI initializes, lists tools, and calls `ga4_get_started` using synthetic canary token | Valid JSON-RPC stdout, 20 default read-only tools, no token reflection | Codex/Inspector live client interoperability |
| Local MCP HTTP | Tests `loopback_http_rejects_unauthenticated_wrong_host_and_wrong_origin` and `local_mcp_bearer_is_not_forwarded_to_google_api` | Local bearer and Google token remain independent, non-loopback access rejected | Multi-user service or public TLS |
| Google request safety | `readonly_429_exhausts_three_attempts_with_redacted_error` and `readonly_socket_drop_fails_without_unsafe_transport_retries` | No transport replay, safe 429 budget, upstream error body not reflected | All live network outages, global request cancellation or provider quota behavior |
| Resource/filter/cursor injection | `deterministic_resource_and_cursor_mutation_corpus_fails_closed`, `adversarial_report_filter_corpus_is_bounded_before_google_network_access` | Rejected before outbound Google request | Coverage-guided fuzz completeness |
| DuckDB sandbox | `cargo test --locked --features scratchpad` in native CI | Selected in-memory session/quotas, hostile SQL denials, evidence bundle caps | Native library CVE safety, temp spill cleanup or multi-user isolation |
| Contract/roadmap integrity | `cargo run --locked --bin synmetra-progress` and pinned upstream schema lock hash checks | 227 atomic tasks, source evidence, 36 names, README/roadmap bars agree | All 36 tools match upstream behavior |

## Disconnected/offline failures

- If Google is unavailable, Synmetra reports a redacted network/HTTP error and only retries idempotent reads for a short bounded interval. It does not silently fall back to another identity or send an annotation write again.
- If a field, offset or cursor is invalid, use the error category and supplied field names; do **not** paste a raw bearer, Google response body, personal landing URLs, account IDs or private property contents into GitHub issues.
- If the embedded DuckDB scratchpad feature is missing, rebuild with `--features scratchpad` or omit `--scratchpad`. Never expose its shared local session manager over HTTP to get around a limitation.
- If a test reports a formatting, Clippy, dependency or pinned-schema failure, inspect the exact failing **step** and source SHA. A green core job does not certify a feature-enabled DuckDB build if that job was canceled.

## Evidence criteria and sanitized incident template

When an automated test fails, record only: Synmetra source SHA; UTC timestamp; CI run URL and job/step; feature profile; synthetic test name; redacted error category and expected/observed outcome. Example:

`SHA=<full_sha>; profile=read_only_stdio; test=synthetic_mcp_initialize; expected=20 read-only tools; observed=validation_error; credentials=redacted; live_GA=false`

Never store OAuth client JSON, access/refresh tokens, personal query strings, user identifiers, private GA property IDs, full private API URLs, account email addresses, source website content or private discussion text in public files.

## Real Google acceptance requires separate permission

The live acceptance matrix [LIVE_ACCEPTANCE_MATRIX.md](LIVE_ACCEPTANCE_MATRIX.md) records which of the 36 tools require an actual GA4 account/role and which operate only locally. All live calls remain **unverified** until an authorized consenting operator supplies a safe test identity/property and an agent runs the test without committing sensitive output. Sandbox and offline smoke cannot be substituted for those gates. No manual user QA is needed for the synthetic CI scenarios above.
