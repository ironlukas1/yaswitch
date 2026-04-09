# Issues — yaswitch-wayland-theme-manager

- No implementation files exist yet; all tasks must create from scratch.
- No test/CI baseline exists; early tasks must establish cargo/test/lint/CI gates.
- Plan still references deleted draft path in a few task reference lines; replace with plan file references during implementation.

- Initial module-folder scaffold (`src/{core,adapters,palette,wallpaper,ui}`) was intentionally left as empty directories only; adding module declarations in `lib.rs` without backing files causes immediate Rust diagnostics.
- No blockers remained after constraining task to compile-clean skeleton only.

- Tests require the invalid fixture to omit `targets`; serde_yaml missing-field error is surfaced as THEME_SCHEMA_INVALID.
