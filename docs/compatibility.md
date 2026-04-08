# Compatibility Matrix (v1)

| Target | Tier | Status | Reload Behavior |
|---|---|---|---|
| Sway | Tier-1 | Full path | Reload supported |
| Hyprland | Tier-1 | Full path | Reload supported |
| Niri | Tier-1 | Full path | Reload supported |
| DWL | Tier-2 | Compatibility | Safe-skip (`SKIP_RELOAD_UNSUPPORTED`) |
| MangoWM | Tier-2 | Compatibility | Safe-skip (`SKIP_RELOAD_UNSUPPORTED`) |

This reflects current implementation behavior and is enforced by tests.

Machine-readable verification is covered by:

- `cargo test --test compositor_compat_matrix -- --nocapture`
- `cargo test --test compositor_tier2_safe_skip -- --nocapture`

These tests emit/validate compatibility matrix artifacts and version-guard behavior.
