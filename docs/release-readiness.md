# Release Readiness Checklist (v1 non-GUI)

This checklist is required before tagging a release.

## Core verification

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo build --workspace --all-features`
- [ ] `cargo test --workspace --all-features`
- [ ] `./yaswitch doctor --json`

## Reliability verification

- [ ] `cargo test --test e2e_atomic_recovery -- --nocapture`
- [ ] `cargo test --test e2e_lock_contention -- --nocapture`
- [ ] `cargo test --test e2e_gui_cli_parity -- --nocapture`

## Compatibility verification

- [ ] `cargo test --test compositor_compat_matrix -- --nocapture`
- [ ] `cargo test --test compositor_tier2_safe_skip -- --nocapture`
- [ ] `cargo run -- apply --theme tests/fixtures/themes/valid-minimal --compositor dwl --json`

## Structured reporting verification

- [ ] `cargo run -- apply --theme tests/fixtures/themes/valid-minimal --dry-run --json`
- [ ] `cargo run -- apply --theme tests/fixtures/themes/valid-vscode --target vscode --dry-run --json`

## Documentation and examples verification

- [ ] `cargo run -- validate-theme examples/themes/minimal`
- [ ] `cargo test --test docs_consistency -- --nocapture`

## Scope lock for this phase

- [ ] Wayland-only support remains explicit.
- [ ] Tier-1 full support and Tier-2 safe-skip behavior are accurately documented.
- [ ] No GUI-only code path is required for successful release checks.
