# ⌨️ Synmetra CLI, environment and Fish configuration

**Status:** experimental Linux/Wayland-first CLI. The default binary runs a **read-only MCP stdio server** and requires no Google credentials for `--help`, `--version`, `auth doctor` or source installation. No Python, npm, Node, gcloud or background daemon is required. Refer to [INSTALL.md](INSTALL.md) for the pinned Rust toolchain and Arch/EndeavourOS installation, and [SECURITY.md](SECURITY.md) for trust limits.

## Native commands and server flags

| Command/option | Behavior | Boundary |
| --- | --- | --- |
| `synmetra` or `synmetra serve` | Runs the default read-only MCP server on stdio | One trusted parent process; stdout reserved for MCP |
| `synmetra --help`, `synmetra -h`, `synmetra serve --help` | Prints built-in usage, flags and environment options | Offline, no Google API call |
| `synmetra --version`, `synmetra -V` | Prints the experimental CLI version | Offline |
| `synmetra serve --http-loopback` | Authenticated MCP Streamable HTTP at `127.0.0.1` | Requires an independent local bearer; never expose through a public proxy |
| `synmetra serve --allow-write` | Makes the annotation creation tool visible | **Not permission alone**: Google edit-scoped OAuth and property role must also be granted |
| `synmetra serve --scratchpad` | Enables local in-memory DuckDB tools | Binary must be compiled with `--features scratchpad`; single trusted stdio client only |
| `synmetra serve --scratchpad --http-loopback` | Rejected before server startup | Per-client DuckDB isolation not implemented |

Flags are not implicitly persistent. Omitting `--allow-write` always restores the default read-only server profile at the next launch. Omitting `--scratchpad` prevents exposing those local tools even in a feature-enabled binary.

## Native OAuth subcommands

| Command/option | Behavior and limits |
| --- | --- |
| `synmetra auth --help` or `synmetra auth login --help` | Show offline OAuth usage and permission notes |
| `synmetra auth login --client-id-file PATH` | Start a one-time Desktop OAuth consent flow with PKCE and randomized localhost callback; requests `analytics.readonly` |
| `--account EMAIL` | Optional Google login hint; **not** identity verification or authorization |
| `--callback-port PORT` | Use a selected localhost callback port instead of OS-chosen ephemeral port; failure to bind is an error |
| `--headless` | Print the consent URL; after authorized browser consent, paste the **complete** `http://127.0.0.1:PORT/callback?code=...&state=...` redirect URL back into Synmetra's private terminal. The code/state/redirect are validated; nothing is written to public logs |
| `--allow-edit` | Explicitly request additional `analytics.edit` permission; still requires `serve --allow-write` and an edit-capable property role |
| `synmetra auth status` | Report configured credential tier and freshness without revealing access or refresh tokens |
| `synmetra auth doctor` | Offline, redacted local diagnostics and setup guidance; no Google network request |
| `synmetra auth command` | Print a native login command template, without exporting secrets |
| `synmetra auth logout` | Delete only local Synmetra credentials; Google-side revocation requires a separate account action |

The Google Cloud Desktop OAuth client JSON is a private local input; it must never be checked into public Git. Google Analytics Admin and Data APIs must be enabled for the appropriate project. A valid scope does not guarantee a property role or v1alpha feature eligibility. Actual account, role, consent and live report acceptance are tracked under T178–T186, not claimed by these instructions.

## Headless browser copy-back (explicit, one-time)

When the Google account owner uses a browser that cannot connect to Synmetra's local callback port (for example, a separately operated machine), run:

```fish
synmetra auth login --client-id-file ~/private/google-desktop-oauth.json --headless
```

Open the emitted Google consent link privately, approve only the intended scopes, and copy the **entire loopback redirect URL** from the browser address bar. Paste that URL into the *same* Synmetra terminal and press Enter. The browser may report that the local address is unreachable; the URL can still be copied. Synmetra checks the exact expected callback origin, ephemeral port, path and per-login OAuth state before redeeming the code with its original PKCE verifier. A missing, duplicated, modified or prior-session state is rejected. The callback contains a one-time authorization secret: never paste it in chat, shell history, a public issue or a tracked config file. The prompt and credential exchange time out; restart the login if the callback expires.

Normal login **without** `--headless` continues to receive the loopback redirect automatically. Copy-back is a local login operation, not an MCP tool or a way to bypass Google's consent/role requirements.

## Environment variables

