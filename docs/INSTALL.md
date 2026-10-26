# 🦀 Native Linux source installation

Synmetra is an **experimental GA4 MCP server**, not yet a parity-certified release. These instructions install the default, read-only Rust CLI and stdio transport. They do **not** install Python, Node, npm, gcloud, or a background daemon. The optional DuckDB scratchpad has a separate native C++ build boundary. For commands, flags, environment variables, permissions and Fish examples, see [CLI.md](CLI.md).

## EndeavourOS / Arch Linux

Install the native build tools with the distribution package manager:

```fish
yay -S --needed git rustup base-devel pkgconf
rustup toolchain install 1.99.1 --profile minimal
git clone https://github.com/sguzman/synmetra.git
cd synmetra
cargo +1.99.1 install --path . --locked --bin synmetra --root ~/.local
fish_add_path -U ~/.local/bin
synmetra auth doctor
```

The build uses the checked-in `Cargo.lock` and the explicitly requested Rust 1.99.1 toolchain. The `--root` argument isolates the installation under `~/.local/bin`, without writing to system directories. No account credential is required to build or run `auth doctor`. This copies only `synmetra`, not the internal progress-ledger utility.

## Optional local pacman package (Arch)

An independently pinned [local PKGBUILD](../packaging/arch/PKGBUILD) is available for users who want pacman to own Synmetra rather than Cargo's user-local root. It references a full reviewed Git commit, fetches locked Rust dependencies, builds offline, runs default Rust tests, and stages the executable and documentation. Install Rust 1.99.1 before running `makepkg -si` from the recipe directory.

The pinned PKGBUILD has now passed a non-root `makepkg` build, package installation, native library linkage inspection, offline `auth doctor` and complete package uninstall inside a disposable Arch Linux container in [GitHub Actions run 40369257171](https://github.com/sguzman/synmetra/actions/runs/40369257171). The test used Arch's `pacman -U/-R` internally; for an interactive EndeavourOS system, `yay` can install a locally built `.pkg.tar.zst` using `yay -U ./synmetra-0.0.1-1-x86_64.pkg.tar.zst`. This is **not yet** a clean host EndeavourOS install (T217), and the package builds the default feature profile without DuckDB.

## Other Linux distributions

Install Git, a C toolchain/linker, `pkg-config` and Rust via your distribution or rustup, then use the same `git clone` and `cargo +1.99.1 install --path . --locked --bin synmetra --root "$HOME/.local"` commands. Add `$HOME/.local/bin` to your shell's PATH. For Fish, use `fish_add_path -U ~/.local/bin`. Default builds use native Rust TLS through rustls; Python, npm, Node and gcloud are not needed at runtime.

## Verify and use

Run `synmetra auth doctor` and `synmetra auth status`. The former provides configuration guidance without revealing tokens. Configure a Google Desktop OAuth client only when ready to access an authorized Analytics property. The default `synmetra serve` mode speaks MCP JSON-RPC over stdio; it does not expose a public port or enable annotation writes.

To build locally without installing, use `cargo +1.99.1 build --locked --bin synmetra`, then run `target/debug/synmetra auth doctor`. Optional experimental scratchpad builds use `cargo +1.99.1 build --locked --features scratchpad` and bundle DuckDB, which also requires a working native C/C++ build environment. See [scratchpad caveats](SCRATCHPAD.md).

## Upgrade and uninstall

From the cloned source directory, update the pinned source revision and rerun `cargo +1.99.1 install --path . --locked --bin synmetra --root ~/.local --force`. Do **not** automatically update a pinned installation to arbitrary upstream code without reviewing changes.

Uninstall the installed executable with `cargo +1.99.1 uninstall --root ~/.local synmetra`. Removing an executable does **not** delete the separate local OAuth configuration, tokens or your Analytics account permissions. Use `synmetra auth logout` **before** uninstalling if you want Synmetra's stored credentials cleared. Google-side access revocation is a separate action.

## Automated verification

The CI core job performs a clean, isolated `cargo install --path . --locked --debug --bin synmetra` into a temporary root, invokes the installed binary with an empty executable PATH and temporary configuration, verifies that no credentials appear in stdout, and uninstalls it. The `--debug` flag avoids an unnecessary second optimized compile in every push; the native release artifact is a separate acceptance milestone. No manual QA is required from the user for this source-install gate.

## Optional experimental native ARM64 binary

A default-feature `aarch64` Linux archive with SHA-256 sidecar and linked-library audit was built and verified on a **native** GitHub-hosted ARM64 Linux runner. See [verified experimental ARM64 artifact](https://github.com/sguzman/synmetra/actions/runs/40400440461/artifacts/12688357449) and [release policy](RELEASE_POLICY.md). This is an expiring Actions artifact, **not** a permanent or supported `v1.0.0` release. It excludes the optional bundled DuckDB feature and has not been clean-installed on ARM64 EndeavourOS. Source installation remains the documented default for that environment.
