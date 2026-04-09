# yaswitch

> **One-liner:** yaswitch makes Wayland theming fast, safe, and scriptable without sacrificing control.

Wayland-first theme manager with transactional applies, rollback safety, wallpaper-aware palette generation, and CLI-first keyboard shortcuts.

## Tiered compositor support

- **Tier-1 (full path):** Sway, Hyprland, Niri
- **Tier-2 (compatibility safe-skip):** DWL, MangoWM

Tier-2 targets remain non-destructive: when reload is unsupported, yaswitch reports explicit safe-skip reason codes and remediation guidance.

## Example theme

An executable sample is available at `examples/themes/minimal`.

## Run as direct executable (no cargo run)

From the repository root:

```bash
./yaswitch doctor --json
./yaswitch apply --theme examples/themes/minimal --compositor sway --dry-run --json
```

The `./yaswitch` launcher builds `target/release/yaswitch` on first run, then executes the binary directly.

Validate it with:

```bash
cargo run -- validate-theme examples/themes/minimal
```

## Current Status

Implemented core through:
- schema validation
- path sandboxing
- reason-code contract
- transaction journal and rollback
- template engine
- adapter contract
- preflight planner
- executor and report contract
- wallpaper manager and deterministic palette cache
- CI baseline

## Quickstart

```bash
cargo run -- validate-theme tests/fixtures/themes/valid-minimal
cargo run -- apply --theme tests/fixtures/themes/valid-minimal --dry-run --json
cargo run -- cycle --compositor sway --json
cargo run -- install-shortcut --compositor sway
cargo run -- doctor --json
```

## Documentation

- Compatibility: `docs/compatibility.md`
- Shortcuts: `docs/shortcuts.md`
- Integration snippets: `docs/integration/shortcuts.md`
