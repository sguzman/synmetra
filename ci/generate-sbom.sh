#!/usr/bin/env bash
# Reproducible source-linked Rust dependency inventory for an experimental binary.
# Native runtime libraries remain separately reported; this is NOT a complete
# license or transitive bundled-C++ attestation.
set -euo pipefail
export LC_ALL=C

binary="${1:-target/release/synmetra}"
destination="${2:-dist}"
test -x "$binary" || { echo 'A compiled native Synmetra binary is required' >&2; exit 2; }
for utility in cargo jq git ldd sha256sum awk sort; do
  command -v "$utility" >/dev/null || { echo "Missing SBOM tool: $utility" >&2; exit 2; }
done
mkdir -p "$destination"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
source_sha="$(git rev-parse --verify HEAD)"
binary_sha="$(sha256sum "$binary" | cut -d ' ' -f1)"

# Resolve all locked Rust packages, including opt-in native DuckDB bindings,
# without copying local manifest paths or cache roots into the published BOM.
cargo +1.99.1 metadata --locked --all-features --format-version 1 > "$scratch/metadata.json"
jq --arg source_sha "$source_sha" '
  . as $manifest
  | ($manifest.packages
      | map({key:.id,value:("pkg:cargo/" + .name + "@" + .version)})
      | from_entries) as $refs
  | {
      bomFormat:"CycloneDX",
      specVersion:"1.5",
      version:1,
      metadata:{
        component:{
          type:"application",
          name:"synmetra",
          version:($manifest.packages | map(select(.name == "synmetra")) | first | .version),
          "bom-ref":($manifest.packages | map(select(.name == "synmetra")) | first | $refs[.id])
        },
        properties:[
          {name:"synmetra:source-commit",value:$source_sha},
          {name:"synmetra:dependency-scope",value:"Cargo.lock resolved with --all-features; native runtime built with default features"},
          {name:"synmetra:license-review",value:"Rust declared licenses are source metadata, not proof of native C++ or system-library redistribution compliance"}
        ]
      },
      components:[
        $manifest.packages[]
        | select(.name != "synmetra")
        | {
            type:"library",
            name:.name,
            version:.version,
            purl:$refs[.id],
            "bom-ref":$refs[.id],
            licenses:(if .license == null then [] else [{license:{name:.license}}] end),
            properties:[{name:"cargo:source",value:(.source // "workspace")}]
          }
      ],
      dependencies:[
        $manifest.resolve.nodes[]
        | {
            ref:$refs[.id],
            dependsOn:([.deps[]?.pkg | $refs[.]] | unique)
          }
      ]
    }
' "$scratch/metadata.json" > "$destination/synmetra-rust-sbom.cdx.json"

jq -e '
  .bomFormat == "CycloneDX"
  and .specVersion == "1.5"
  and (.components | length > 100)
  and (([.components[]."bom-ref"] | unique | length) == (.components | length))
  and (.dependencies | length > 100)
' "$destination/synmetra-rust-sbom.cdx.json" > /dev/null

# Report runtime loader names without publishing absolute runner paths.
ldd "$binary" > "$scratch/ldd.txt"
if grep -Fq 'not found' "$scratch/ldd.txt"; then
  echo 'Unresolved native shared library' >&2
  exit 1
fi
awk 'NF && $1 != "statically" { print $1 }' "$scratch/ldd.txt" | sort -u \
  > "$scratch/runtime-libraries.txt"
jq -Rn \
  --arg commit "$source_sha" \
  --arg digest "$binary_sha" \
  '{
    schema:"synmetra_native_runtime_inventory",
    version:1,
    source_commit:$commit,
    binary_sha256:$digest,
    host_linker_view:"ldd on the x86-64 Linux CI runner; not a dependency license opinion",
    native_libraries:[inputs | select(length > 0)]
  }' < "$scratch/runtime-libraries.txt" \
  > "$destination/synmetra-native-runtime.json"
jq -e '.schema == "synmetra_native_runtime_inventory"
  and (.native_libraries | length >= 2)
  and (.binary_sha256 | length == 64)' \
  "$destination/synmetra-native-runtime.json" > /dev/null

echo 'Machine-readable Rust SBOM and linked-library inventory validated'
