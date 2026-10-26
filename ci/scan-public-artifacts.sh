#!/usr/bin/env bash
# Redacted synthetic/experimental release artifact pre-upload guard.
# Never prints the matched credential or private contents.
set -euo pipefail
export LC_ALL=C
umask 077

patterns=(
  'gh[pousr]_[[:alnum:]]{36}'
  'github_pat_[[:alnum:]_]{80,}'
  'AIza[[:alnum:]_-]{35}'
  'sk-proj-[[:alnum:]_-]{20,}'
  '-----BEGIN (RSA |EC |OPENSSH |DSA )?PRIVATE KEY-----'
)

scan_text() {
  local path="$1"
  test -f "$path" || {
    echo 'Expected release text file was absent' >&2
    return 1
  }
  local pattern
  for pattern in "${patterns[@]}"; do
    if grep -I -q -E -e "$pattern" -- "$path"; then
      echo 'High-confidence credential signature in a candidate release asset (match suppressed)' >&2
      return 1
    fi
  done
  if [[ -n "${GITHUB_WORKSPACE:-}" ]] && grep -Fq -- "$GITHUB_WORKSPACE" "$path"; then
    echo 'Absolute checkout path in candidate release text (value suppressed)' >&2
    return 1
  fi
}

scan_package() (
  set -euo pipefail
  local archive="$1" dist="$2"
  test -f "$archive" && test -d "$dist" || {
    echo 'Release archive or distribution directory missing' >&2
    exit 1
  }
  local root
  root="$(mktemp -d)"
  trap 'rm -rf "$root"' EXIT

  # Verify the known archive manifest *before extraction* so a staged
  # traversal path, extra file or unexpected symlink never reaches publish.
  tar -tzf "$archive" | sed 's#^\./##' | sed '/^$/d' | sort > "$root/observed" || { echo 'Unreadable release archive manifest' >&2; return 1; }
  printf '%s\n' INSTALL.md LICENSE README.md SOURCE_COMMIT synmetra | sort > "$root/expected"
  if ! cmp -s "$root/observed" "$root/expected"; then
    echo 'Experimental release archive contains an unexpected path' >&2
    exit 1
  fi
  mkdir -p "$root/unpacked"
  tar -xzf "$archive" -C "$root/unpacked" || { echo 'Invalid release archive extraction' >&2; return 1; }
  if [[ -n "$(find "$root/unpacked" -type l -print -quit)" ]]; then
    echo 'Release archive contains a symlink' >&2
    exit 1
  fi

  local name
  for name in README.md INSTALL.md LICENSE SOURCE_COMMIT; do
    scan_text "$root/unpacked/$name" || return 1
  done
  for name in \
    synmetra-experimental-linux-x86_64.tar.gz.sha256 \
    synmetra-rust-sbom.cdx.json \
    synmetra-native-runtime.json; do
    scan_text "$dist/$name" || return 1
  done
  test -x "$root/unpacked/synmetra" || { echo 'Expected executable missing from release archive' >&2; return 1; }
  echo 'Experimental release text/manifest guard passed (high-confidence signatures only)'
)

self_test() (
  local root
  root="$(mktemp -d)"
  trap 'rm -rf "$root"' EXIT
  mkdir -p "$root/dist" "$root/stage"
  printf '%s\n' 'offline synthetic README' > "$root/stage/README.md"
  printf '%s\n' 'synthetic install' > "$root/stage/INSTALL.md"
  printf '%s\n' 'synthetic license' > "$root/stage/LICENSE"
  printf '%040d\n' 0 > "$root/stage/SOURCE_COMMIT"
  printf '#!/bin/sh\nexit 0\n' > "$root/stage/synmetra"
  chmod 755 "$root/stage/synmetra"
  printf '%s\n' '{}' > "$root/dist/synmetra-rust-sbom.cdx.json"
  printf '%s\n' '{}' > "$root/dist/synmetra-native-runtime.json"
  printf '%s\n' 'synthetic checksum record' > "$root/dist/synmetra-experimental-linux-x86_64.tar.gz.sha256"
  local archive="$root/dist/synmetra-experimental-linux-x86_64.tar.gz"
  tar -C "$root/stage" -czf "$archive" .
  scan_package "$archive" "$root/dist" > "$root/success.log"
  # Construct a deliberately fake high-confidence token signature at runtime.
  # Never store a full credential-looking canary in the public Git worktree.
  local canary
  canary="$(printf 'ghp_%036d' 0)"
  printf '%s\n' "$canary" > "$root/stage/README.md"
  tar -C "$root/stage" -czf "$archive" .
  if scan_package "$archive" "$root/dist" > "$root/rejected.log" 2>&1; then
    echo 'Artifact scanner accepted its deliberately fake credential marker' >&2
    exit 1
  fi
  if grep -Fq "$canary" "$root/rejected.log"; then
    echo 'Artifact scanner exposed a canary value in diagnostics' >&2
    exit 1
  fi
  # Failure propagation must work inside "if scan_package", where Bash
  # normally suppresses errexit throughout the called function.
  printf '%s\n' 'offline synthetic README' > "$root/stage/README.md"
  printf '%s\n' "$canary" > "$root/dist/synmetra-rust-sbom.cdx.json"
  tar -C "$root/stage" -czf "$archive" .
  if scan_package "$archive" "$root/dist" > "$root/rejected.log" 2>&1; then
    echo 'Artifact scanner accepted a deliberately fake SBOM credential marker' >&2
    exit 1
  fi
  if grep -Fq "$canary" "$root/rejected.log"; then
    echo 'Artifact scanner exposed an SBOM canary in diagnostics' >&2
    exit 1
  fi

  printf '%s\n' '{}' > "$root/dist/synmetra-rust-sbom.cdx.json"
  printf '%s\n' 'unlisted' > "$root/stage/unexpected-file"
  tar -C "$root/stage" -czf "$archive" .
  if scan_package "$archive" "$root/dist" > "$root/rejected.log" 2>&1; then
    echo 'Artifact scanner accepted an unlisted release archive member' >&2
    exit 1
  fi
  echo 'Synthetic artifact scanner self-test passed: safe package accepted; archive, SBOM and unexpected path rejected without disclosure'
)

if [[ "${1:-}" == "--self-test" && "$#" -eq 1 ]]; then
  self_test
elif [[ "$#" -eq 2 ]]; then
  scan_package "$1" "$2"
else
  echo 'Usage: ci/scan-public-artifacts.sh --self-test | ARCHIVE DIST_DIRECTORY' >&2
  exit 2
fi
