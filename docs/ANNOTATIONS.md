# ✍️ Opt-in Google Analytics annotations

The official Google Analytics MCP server includes `create_property_annotation`, which is absent from SednaLabs' read-only Rust reference. Synmetra includes the tool but keeps it **disabled and hidden by default**.

To expose the write tool locally, explicitly start `synmetra serve --allow-write`. The selected Google OAuth identity must independently have the `https://www.googleapis.com/auth/analytics.edit` scope and appropriate property role; the built-in native `synmetra auth login` currently requests **read-only** access and will not silently expand scopes. An explicitly supplied authorized short-lived token through `SYNMETRA_GOOGLE_ACCESS_TOKEN` can be used for write testing. The server will otherwise return a Google 403 without mutating anything.

Annotation inputs: `property_id`, `title` (1–60 characters), `start_date` (YYYY-MM-DD), optional `end_date` (not earlier than start), optional `description` (up to 150 characters), `color` (PURPLE, BROWN, BLUE, GREEN, RED or CYAN; default BLUE). ORANGE is Google-reserved and rejected. Google calls use `v1alpha` Admin API over rustls, and writes never auto-retry.

**Verification:** synthetic tests validate color/date/length restrictions, API path, flag-dependent tool visibility, and rejection without credentials. No live annotation has been created; live write acceptance is a separate user-authorized test-property gate. MCP client host confirmation is recommended; Synmetra's explicit `--allow-write` is an operator capability gate, not a per-call interactive confirmation.

Do not expose `--allow-write` over a publicly reachable HTTP transport.
