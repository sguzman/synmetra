# 🤖 Codex CLI: local Synmetra registration

**Status:** based on [Codex CLI's pinned native MCP command implementation](https://github.com/openai/codex/blob/rust-v0.164.0/codex-rs/cli/src/mcp_cmd.rs). Core CI now includes an isolated registration/get/list/remove smoke using official Codex `rust-v0.164.0` Linux musl release archive verified against GitHub's published SHA-256; this new CI step must pass before its registration acceptance is credited. **A full live Codex multi-tool session is not yet accepted** (T185). This is a user-level setup recipe, not proof of real GA authorization or a requirement to use npm, Python, Node, or gcloud.

## Register one native Rust stdio executable

After installing the read-only binary under `~/.local/bin` as in [INSTALL.md](INSTALL.md), use this single command in Fish:

```fish
codex mcp add synmetra -- ~/.local/bin/synmetra serve
```

Codex stores the named MCP entry in its user-level `~/.codex/config.toml`. This registers a local stdio command, **not** a hosted server or HTTP listener. Google credentials are deliberately not passed as command-line arguments or stored as `codex mcp add --env KEY=SECRET`; the server reads its own private XDG OAuth file after an explicitly authorized login. The default Synmetra server remains read-only with no mutation or scratchpad enabled.

To inspect the resulting entry and remove it later:

```fish
codex mcp get synmetra
codex mcp list
codex mcp remove synmetra
```

The installation path must exist and be executable on the machine that runs Codex. If the binary is built but not installed, choose the correct **absolute executable path** rather than relying on the current working directory. Do not create duplicate entries if `codex mcp get synmetra` already finds one.

## Before attempting a real Google query

`synmetra auth doctor` runs offline. A future consenting operator can authorize `synmetra auth login --client-id-file PRIVATE_PATH` through a Desktop browser OAuth flow; see [GOOGLE_CLOUD.md](GOOGLE_CLOUD.md). Neither registering Codex nor listing tools proves Google property access. The read-only discovery names should be **20** by default; optional 15 DuckDB tools require building with `--features scratchpad` and explicit `serve --scratchpad`. The single annotation mutation requires `--allow-write` and separate edit-scoped Google authorization.

If Codex reports a missing executable, verify Synmetra's location. If it reports Google authentication errors, inspect local `auth doctor/status` rather than forwarding tokens into Codex prompts. If an MCP client produces a transport/schema error, use synthetic CI fixtures and record only redacted source SHA and error categories. The full Codex acceptance gate requires independent automated client execution and a separately authorized GA test identity/property; no manual user QA is substituted.

## Offline registration acceptance

Synmetra's GitHub Actions core job installs a reviewed and **SHA-256-checked official native Codex Linux binary** from the `rust-v0.164.0` release into a disposable CI directory. It uses an isolated `CODEX_HOME` (not the runner user's real configuration), performs `codex mcp add synmetra -- ABSOLUTE_BIN serve`, validates the JSON values returned by `codex mcp get synmetra --json`, confirms `mcp list --json`, removes the entry, and verifies the list is empty. Neither Google OAuth nor a Codex account login is required for this configuration-only smoke. Archive hash: `90dd0b03b28afcb7fa733a4f73b413db2916a47347efd43fb669cc1c774ee8e4`. This test does **not** establish T185's authenticated end-to-end analysis or third-party MCP Inspector compatibility.

## Configuration and security limits

The official CLI command is `codex mcp add NAME -- EXECUTABLE ARGS` for stdio. Optional direct TOML entries use `[mcp_servers.synmetra]` with a `command` and `args` list, but the managed `codex mcp add` command is preferred to avoid hand editing. User-level config is not a portable per-project repository file, and it should never be committed.

**Do not** register Synmetra's experimental loopback HTTP transport as an internet-accessible service or paste its inbound bearer token into a public config. Hosted, multi-identity and shared scratchpad use is outside the accepted boundary. More MCP client profiles are in [MCP_CLIENTS.md](MCP_CLIENTS.md).
