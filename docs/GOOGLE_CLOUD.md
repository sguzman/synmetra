# 🔑 Google Cloud OAuth setup (native, no gcloud)

**State:** setup instructions for a locally owned Google Analytics account. Synmetra is experimental: real-user consent, property roles, API eligibility and Google response parity are **not yet accepted** in [LIVE_ACCEPTANCE_MATRIX.md](LIVE_ACCEPTANCE_MATRIX.md). Configuring a Google Cloud project is not evidence of a successful live test.

## 1. Enable exactly the APIs Synmetra uses

In [Google Cloud Console](https://console.cloud.google.com/) select or create a project under your control. In **APIs & Services → Library**, search for and enable **Google Analytics Admin API** and **Google Analytics Data API**. Enable both: account/property management and data/realtime reports use different API hosts. Review project service restrictions, quotas and applicable billing notices in the console. A Google Cloud project does not automatically grant Analytics property permissions.

Official Google references: [Admin API quickstart](https://developers.google.com/analytics/devguides/config/admin/v1/quickstart), [Data API quickstart](https://developers.google.com/analytics/devguides/reporting/data/v1/quickstart), [OAuth Desktop Apps](https://developers.google.com/identity/protocols/oauth2/native-app).

Google quickstarts may show `gcloud` for **their** sample programs; Synmetra's own native Desktop OAuth flow does **not** require installing or running `gcloud`, Python or npm.

## 2. Configure OAuth consent and a Desktop application client

Use **Google Auth Platform / OAuth consent** in the selected Cloud project (console navigation labels can change). Set the app name, user audience and contact details according to Google's requirements. If the app is in testing mode, add your authorized account as a test user when required. Testing-mode refresh tokens can be time-limited; do not assume credentials will remain valid indefinitely. Only request scopes Synmetra actually needs.

In **Clients / Credentials**, create an **OAuth client ID** whose application type is **Desktop app** (not Web application or service account). Download its JSON to a private user-readable file outside the Git working tree. The current Rust implementation expects an `installed` Desktop OAuth JSON object and a client ID ending in `.apps.googleusercontent.com`. The native login chooses a localhost callback port and uses PKCE/state; no manually configured public redirect or `gcloud auth application-default login` is necessary.

Never commit Desktop client JSON, `credentials.json`, authorization callback URLs, request captures, refresh tokens or unredacted account/property identifiers.

## 3. Distinguish Google scopes from property roles

| Use | Scope requested by Synmetra | Additional constraint |
| --- | --- | --- |
| Default account/property discovery and Data API reports | `https://www.googleapis.com/auth/analytics.readonly` | The consenting Google user must actually have access to a relevant GA4 account/property |
| Access audit reports | `analytics.readonly` | Google Analytics Administrator permissions are required at the appropriate resource level |
| Explicitly opt-in annotation creation | `analytics.readonly` plus `https://www.googleapis.com/auth/analytics.edit` | Edit-capable GA4 property role; the server also requires `--allow-write` and the client must explicitly approve the write |

Scopes are documented by [Google OAuth scope reference](https://developers.google.com/identity/protocols/oauth2/scopes). **Scopes and Cloud project IAM are not equivalent to GA4 account/property access**: manage the actual property membership through the Google Analytics UI. Do not elevate roles solely to make an experimental test pass. Some v1alpha funnel/conversion requests may fail due to API eligibility even with a valid read scope; record that limitation instead of inventing a success.

## 4. Offline diagnosis and optional authorized browser consent

The following examples use Fish on EndeavourOS; no Google credentials are needed for diagnostic commands:

```fish
synmetra --help
synmetra auth doctor
synmetra auth status
```

A consenting account owner can explicitly initiate read-only Desktop OAuth with their privately downloaded client JSON:

```fish
synmetra auth login --client-id-file ~/private/google-desktop-oauth.json
```

This opens a native browser-based Google consent interaction with a random loopback callback. For an authorized browser that cannot reach Synmetra's loopback port, `--headless` prints the consent URL and prompts for the **complete redirected localhost callback URL** after the person consents; the native CLI verifies the exact redirect and state before redeeming the PKCE authorization code. This is a one-time browser copy-back step, not a credential-sharing mechanism. Never put tokens or callback URLs into a public issue or commit. To opt in to editor-scoped annotation tests later, use `--allow-edit` **only when explicitly authorized** and separately start the MCP with `--allow-write`; default is read-only. Synmetra verifies the native OAuth granted scope before any mutation; an external `SYNMETRA_GOOGLE_ACCESS_TOKEN` override has no locally verifiable scope provenance and **cannot be used for writes**. Google still checks the property editor role independently.

The credentials file lives under `$XDG_CONFIG_HOME/synmetra/credentials.json` (default `~/.config/synmetra/credentials.json`) with private Unix permissions. `auth logout` deletes Synmetra's local file but does not revoke Google-side authorization; revoke separately through the Google Account permissions page if desired. Avoid exporting any long-lived secrets to shell history.

## Optional quota project (no gcloud runtime)

To attribute Google API quota to an operator-approved project, set `SYNMETRA_GOOGLE_QUOTA_PROJECT` to its **project ID or numeric project number** before launching Synmetra. It adds Google's `x-goog-user-project` request header to outbound Admin/Data API calls, including opted-in annotation writes. Invalid project values fail locally without contacting Google; the setting never changes a Google OAuth account's permissions. The consenting principal must have `serviceusage.services.use` on that project (usually the Service Usage Consumer role). A Google HTTP 403 may reflect that quota-project permission as well as an API/scope/property-role issue. See [Google Cloud's quota project documentation](https://docs.cloud.google.com/docs/quotas/set-quota-project). No Google project or live role has been verified for this experimental server.

## Optional explicit `authorized_user` application credentials (native Rust)

For a **user-authorized** `authorized_user` Google Application Default Credentials JSON that the operator already owns, Synmetra supports a separate read-only opt-in. Set **both** `SYNMETRA_GOOGLE_ADC_USER=1` and `GOOGLE_APPLICATION_CREDENTIALS=/absolute/private/user.json`. The path must be a regular, non-symlinked, mode-0600 file of at most 32 KiB, and no tokens or JSON contents should be copied to a repository, CI output or chat. The native refresh path uses the fixed Google OAuth token endpoint with rustls; short-lived access tokens and any refresh-token rotation are held only in process memory. The original ADC file is not modified. Synmetra's own saved XDG Desktop OAuth credentials take precedence.

This **does not** run `gcloud`, import implicit shared ADC, authorize service-account JSON or impersonated identities, or grant an annotation write. An explicit `SYNMETRA_GOOGLE_ACCESS_TOKEN` bearer override is separately supported for reads and takes precedence over this mode. A successful offline synthetic refresh does not establish live Google scope or property eligibility; T049–T052 and T178–T186 remain separately gated.

## 5. Diagnose without exposing secrets

- **No credentials**: run `synmetra auth doctor`; ensure a downloaded Desktop OAuth client JSON exists in a private folder, not in the repository.
- **OAuth callback/consent failure**: confirm a local browser can reach loopback `127.0.0.1`, selected OAuth account is an eligible tester and the client is a Desktop application. A denied request is not a reason to disable state/PKCE checks.
- **Google HTTP 401/403**: check authentication freshness, granted scopes, enabled APIs and actual GA4 property role; do not treat all 403s as one cause. `auth status` is offline and does not prove a particular property is accessible.
- **429/5xx**: Google client has bounded read-only retries and never automatically repeats annotation writes. Do not keep retrying a possibly applied write without inspecting the test property's annotations.
- **Eligible account, incompatible alpha API**: record endpoint/role eligibility and time, using only redacted non-identifying descriptions in public acceptance artifacts.

The real-account acceptance tasks T178–T186 must use a specifically authorized test property and a reproducible **redacted** ledger. No consent, account verification or mutation is performed by this document.
