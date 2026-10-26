#!/usr/bin/env bash
# Two independent raw MCP JSON-RPC client profiles over the installed stdio
# transport. rmcp SDK's separate in-memory client also runs in core cargo tests.
# Never contact Google or use the real operator's XDG config.
set -euo pipefail
export LC_ALL=C
umask 077

binary="${1:-target/debug/synmetra}"
test -x "$binary" || { echo 'Missing native MCP binary' >&2; exit 2; }
command -v jq >/dev/null || { echo 'jq is required for conformance verification' >&2; exit 2; }
command -v timeout >/dev/null || { echo 'timeout is required for bounded conformance verification' >&2; exit 2; }

root="$(mktemp -d)"
trap 'rm -rf "$root"' EXIT
mkdir -p "$root/xdg"

for version in 2025-06-18 2025-11-25; do
  client="synthetic-raw-client-${version}"
  printf '%s\n' \
    "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{\"protocolVersion\":\"${version}\",\"capabilities\":{},\"clientInfo\":{\"name\":\"${client}\",\"version\":\"1.0\"}}}" \
    '{"jsonrpc":"2.0","method":"notifications/initialized"}' \
    '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' \
    '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"ga4_get_started","arguments":{}}}' \
    '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"get_account_summaries","arguments":{}}}' \
    > "$root/requests.jsonl"

  timeout 15s env -u SYNMETRA_GOOGLE_ACCESS_TOKEN \
    -u SYNMETRA_MCP_BEARER_TOKEN \
    XDG_CONFIG_HOME="$root/xdg" "$binary" serve \
    < "$root/requests.jsonl" \
    > "$root/responses.jsonl" \
    2> "$root/stderr.txt"

  jq --slurp --exit-status --arg version "$version" '
    length == 4
    and all(.[]; .jsonrpc == "2.0")
    and any(.[]; .id == 1
      and .result.serverInfo.name == "synmetra"
      and (.result.protocolVersion | type == "string")
      and (.result.capabilities.tools | type == "object"))
    and any(.[]; .id == 2
      and (.result.tools | length) == 20
      and (.result.tools | all(.inputSchema.type == "object"))
      and (.result.tools | all(.name != "create_property_annotation")))
    and any(.[]; .id == 3
      and .result.isError == false
      and .result.structuredContent.ok == true
      and (.result.structuredContent.data | type == "object"))
    and any(.[]; .id == 4
      and .result.isError == true
      and .result.structuredContent.ok == false
      and (.result.structuredContent.error.code | type == "string")
      and (.result.structuredContent.error.category | type == "string"))
    and all(.[]; (.error | not))
  ' "$root/responses.jsonl" > /dev/null

  if grep -Eiq 'access_token": "|refresh_token": "|client_secret": "' \
    "$root/responses.jsonl" "$root/stderr.txt"; then
    echo 'Conformance output unexpectedly contained a credential field' >&2
    exit 1
  fi
  if test -s "$root/stderr.txt"; then
    echo 'Synthetic raw MCP conformance emitted unexpected diagnostics' >&2
    exit 1
  fi
  printf 'Raw MCP client profile %s: initialize, discover, success and typed error passed\n' "$version"
done
echo 'Two isolated raw stdio MCP clients verified; rmcp SDK client tests run separately in cargo test'
