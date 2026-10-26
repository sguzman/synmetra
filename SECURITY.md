# Security Policy

## Supported versions

Synmetra is an **experimental** Rust Google Analytics MCP server at version `0.0.1`. No release is certified for production Google Analytics accounts, public hosting or multi-tenant usage. The default development branch may receive security fixes; there is no supported stable release or backport promise yet.

| Release line | Security status |
| --- | --- |
| `main` (`0.0.1` experimental) | Best-effort vulnerability triage; not production-supported |
| Any future stable `v1.x` | Not yet released |
| Older snapshots / untracked forks | No security support commitment |

## Reporting a vulnerability privately

**Do not include secrets, property identifiers, private analytics, OAuth redirect URLs, proof-of-concept credentials or exploit details in a public issue, pull request, commit or discussion.**

1. Visit the repository [Security Advisories page](https://github.com/sguzman/synmetra/security/advisories). If GitHub offers **Report a vulnerability**, use that private form to send the details to the repository maintainers.
2. If **Report a vulnerability** is not available, private vulnerability reporting may be disabled. Do **not** submit the vulnerability itself publicly. You may open a *metadata-only* GitHub issue requesting a private disclosure channel, with no technical details, test secrets, reproduced exploit path or affected account names. Do not assume this fallback is a confidential transport.
3. The repository owner must enable [GitHub private vulnerability reporting](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configure-for-a-repository) to support guaranteed private intake. **This configuration has not been independently verified here**. Do not claim the reporting channel is active until tested.

A useful **private** report contains the commit SHA, affected release/profile, vulnerability class, impact, minimized synthetic reproducer, expected/observed behavior and whether the issue requires an authorized Google account. Do not transmit real Google tokens or actual customer analytics unless an explicit secure process is agreed.

## Handling and coordination

Maintainers should acknowledge and triage privately, reproduce using synthetic fixtures where possible, prioritize credential exposure, Google OAuth/MCP identity separation, SQL/file escapes and unauthorized mutation, and agree on remediation/disclosure timing with the reporter. No response-time or bounty promise is made at the experimental stage. Publish a security advisory and affected/fixed versions when a fix has been validated.

Potential exposures include native DuckDB/C++ dependencies, loopback MCP bearer handling, OAuth token rotation, malicious Google provider payloads, and tool argument injection. Avoid publicly linking unpatched exploit details. Source changes should be committed directly to the established project workflow with clean synthetic test evidence; never put private reports or tokens into public commits.

For operational control status and unresolved security gates, see [docs/SECURITY.md](docs/SECURITY.md), [docs/LOCAL_EXECUTION_THREAT_MODEL.md](docs/LOCAL_EXECUTION_THREAT_MODEL.md), and [docs/progress.json](docs/progress.json).