| Variable | Purpose | Security |
| --- | --- | --- |
| `SYNMETRA_GOOGLE_ACCESS_TOKEN` | Optional Google OAuth access-token override for a session | Overrides stored local credentials for **read calls only**; the annotation mutation rejects unverified external token scopes. Does **not** automatically refresh; never echo or put into command history |
| `SYNMETRA_GOOGLE_ADC_USER` | **Opt-in only**: set to `1` to permit native read-only `authorized_user` ADC refresh | Has no effect unless `GOOGLE_APPLICATION_CREDENTIALS` is also set; never grants annotation writes |
| `GOOGLE_APPLICATION_CREDENTIALS` | Absolute path to a private `authorized_user` ADC JSON supplied by the account owner | Not read unless `SYNMETRA_GOOGLE_ADC_USER=1`; mode 0600, regular non-symlinked file, max 32 KiB; not modified or committed |
| `SYNMETRA_MCP_BEARER_TOKEN` | Required for `--http-loopback`; independent local HTTP principal secret | Minimum 32 printable nonspace ASCII characters; **never** forward to Google |
| `SYNMETRA_GOOGLE_QUOTA_PROJECT` | Optional Google Cloud project ID/number for outbound `x-goog-user-project` quota attribution | Explicit operator opt-in. Requires `serviceusage.services.use` permission on that project; malformed values are rejected **before** any Google request |
| `SYNMETRA_MCP_PORT` | Optional positive u16 loopback HTTP port, default `44128` | Bound to `127.0.0.1` regardless of caller-specified address |
| `XDG_CONFIG_HOME` | Optional user configuration root | Private OAuth location is `$XDG_CONFIG_HOME/synmetra/credentials.json` |
| `HOME` | Fallback when XDG config root is unset | Fallback config is `~/.config/synmetra/credentials.json` |

Private Synmetra config is written with 0700 directory and 0600 file modes. These Unix permissions do not protect secrets against malware already running under the same UID. Internal test-only schema capture switches are not documented as supported operator configuration.

## Fish shell workflows (no npm)

Run offline setup diagnostics after native installation:

```fish
synmetra --help
synmetra auth doctor
synmetra auth status
```

For a separately authorized Google Desktop OAuth client JSON, read-only setup is:

```fish
synmetra auth login --client-id-file ~/private/google-desktop-oauth.json
synmetra serve
```

For an **already authorized** Google `authorized_user` ADC JSON, the optional native read-only mode requires both variables explicitly (and does not invoke `gcloud`):

```fish
set -gx SYNMETRA_GOOGLE_ADC_USER 1
set -gx GOOGLE_APPLICATION_CREDENTIALS ~/private/google-authorized-user.json
synmetra serve
```

Use an **absolute** file path and restrict it to mode 0600. The local Synmetra Desktop OAuth file, if present, takes priority; otherwise the ADC user refresh token exchanges for a short-lived token using native rustls, caches the result only in memory, and never rewrites the supplied JSON. Service accounts, impersonation, delegation, arbitrary credential JSON and annotation writes are **not** enabled by this mode. Removing the opt-in and restarting disables it. This is not a substitute for real OAuth scope/property-role acceptance.

For the optional DuckDB build, use the source directory and pinned Rust toolchain:

```fish
cargo +1.99.1 build --locked --features scratchpad --bin synmetra
./target/debug/synmetra serve --scratchpad
```

The following Fish example prompts for an already-generated local bearer **without putting the secret in command-line arguments or history**. Enter a private random token with at least 32 printable ASCII characters. Do not put it into a script tracked by Git:

```fish
read -s -P 'Local MCP bearer: ' SYNMETRA_MCP_BEARER_TOKEN
set -gx SYNMETRA_MCP_BEARER_TOKEN $SYNMETRA_MCP_BEARER_TOKEN
synmetra serve --http-loopback
```

For optional Google Cloud quota attribution, an authorized operator can configure a private shell environment variable such as `set -gx SYNMETRA_GOOGLE_QUOTA_PROJECT quota-project-123` before launching the server. The server sends that value as `x-goog-user-project` on both read and explicitly enabled write requests, never as authentication. It does not discover/alter gcloud credentials or grant Google project permissions. Google's quota project can require the caller's `serviceusage.services.use` role. This override must not be sourced from MCP tool arguments.

Do not substitute a Google OAuth access token as the local MCP bearer. The two authentication boundaries must stay separate, including when configuring an agent. `--http-loopback` does not provide TLS or any supported remote-hosting guarantee.

## Error and QA boundaries

- Missing native scratchpad feature: rebuild explicitly with `--features scratchpad`, or omit `--scratchpad`.
- Missing Google credentials: use `auth doctor` and optional local OAuth consent; do not paste secrets into logs, issues or chat.
- HTTP bearer missing, too short or non-ASCII: correct the private env configuration; never disable the check to make a remote listener work.
- Stale Google credentials may refresh on an authorized MCP call; `auth status` is a safe local check, not a Google property or role validation.
- `auth logout` removes only Synmetra credentials, not OAuth grants or external backups.
- Local authentication, parsing and mock Google tests run automatically in CI; authorized real-Google QA and clean EndeavourOS installation tests are still open. See [LIVE_ACCEPTANCE_MATRIX.md](LIVE_ACCEPTANCE_MATRIX.md) and the [acceptance ledger](progress.json).
