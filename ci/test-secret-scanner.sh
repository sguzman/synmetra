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
  "-----BEGIN PRIVATE KEY-----"; do
  printf 'const synthetic = "%s";\n' "$fixture" > "$repo/fixture.txt"
  git -C "$repo" add -- fixture.txt
  assert_rejected_redacted "$fixture"
done

# Scan tracked worktree content, not only committed/index content.
printf 'sample near-miss = ghp_%s\n' "$(printf '%035d' 0)" > "$repo/fixture.txt"
assert_clean
echo 'Synthetic secret-scanner positive/negative fixtures passed without disclosing matches'
