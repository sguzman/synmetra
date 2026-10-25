# 📌 Source-pinned MCP schema reference

- Source: SednaLabs ga4-mcp, pinned commit `039fd4620064fc4b76dc6bca641945aba52c14bc`.
- File: [tool_schema_snapshot.v1.json](tool_schema_snapshot.v1.json), copied unchanged from [upstream original](https://github.com/sednalabs/ga4-mcp/blob/039fd4620064fc4b76dc6bca641945aba52c14bc/spec/tool_schema_snapshot.v1.json).
- License: upstream Apache-2.0, see [LICENSE-SednaLabs.txt](LICENSE-SednaLabs.txt). Attribution retained; Synmetra's original independently written Rust code remains MIT.
- Purpose: test against 35 published SednaLabs MCP tool input schemas without depending on its runtime or Python. This is a **reference fixture**, not implementation or parity evidence.
- Google's Python MCP has a separately generated dynamic tool schema; it has **not** yet been snapshotted. Do not claim exhaustive 36-tool signature parity.

The upstream file is JSON data, and source updates require an explicit diff and a newly pinned commit. Never silently overwrite this fixture from main.
