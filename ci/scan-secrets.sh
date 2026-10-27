#!/usr/bin/env bash
# Redacted public-tree and optional Git history credential scan.
# Matches, blob hashes and historical paths are never printed.
set -euo pipefail

if (( $# > 1 )) || { (( $# == 1 )) && [[ "$1" != "--history" ]]; }; then
  echo 'Usage: ci/scan-secrets.sh [--history]' >&2
  exit 2
fi

if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo 'Tracked source secret scan requires a Git repository' >&2
  exit 2
fi

# Patterns intentionally match high-confidence provider credential formats;
# generic words like "token" are not matches, because our offline synthetic
# regression suite needs redacted fake tokens for testing.
patterns=(
  'gh[pousr]_[[:alnum:]]{36}'
  'github_pat_[[:alnum:]_]{80,}'
  'AIza[[:alnum:]_-]{35}'
  'sk-proj-[[:alnum:]_-]{20,}'
  '-----BEGIN (RSA |EC |OPENSSH |DSA )?PRIVATE KEY-----'
)
failed=0
for expression in "${patterns[@]}"; do
  if git grep -I -q -E -e "$expression" -- .; then
    echo 'Potential real credential/private-key material in a tracked file; details suppressed' >&2
    failed=1
  fi
done
if ((failed)); then
  exit 1
fi

# --history requires complete reachable history (not GitHub's shallow
# checkout). Never let a shallow success masquerade as a history audit.
if [[ "${1:-}" == "--history" ]]; then
  if [[ "$(git rev-parse --is-shallow-repository)" != "false" ]]; then
    echo 'History secret scan requires an unshallowed Git checkout' >&2
    exit 2
  fi
  reviewed_historical_fixture_seen=false
  revisions="$(git rev-list --all)" || {
    echo 'Unable to enumerate reachable Git history' >&2
    exit 2
  }
  while IFS= read -r revision; do
    [[ -n "$revision" ]] || continue
    grep_args=(-I -q -E)
    for expression in "${patterns[@]}"; do
      grep_args+=(-e "$expression")
    done
    status=0
    git grep "${grep_args[@]}" "$revision" -- . >/dev/null 2>&1 || status=$?
    if ((status == 0)); then
      # One old scanner regression intentionally included a single naked
      # synthetic PEM header (without a footer/key payload). The exact file
      # was independently reviewed and is pinned by its Git *blob* hash.
      # This is NOT a file/path allowlist: a different byte in that file, a
      # copy elsewhere, or any other matching blob fails the history audit.
      reviewed_fixture='b661d59ad2b88048e16c48ce283cdcc3e751d8b6'
      only_reviewed_fixture=true
      found_match=false
      while IFS= read -r -d '' matched_path; do
        found_match=true
        matched_path="${matched_path#*:}"
        if [[ "$matched_path" != 'ci/test-secret-scanner.sh' ]]; then
          only_reviewed_fixture=false
          break
        fi
        object="$(git rev-parse --verify "$revision:$matched_path" 2>/dev/null)" || {
          only_reviewed_fixture=false
          break
        }
        if [[ "$object" != "$reviewed_fixture" ]]; then
          only_reviewed_fixture=false
          break
        fi
      done < <(git grep -I -l -z -E "${grep_args[@]:3}" "$revision" -- .)
      if [[ "$found_match" == true && "$only_reviewed_fixture" == true ]]; then
        reviewed_historical_fixture_seen=true
        continue
      fi
      echo 'Potential historical credential/private-key material in reachable Git history; details suppressed' >&2
      exit 1
    fi
    if ((status != 1)); then
      echo 'Historical credential scan could not read a reachable revision' >&2
      exit 2
    fi
  done <<< "$revisions"
  if [[ "$reviewed_historical_fixture_seen" == true ]]; then
    echo 'Reviewed immutable historical scanner-only test fixture: accepted (no key payload)'
  fi
  echo 'Other reachable Git history high-confidence credential signatures: none detected'
fi

# The scanner does not parse proprietary Google OAuth JSON, infer unrecognized
# secrets, or inspect unreachable/pruned Git objects. A passing heuristic
# source/history scan is not an independent secret-audit certificate.
echo 'Tracked-source high-confidence credential signatures: none detected'
