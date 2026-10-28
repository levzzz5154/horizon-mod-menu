# Validation for 0.1.1

Tested with Horizon source `83f15fa276c44711276b6ceb5f0fe0c9c534dc66` (26.3),
mod API 0.1, API level 0.

- Formatting and Clippy with warnings denied: passed.
- Four model tests: passed.
- `.hmod` built and checked with `cargo-horizon-mod`: passed; registry trial run
  started, ran 40 ticks and 120 frames, and invoked `/modmenu` successfully.
- Actual component loaded in Horizon's sandbox: passed. The driver verifies
  enabled/disabled native and sandboxed rows, counts, pagination, filters,
  case-insensitive search, selected details, refresh, the M binding and settings.
- Missing and unsupported inventory feeds show an explicit message: passed.
- Launcher inventory scan: packaged/manual mods, corrupt files, native metadata,
  disabled entries, opt-in storage and absence of absolute paths: passed.

The menu lists installation/configuration state. It cannot diagnose runtime mod
failures and needs a launcher supporting the schema-1 inventory feed. No sandbox
capability or ownership check is bypassed.

An offscreen game session using prepared 26.3 resources and fresh temporary
homes confirmed the menu at the default GUI scale on the integrated-server
path. M and typing `/modmenu` both opened the screen. The list showed Mod Menu
0.1.1 enabled, Coordinates HUD disabled, and the native Veinminer bundle disabled.
The 0.1.1 row labels and summary fit the 200-pixel menu width. A screenshot is
included with the release. No real player folders were modified.
