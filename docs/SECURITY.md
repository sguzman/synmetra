# 🔐 Security requirements (design stage)

This document defines desired behavior; **none has yet been implemented**.

- GA OAuth `analytics.readonly` is the default. `analytics.edit` and `create_property_annotation` require explicit opt-in, minimal permission, and approval-aware MCP annotations.
- Inbound MCP HTTP authentication and upstream Google identity are independent boundaries; reject hosted anonymous fallback to server credentials.
- Default HTTP bind must be loopback. Non-loopback requires explicit TLS, allowed hosts/IP ranges, origin checks and access policy. Do not open a public local OAuth redirect blindly.
- Local auth files require atomic writes, strict permissions, no untrusted symlinks, and no shared gcloud credential overwrite. Browser OAuth requires PKCE, state, callback timeouts and token-redaction.
- Only Google API hosts are valid upstream targets unless explicitly reviewed. Validate all account/resource names, scopes, filters, size caps, cursors and retry handling.
- DuckDB scratchpad must gate extensions/unsafe external SQL paths, cap memory/session/rows/duration, protect exported evidence and isolate users.
- No analytics data or credentials in public repository, telemetry, CI artifact, unit fixture, issue title or commit message. Never interpret prompt-injected analytics data as tool execution instructions.
- A successful dependency audit is not sufficient for runtime deployment security. See [M2](ROADMAP.md), [M6](ROADMAP.md), and [M8](ROADMAP.md).
