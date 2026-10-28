# Mod Menu for Horizon

An independently written sandboxed `.hmod` by levzzz5154. Browse the mods installed
in the current Horizon instance, including native source mods and disabled mods.
This is not a port of another project's Mod Menu and includes no Minecraft assets.

- Press **M** while playing (rebind in Options → Mods → Mod Menu).
- Run **`/modmenu`**, or **`/modmenu text`** to search names, ids and mod types.
- Open **Options → Mods → Mod Menu → Settings**.
- Filter All / Enabled / Disabled, page through the list, select a row for details,
  or refresh after changing mods in the launcher. Read-only: no toggles or native
  rebuilds are triggered by this menu.

## Compatibility

Built against Minecraft 26.3 / Horizon nightly `83f15fa27` using mod API 0.1,
API level 0. Uses only `ui`, `input` and its own `storage` capabilities. It needs
**a launcher with the Mod Menu inventory feed** ([Horizon #3112](https://github.com/horizon-rust/horizon/pull/3112)) to enumerate installed mods.
On a launcher without the feed it shows an explicit update message, not a false
empty list. Other Horizon revisions have unverified support.

The launcher writes `horizon-mods/.horizon-mod-data/mod_menu/inventory.json` only
when Mod Menu is installed. The schema is in [docs/inventory-feed.md](docs/inventory-feed.md).
The file is refreshed after installs, removals, enable/disable changes and before
launching an instance. It contains display metadata, not account information or
absolute file paths. The mod reads it through Horizon's existing sandbox storage
API. Enabled means the launcher's selected state; it is not proof a mod's code is
running, and changes made during a session apply on the next launch.

## Build and validate

Requires Rust with `wasm32-unknown-unknown`, and `cargo-horizon-mod` from Horizon.
SDK and sandbox-test dependencies are pinned to the tested source revision.

```sh
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo horizon-mod build
cargo horizon-mod check target/horizon-mod/mod_menu.hmod
cargo run --manifest-path tools/sandbox/Cargo.toml --locked -- target/horizon-mod/mod_menu.hmod
```

The model tests cover parsing, malformed/incompatible feeds, disabled/native mods,
filters, search, page boundaries and display sanitation. The sandbox driver loads
the actual component and exercises the UI, commands, keyboard binding, settings,
refresh, absent feeds and compatibility errors. Builds contain only this mod's
manifest and compiled WebAssembly component. Source licence: [MIT](LICENSE). Validation results: [docs/validation.md](docs/validation.md).
