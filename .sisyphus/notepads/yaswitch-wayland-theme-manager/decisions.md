# Decisions — yaswitch-wayland-theme-manager

- Language: Rust.
- Reload policy: safe-skip for unsupported/unsafe reload paths.
- UX: GUI-first, but must preserve keyboard-shortcut-compatible command route.
- ThemeSpec v1 manifest default: `theme.yaml` or `theme.json` (exactly one) with required keys from plan.
