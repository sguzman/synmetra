#!/usr/bin/env bash
# Exercise the exact upstream-drift workflow shell body with mocked curl/gh.
# The test makes no real network requests or maintenance issues.
set -euo pipefail
cd "$(dirname "$0")/.."
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
awk '/^        run: \|$/ { copy=1; next } copy { sub(/^          /, ""); print }' \
  .github/workflows/upstream-drift.yml > "$tmp/watch.sh"
bash -n "$tmp/watch.sh"
test "$(grep -c 'exit "\$changed"' "$tmp/watch.sh")" -eq 1
mkdir "$tmp/bin"
export GITHUB_REPOSITORY=sguzman/synmetra
export GITHUB_STEP_SUMMARY="$tmp/summary"
export SYNMETRA_TEST_ISSUES="$tmp/issues"
export GH_TOKEN=fixture-not-a-credential
cat > "$tmp/bin/curl" <<'MOCK_CURL'
#!/usr/bin/env bash
set -euo pipefail
source=
for arg in "$@"; do
  case "$arg" in
    *sednalabs/ga4-mcp/contents/*) source=sednalabs ;;
    *googleanalytics/google-analytics-mcp/contents/*) source=googleanalytics ;;
  esac
done
test -n "$source"
pinned="$(jq -er --arg source "$source" '.[$source].blob_sha' spec/upstream/drift-lock.json)"
if test "$source" = "$SYNMETRA_TEST_CHANGED"; then
  printf '{"sha":"%040d"}\n' 1
else
  printf '{"sha":"%s"}\n' "$pinned"
fi
MOCK_CURL
cat > "$tmp/bin/gh" <<'MOCK_GH'
#!/usr/bin/env bash
set -euo pipefail
case "$1 $2" in
  'api --paginate')
    if test -s "$SYNMETRA_TEST_ISSUES"; then
      jq -R -s 'split("\n") | map(select(length > 0) | {title:.,number:1})' \
        "$SYNMETRA_TEST_ISSUES"
    else
      echo '[]'
    fi
    ;;
  'issue create')
    while test "$#" -gt 0; do
      if test "$1" = "--title"; then
        printf '%s\n' "$2" >> "$SYNMETRA_TEST_ISSUES"
        exit 0
      fi
      shift
    done
    echo 'Missing mocked issue title' >&2
    exit 1
    ;;
  *)
    echo 'Unexpected mocked GitHub command' >&2
    exit 1
    ;;
esac
MOCK_GH
chmod +x "$tmp/bin/gh" "$tmp/bin/curl"
export PATH="$tmp/bin:$PATH"

SYNMETRA_TEST_CHANGED= bash "$tmp/watch.sh"
test ! -s "$SYNMETRA_TEST_ISSUES"
if SYNMETRA_TEST_CHANGED=sednalabs bash "$tmp/watch.sh"; then
  echo 'Expected new SednaLabs drift failure' >&2; exit 1
fi
grep -Fxq '[upstream-drift] Review sednalabs signature source' \
  "$SYNMETRA_TEST_ISSUES"
test "$(wc -l < "$SYNMETRA_TEST_ISSUES")" -eq 1
if SYNMETRA_TEST_CHANGED=sednalabs bash "$tmp/watch.sh"; then
  echo 'Expected existing SednaLabs drift failure' >&2; exit 1
fi
test "$(wc -l < "$SYNMETRA_TEST_ISSUES")" -eq 1
if SYNMETRA_TEST_CHANGED=googleanalytics bash "$tmp/watch.sh"; then
  echo 'Expected new Google signature drift failure' >&2; exit 1
fi
grep -Fxq '[upstream-drift] Review googleanalytics signature source' \
  "$SYNMETRA_TEST_ISSUES"
test "$(wc -l < "$SYNMETRA_TEST_ISSUES")" -eq 2
SYNMETRA_TEST_CHANGED= bash "$tmp/watch.sh"
test "$(wc -l < "$SYNMETRA_TEST_ISSUES")" -eq 2
echo 'Offline upstream drift detection and deduplication fixture tests passed'
