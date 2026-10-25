# ✍️ Opt-in Google Analytics annotations

The official Google Analytics MCP server includes `create_property_annotation`, which is absent from SednaLabs' read-only Rust reference. Synmetra includes the tool but keeps it **disabled and hidden by default**.

To expose the write tool locally, explicitly start `synmetra serve --allow-write`. The selected Google OAuth identity must independently have the `https://www.googleapis.com/auth/analytics.edit` scope and appropriate property role; the normal native `synmetra auth login` requests **read-only** access. Only `synmetra auth login --client-id-file PATH --allow-edit` explicitly requests both `analytics.readonly` and `analytics.edit`. The server separately requires `--allow-write` to advertise the mutation tool. An explicitly supplied authorized short-lived token through `SYNMETRA_GOOGLE_ACCESS_TOKEN` can be used for write testing. The server will otherwise return a Google 403 without mutating anything.

Annotation inputs: `property_id`, `title` (1–60 characters), `start_date` (YYYY-MM-DD), optional `end_date` (not earlier than start), optional `description` (up to 150 characters), `color` (PURPLE, BROWN, BLUE, GREEN, RED or CYAN; default BLUE). ORANGE is Google-reserved and rejected. Google calls use `v1alpha` Admin API over rustls, and writes never auto-retry.

**Verification:** synthetic tests validate color/date/length restrictions, API path, flag-dependent tool visibility, and rejection without credentials. No live annotation has been created; live write acceptance is a separate user-authorized test-property gate. MCP client host confirmation is recommended; Synmetra's explicit `--allow-write` is an operator capability gate, not a per-call interactive confirmation.

Do not expose `--allow-write` over a publicly reachable HTTP transport.

**Native edit consent:** The new `auth login --client-id-file PATH --allow-edit` mode explicitly requests Google Analytics read and edit scopes with PKCE. Regular login remains read-only, and the token response must include *every* requested scope before credentials are saved. Locally stored read+edit credentials do not enable writes by themselves; the MCP server still defaults to no `create_property_annotation` tool. Synthetic scope tests pass only after CI, and an actual Google consent / annotation write remains unverified.
