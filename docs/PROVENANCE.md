# ⚖️ Provenance and licensing

- **Synmetra:** [MIT](../LICENSE), as already selected by the repository owner. Do not rewrite the original author/copyright notice.
- **Google Analytics MCP:** Google LLC, [Apache-2.0](https://github.com/googleanalytics/google-analytics-mcp/blob/ce84226bf4cedea7f2910f5afb07a6b9138f6553/LICENSE), pinned at `ce84226bf4cedea7f2910f5afb07a6b9138f6553`.
- **SednaLabs GA4 MCP:** SednaLabs and contributors, [Apache-2.0](https://github.com/sednalabs/ga4-mcp/blob/039fd4620064fc4b76dc6bca641945aba52c14bc/LICENSE), pinned at `039fd4620064fc4b76dc6bca641945aba52c14bc`.

Synmetra aims to implement **public behavior/contracts** with independent Rust code. Reference documentation, tool names, Google Analytics REST API specifications and independent test fixtures are our starting points, not a blanket license to copy arbitrary source/assets. If Apache-2.0 code is actually adapted, preserve copyright notices, relevant LICENSE and NOTICE, identify adapted files and changes, and verify combined license obligations. This repository is not an official Google product.

**Public-source hygiene:** no credentials, auth outputs, private transcripts, personal website analytics, real property/account IDs, user home paths or unredacted error logs. CI fixtures must be synthetic. Auth acceptance uses local secrets and redacted records only.

## 📜 Pinned schema fixture used by contract tests

`spec/upstream/sednalabs-tool-schema.v1.json` is an **unchanged** schema snapshot from SednaLabs GA4 MCP at commit `039fd4620064fc4b76dc6bca641945aba52c14bc` (original path `spec/tool_schema_snapshot.v1.json`, Git blob `646c0baa22692b73430a0ee96415b0e2fe6215a3`). The upstream author retains attribution and copyright; copy distributed under Apache License 2.0, reproduced as `licenses/SEDNALABS-APACHE-2.0.txt`. This snapshot is reference material, not evidence that Synmetra implements those signatures. Do not modify it silently; update provenance and drift checks on reviewed upstream bumps.

## 📐 Contract V1 response schema

The read-only fixture [`spec/upstream/sednalabs-payload-contract-v1.schema.json`](../spec/upstream/sednalabs-payload-contract-v1.schema.json) is copied without modifications from `sednalabs/ga4-mcp` at commit `039fd4620064fc4b76dc6bca641945aba52c14bc`, original `spec/payload_contract_v1.schema.json`, under SednaLabs' Apache-2.0 license (already retained under `licenses/`). It defines the upstream success and failure envelope for future native MCP contract tests. Including it does not establish Synmetra's behavioral parity or end-to-end validation.
