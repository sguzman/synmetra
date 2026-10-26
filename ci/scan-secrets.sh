#!/usr/bin/env bash
# Redacted public-tree heuristic scan. No matched credential strings are printed.
set -euo pipefail

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

# The scanner does not attempt to parse proprietary Google OAuth JSON,
# infer unrecognized account secrets, or inspect Git history. A passing
# heuristic scan is not an independent secret-audit certificate.
echo 'Tracked-source high-confidence credential signatures: none detected'
