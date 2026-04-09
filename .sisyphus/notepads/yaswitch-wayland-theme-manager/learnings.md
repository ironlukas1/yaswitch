# Learnings — yaswitch-wayland-theme-manager

- Repo started greenfield: only `its`, `session-ses_2ac9.md`, and `.sisyphus` planning files exist.
- Lock-ins from planning: Rust, safe-skip for unsafe reload, phased adapter rollout, GUI-first UX.
- Phase-1 compositor support policy: Tier-1 full apply (Sway/Hyprland/Niri), Tier-2 compatibility safe-skip (DWL/MangoWM).
- Critical architecture principle: one shared core apply pipeline for CLI/GUI/shortcut paths.

- Task 1 scaffold completed: created minimal Rust crate entrypoints (`src/lib.rs`, `src/main.rs`, `src/bin/yaswitch-gui.rs`) plus fixture placeholders under `tests/fixtures/`.
- Added baseline cargo aliases in `.cargo/config.toml` and minimal lint/format config files (`clippy.toml`, `rustfmt.toml`) to keep bootstrap checks stable.
- Verification status: `cargo check --workspace`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace --all-features` all pass.

- ThemeSpec v1 validation implemented with strict schema gate (yaml/json exactly one), hex palette checks, and required keys enforced.
- CLI validate-theme command now emits THEME_SCHEMA_INVALID on schema failure and exits non-zero.
