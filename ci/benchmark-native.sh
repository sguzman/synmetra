#!/usr/bin/env bash
# Offline, synthetic Linux latency/RSS microbenchmark. No Google API requests.
set -euo pipefail
export LC_ALL=C

binary="${1:-target/debug/synmetra}"
runs="${SYNMETRA_BENCH_RUNS:-5}"
if [[ ! "$runs" =~ ^[1-9][0-9]?$ ]] || (( runs > 20 )); then
  echo "SYNMETRA_BENCH_RUNS must be an integer from 1 to 20" >&2
  exit 2
fi
for utility in jq /usr/bin/time git; do
  command -v "$utility" >/dev/null || {
    echo "Missing Linux benchmark tool: $utility" >&2
    exit 2
  }
done
binary="$(realpath "$binary")"
test -x "$binary" || { echo "Executable not found: $binary" >&2; exit 2; }
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/xdg"

initialize='{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"synmetra-bench","version":"1"}}}'
initialized='{"jsonrpc":"2.0","method":"notifications/initialized"}'
list='{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}'
call='{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"ga4_get_started","arguments":{}}}'
printf '%s\n' "$initialize" "$initialized" "$list" "$call" > "$scratch/protocol.jsonl"

: > "$scratch/measurements.jsonl"
for ((i = 1; i <= runs; i++)); do
  /usr/bin/time -f '%e %M' -o "$scratch/timing" \
    env -u SYNMETRA_GOOGLE_ACCESS_TOKEN -u SYNMETRA_MCP_BEARER_TOKEN \
      XDG_CONFIG_HOME="$scratch/xdg" "$binary" --help > "$scratch/help.txt"
  read -r elapsed rss < "$scratch/timing"
  grep -Fq 'Synmetra' "$scratch/help.txt"
  printf '{"run":%d,"profile":"fresh_help","elapsed_s":%s,"max_rss_kib":%s}\n' \
    "$i" "$elapsed" "$rss" >> "$scratch/measurements.jsonl"

  /usr/bin/time -f '%e %M' -o "$scratch/timing" \
    env -u SYNMETRA_GOOGLE_ACCESS_TOKEN -u SYNMETRA_MCP_BEARER_TOKEN \
      XDG_CONFIG_HOME="$scratch/xdg" "$binary" serve \
      < "$scratch/protocol.jsonl" > "$scratch/responses.jsonl" 2> "$scratch/err.txt"
  read -r elapsed rss < "$scratch/timing"
  jq -s -e '
    length == 3
    and any(.[]; .id == 1 and .result.serverInfo.name == "synmetra")
    and any(.[]; .id == 2 and (.result.tools | length) == 20)
    and any(.[]; .id == 3 and .result.structuredContent.ok == true)
  ' "$scratch/responses.jsonl" > /dev/null
  test ! -s "$scratch/err.txt"
  printf '{"run":%d,"profile":"fresh_mcp_roundtrip","elapsed_s":%s,"max_rss_kib":%s}\n' \
    "$i" "$elapsed" "$rss" >> "$scratch/measurements.jsonl"
done

# A separate process contains multiple calls in one initialized session.
# This measures the process lifecycle plus all calls, not individual latency.
{
  printf '%s\n' "$initialize" "$initialized" "$list"
  for ((i = 1; i <= runs; i++)); do
    printf '{"jsonrpc":"2.0","id":%d,"method":"tools/call","params":{"name":"ga4_get_started","arguments":{}}}\n' "$((i + 2))"
  done
} > "$scratch/repeated.jsonl"
/usr/bin/time -f '%e %M' -o "$scratch/timing" \
  env -u SYNMETRA_GOOGLE_ACCESS_TOKEN -u SYNMETRA_MCP_BEARER_TOKEN \
    XDG_CONFIG_HOME="$scratch/xdg" "$binary" serve \
    < "$scratch/repeated.jsonl" > "$scratch/repeated-responses.jsonl" 2> "$scratch/repeated-err.txt"
read -r elapsed rss < "$scratch/timing"
jq -s -e --argjson runs "$runs" '
  length == ($runs + 2)
  and ([.[] | select(.result.structuredContent.ok == true)] | length) == $runs
' "$scratch/repeated-responses.jsonl" > /dev/null
test ! -s "$scratch/repeated-err.txt"
printf '{"run":1,"profile":"same_process_repeated_calls","calls":%d,"elapsed_s":%s,"max_rss_kib":%s}\n' \
  "$runs" "$elapsed" "$rss" >> "$scratch/measurements.jsonl"

jq -s --arg sha "$(git rev-parse HEAD)" --arg kernel "$(uname -srmo)" \
  --arg binary "$binary" --arg note 'Synthetic offline test; fresh process is not an OS cold page cache. Timings include process startup/teardown and full calls. No latency SLO is claimed.' \
  '{source_sha:$sha, kernel:$kernel, binary:$binary, notes:$note, measurements:.}' \
  "$scratch/measurements.jsonl"
