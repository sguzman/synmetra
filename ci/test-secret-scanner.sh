#!/usr/bin/env bash
# Synthetic controls for the source credential scanner. Never echo matched data.
set -euo pipefail
script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
scanner="$script_dir/scan-secrets.sh"
root="$(mktemp -d)"
trap 'rm -rf -- "$root"' EXIT
repo="$root/fixture"
mkdir -p -- "$repo"
git -C "$repo" init -q
git -C "$repo" config user.name 'Synmetra synthetic fixture'
git -C "$repo" config user.email 'synthetic@example.invalid'

assert_clean() {
  local output="$root/scan.log"
  if ! (cd -- "$repo" && bash "$scanner") > "$output" 2>&1; then
    echo 'Secret scanner rejected a synthetic safe fixture' >&2
    exit 1
  fi
  grep -Fq 'none detected' "$output" || {
    echo 'Secret scanner omitted clean result' >&2
    exit 1
  }
}

assert_rejected_redacted() {
  local needle="$1" output="$root/scan.log"
  if (cd -- "$repo" && bash "$scanner") > "$output" 2>&1; then
    echo 'Secret scanner failed to reject synthetic credential fixture' >&2
    exit 1
  fi
  if grep -Fq -- "$needle" "$output"; then
    echo 'Secret scanner exposed a synthetic secret in diagnostic output' >&2
    exit 1
  fi
  if ! grep -Fq 'details suppressed' "$output"; then
    echo 'Secret scanner omitted safe redacted failure explanation' >&2
    exit 1
  fi
}

printf 'TOKEN = "synthetic-unit-test-marker"\n' > "$repo/fixture.txt"
git -C "$repo" add -- fixture.txt
assert_clean

# Build credential-shaped canaries at runtime; don't check in full strings.
repeater=$(printf '%036d' 0)
for fixture in \
  "ghp_$repeater" \
  "gho_$repeater" \
  "github_pat_$(printf '%080d' 0)" \
  "AIza$(printf '%035d' 0)" \
  "sk-proj-$(printf '%020d' 0)" \
  "-----BEGIN "${KEY_TYPE:-PRIVATE}" KEY-----"; do
  printf 'const synthetic = "%s";\n' "$fixture" > "$repo/fixture.txt"
  git -C "$repo" add -- fixture.txt
  assert_rejected_redacted "$fixture"
done

# Scan tracked worktree content, not only committed/index content.
printf 'sample near-miss = ghp_%s\n' "$(printf '%035d' 0)" > "$repo/fixture.txt"
assert_clean

# Reproduce the previously invisible threat: a token-shaped string committed
# in an old revision and scrubbed from the current tracked worktree.
git -C "$repo" commit -q -m 'baseline safe text' -- fixture.txt
assert_clean
(cd -- "$repo" && bash "$scanner" --history) > "$root/scan.log" 2>&1 || {
  echo 'Full history scanner rejected a clean synthetic repository' >&2
  exit 1
}
historic_canary="ghp_$(printf '%036d' 9)"
printf '%s\n' "$historic_canary" > "$repo/fixture.txt"
git -C "$repo" add fixture.txt
git -C "$repo" commit -q -m 'synthetic history canary'
printf 'clean committed state\n' > "$repo/fixture.txt"
git -C "$repo" add fixture.txt
git -C "$repo" commit -q -m 'redact synthetic fixture from HEAD'
assert_clean

# A current-tree scanner alone cannot see this, but --history must deny it
# without echoing the token, blob SHA, path or commit contents.
if (cd -- "$repo" && bash "$scanner" --history) > "$root/scan.log" 2>&1; then
  echo 'Historical credential scanner missed a removed committed canary' >&2
  exit 1
fi
grep -Fq 'historical credential/private-key material' "$root/scan.log" || {
  echo 'Historical secret scanner omitted a safe error category' >&2
  exit 1
}
if grep -Fq -- "$historic_canary" "$root/scan.log"; then
  echo 'Historical secret scanner disclosed its synthetic matched canary' >&2
  exit 1
fi

# A full-history claim from a shallow checkout must fail closed.
shallow="$root/shallow"
git clone -q --depth 1 "file://$repo" "$shallow"
if (cd -- "$shallow" && bash "$scanner" --history) > "$root/scan.log" 2>&1; then
  echo 'Shallow history incorrectly received a passing full-history scan' >&2
  exit 1
fi
grep -Fq 'unshallowed Git checkout' "$root/scan.log" || {
  echo 'Shallow source error was not diagnosed safely' >&2
  exit 1
}

echo 'Synthetic current and historical secret-scanner fixtures passed without disclosing matches'
