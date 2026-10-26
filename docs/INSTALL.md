# 🦀 Native Linux source installation

Synmetra is an **experimental GA4 MCP server**, not yet a parity-certified release. These instructions install the default, read-only Rust CLI and stdio transport. They do **not** install Python, Node, npm, gcloud, or a background daemon. The optional DuckDB scratchpad has a separate native C++ build boundary.

## EndeavourOS / Arch Linux

Install the native build tools with the distribution package manager:

```fish
sudo pacman -S --needed git rustup base-devel pkgconf
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

The PKGBUILD has static shell and metadata verification in CI. A native Arch makepkg build/install smoke has **not yet been accepted**; the verified Cargo source installation above is the currently tested Linux route.

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
