# yaswitch — Wayland Theme Manager (Rust, GUI-first)

## TL;DR
> **Summary**: Build `yaswitch` as a Rust, Wayland-only theme manager with transactional apply/rollback, wallpaper-derived palettes, GUI-first UX, and keyboard shortcut compatibility.
> **Deliverables**:
> - Rust CLI+GUI application with shared core engine
> - Theme schema and directory model at `~/.config/yaswitch/themes/<theme>/...`
> - Transaction-safe apply pipeline (backup, atomic writes, rollback)
> - Adapter matrix for Sway/Hyprland/Niri full path + DWL/MangoWM safe-skip compatibility in v1
> - Wallpaper switching + deterministic palette generation
> - CI + tests + evidence artifacts for agent-executable verification
> **Effort**: XL
> **Parallel**: YES - 4 waves
> **Critical Path**: T1 → T2 → T7 → T9 → T10 → T11 → T15/T16 → T19 → T22 → Final Verification

## Context
### Original Request
Build a Linux Wayland-only color scheme/theme switcher named `yaswitch` that supports Sway, Hyprland, DWL, MangoWM, and Niri; stores per-theme app dotfiles under `~/.config/yaswitch/themes/<theme>/...`; supports wallpaper switching and palette extraction; offers menu + keyboard-shortcut compatibility; and targets broad app coverage over time.

### Interview Summary
- Implementation language locked: **Rust**.
- Reload policy locked: **safe-skip** when hot reload is unsafe/unavailable.
- Rollout locked: **phased adapter strategy**.
- UX locked: **GUI-first**, while retaining keyboard-shortcut-compatible command path.
- Repository is greenfield: no source code/test/CI currently exists.
- Applied defaults for decision completeness:
  - GUI framework default: **egui/eframe** for v1.
  - Palette backend order: **matugen CLI primary**, deterministic in-process fallback.
  - Phase-1 compositor support tiers: **Tier-1 full apply = Sway/Hyprland/Niri**; **Tier-2 compatibility safe-skip = DWL/MangoWM**.

### Metis Review (gaps addressed)
- Added strict guardrails for transactional writes, path sandboxing, and adapter capability checks.
- Added explicit scope boundary to prevent v1 overreach.
- Added acceptance gates for rollback recovery, deterministic palette behavior, and concurrency locking.
- Added explicit edge-case handling for missing theme fields, unreadable wallpaper, concurrent apply operations, and unavailable compositor sockets.

## Work Objectives
### Core Objective
Deliver a production-safe v1 of `yaswitch` with reliable theme application, state safety, and clear compatibility reporting across target Wayland compositors.

### Deliverables
1. Rust project structure with shared core engine used by both GUI and command invocation paths.
2. Versioned theme schema + validator for `~/.config/yaswitch/themes/<theme>/...`.
3. Transactional apply/rollback system with backup journal.
4. Wallpaper manager + deterministic palette pipeline.
5. Compositor/app adapter framework with capability matrix and safe-skip reason codes.
6. GUI-first interaction path wired to the same apply engine as keyboard-triggered flow.
7. CI/test harness with reproducible evidence artifacts.

### Locked Contracts (decision-complete defaults)
#### ThemeSpec v1 (required)
- Theme root: `~/.config/yaswitch/themes/<theme-name>/`
- Required manifest file: `theme.yaml` (or `theme.json`, exactly one)
- Required manifest keys:
  - `schema_version: 1`
  - `theme_name: <string>`
  - `palette: {base00..base0F}` (16 canonical hex entries)
  - `targets: { <target-id>: { template: <path>, destination: <path>, mode: inject|overwrite } }`
  - `wallpaper: { path: <path>, mode: fit|fill|center }`
- Optional keys:
  - `adapter_overrides`, `metadata`, `notes`

#### Apply failure policy (required)
- Critical mutation failure => **rollback-all** within transaction scope.
- Unsupported/unsafe reload => **safe-skip + reason code** (non-fatal).
- Optional adapter failure (configured optional target) => continue apply, mark degraded status.

#### Runtime paths (required)
- State file: `~/.local/share/yaswitch/state.json`
- Backup journal: `~/.local/share/yaswitch/backups/<timestamp>/`
- Cache: `~/.cache/yaswitch/`

### Definition of Done (verifiable conditions with commands)
- `cargo fmt --all -- --check` exits `0`.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` exits `0`.
- `cargo test --workspace --all-features` exits `0`.
- `cargo run -- validate-theme ~/.config/yaswitch/themes/example` exits `0` for valid fixtures and non-zero for invalid fixtures.
- `cargo run -- apply --theme example --compositor sway --dry-run --json` outputs `status:"planned"` and no filesystem mutation.
- `cargo run -- apply --theme example --compositor dwl --json` exits `0` with explicit `SKIP_RELOAD_UNSUPPORTED` reason code.
- `cargo test --workspace e2e_atomic_recovery` exits `0` proving rollback consistency.

### Must Have
- Wayland-only support model.
- Theme root at `~/.config/yaswitch/themes/<theme>/...`.
- Safe-skip behavior for unsupported reload operations.
- Backup + atomic write + rollback before any destructive mutation.
- GUI-first workflow plus keyboard shortcut compatibility.
- Machine-readable apply report per adapter (applied/skipped/failed/rolled_back).
- Phase-1 support matrix enforced in code/docs: Tier-1 (Sway/Hyprland/Niri), Tier-2 compatibility-safe-skip (DWL/MangoWM).

### Must NOT Have (guardrails, AI slop patterns, scope boundaries)
- No blind process restarts to force theme consistency.
- No non-Wayland support in v1.
- No plugin marketplace/runtime extension system in v1.
- No app/compositor logic hardcoded in GUI layer.
- No direct writes outside approved target paths and yaswitch-managed backup/state directories.

## Verification Strategy
> ZERO HUMAN INTERVENTION — all verification is agent-executed.
- Test decision: **TDD** using Rust unit + integration tests (plus deterministic fixture-based smoke tests).
- Frameworks: `cargo test`, integration harness with fixture directories, command-level smoke checks via Bash.
- QA policy: Every task includes happy + failure scenario with binary pass/fail conditions.
- Evidence: `.sisyphus/evidence/task-{N}-{slug}.{ext}`

## Execution Strategy
### Parallel Execution Waves
> Target: 5-8 tasks per wave. Shared dependencies are extracted early.

Wave 1: Foundation + contracts (T1–T6)
- Project/toolchain bootstrap, schema contract, path safety, error/result model, test harness, CI baseline.

Wave 2: Core transactional engine (T7–T12)
- Backup/journal/atomic writes, template/injection pipeline, adapter contracts, planning/execution/rollback, reporting.

Wave 3: Integrations + compatibility matrix (T13–T18)
- Wallpaper and palette pipeline, compositor adapters, app adapters, safe-skip compatibility adapters.

Wave 4: GUI-first UX + system hardening (T19–T24)
- GUI flow, keyboard-shortcut compatibility integration, dry-run diagnostics UX, e2e suite, packaging/docs/sample themes.

### Dependency Matrix (full, all tasks)
| Task | Depends On | Notes |
|---|---|---|
| T1 | - | Bootstrap |
| T2 | T1 | Theme schema contract |
| T3 | T1 | Path/XDG safety |
| T4 | T1 | Error/result contract |
| T5 | T1 | Test scaffolding |
| T6 | T1,T5 | CI requires test scripts |
| T7 | T2,T3,T4 | Transaction layer depends on schema+path+errors |
| T8 | T2,T7 | Template/injection needs schema + file safety |
| T9 | T4 | Adapter contract uses result model |
| T10 | T7,T9 | Planning uses transactions + capabilities |
| T11 | T8,T9,T10 | Executor ties all core pieces |
| T12 | T10,T11 | Reporting based on planner+executor outputs |
| T13 | T3,T4 | Wallpaper manager uses safe paths + errors |
| T14 | T13 | Palette generation consumes wallpaper manager |
| T15 | T9,T10,T11 | Tier-1 compositor adapters plug into core |
| T16 | T9,T12 | Compatibility safe-skip adapters + diagnostics |
| T17 | T8,T9,T11 | App adapters depend on injection and executor |
| T18 | T17,T12 | VSCode adapter + restart-risk reporting |
| T19 | T10,T11,T12 | GUI must call shared plan/apply/report APIs |
| T20 | T15,T16,T19 | Shortcut compatibility depends on compositor matrix + GUI command path |
| T21 | T12,T19 | Diagnostics UI/CLI based on report contract |
| T22 | T6,T11,T15,T17,T19 | End-to-end test suite |
| T23 | T15,T16,T22 | Compositor compatibility smoke matrix |
| T24 | T19,T20,T22,T23 | Packaging/docs after behavior is stable |

### Agent Dispatch Summary (wave → task count → categories)
| Wave | Task Count | Primary Categories |
|---|---:|---|
| Wave 1 | 6 | unspecified-high, quick |
| Wave 2 | 6 | deep, unspecified-high |
| Wave 3 | 6 | deep, unspecified-high |
| Wave 4 | 6 | visual-engineering, deep, writing |

## TODOs
> Implementation + Test = ONE task. Never separate.
> EVERY task includes Agent Profile + Parallelization + QA Scenarios.

- [x] 1. Bootstrap Rust workspace and project skeleton

  **What to do**: Create a Rust workspace for `yaswitch` with `src/lib.rs`, `src/main.rs`, `src/bin/yaswitch-gui.rs`, `tests/`, `fixtures/`, and module folders (`core`, `adapters`, `palette`, `wallpaper`, `ui`). Add strict lint/format config and baseline command aliases in `Cargo.toml`.
  **Must NOT do**: Do not add application logic in this task; do not include non-Wayland targets.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: foundation architecture and file layout.
  - Skills: [`coding-standards`, `tdd-workflow`] — enforce idiomatic Rust and test-first scaffolding.
  - Omitted: [`frontend-patterns`] — no UI behavior implemented yet.

  **Parallelization**: Can Parallel: NO | Wave 1 | Blocks: T2,T3,T4,T5,T6 | Blocked By: none

  **References** (executor has NO interview context — be exhaustive):
  - Requirement source: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:936-938` — primary product requirements and `~/.config/yaswitch` model.
  - Inspiration index: `/home/lunix/Projects/theme-changer/its:1` — source projects to benchmark structure.
  - Planning decisions: `/home/lunix/Projects/theme-changer/.sisyphus/drafts/yaswitch-wayland-theme-manager.md` — locked decisions (Rust, safe-skip, phased rollout, GUI-first).
  - Rust workspace reference: `https://doc.rust-lang.org/cargo/reference/workspaces.html` — canonical workspace setup.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo check --workspace` exits `0`.
  - [ ] `cargo fmt --all -- --check` exits `0`.
  - [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` exits `0`.
  - [ ] Repository contains paths: `src/lib.rs`, `src/main.rs`, `src/bin/yaswitch-gui.rs`, `tests/fixtures/`.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Workspace builds cleanly
    Tool: Bash
    Steps: Run `cargo check --workspace`; run `cargo fmt --all -- --check`; run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
    Expected: All commands exit 0 with no warnings elevated to errors
    Evidence: .sisyphus/evidence/task-1-bootstrap-workspace.txt

  Scenario: Missing-file guard
    Tool: Bash
    Steps: Run `test -f src/bin/yaswitch-gui.rs && test -f src/lib.rs && test -d tests/fixtures`
    Expected: Command exits 0; if any path is absent task fails
    Evidence: .sisyphus/evidence/task-1-bootstrap-workspace-error.txt
  ```

  **Commit**: YES | Message: `chore(scaffold): initialize rust workspace for yaswitch` | Files: `Cargo.toml`, `src/*`, `tests/fixtures/*`, lint config files

- [x] 2. Define ThemeSpec v1 schema and validator

  **What to do**: Specify and implement `ThemeSpec v1` as typed Rust models plus JSON/TOML schema validation for `~/.config/yaswitch/themes/<theme>/...`. Mandate fields for palette metadata, target app mappings, optional wallpaper metadata, and per-adapter overrides.
  **Must NOT do**: Do not silently coerce invalid values; do not auto-fill critical missing keys.

  **Recommended Agent Profile**:
  - Category: `deep` — Reason: contract design impacts all downstream modules.
  - Skills: [`backend-patterns`, `tdd-workflow`] — schema-first modeling with failing tests.
  - Omitted: [`frontend-patterns`] — no UI behavior.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: T7,T8,T10 | Blocked By: T1

  **References** (executor has NO interview context — be exhaustive):
  - Requirement source: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:936-938` — per-theme dotfiles requirement.
  - Inspiration index: `/home/lunix/Projects/theme-changer/its:1` — external schema inspiration set.
  - Existing pattern insight: `https://github.com/InioX/matugen` — palette structure conventions.
  - Existing pattern insight: `https://github.com/nitinbhat972/cwal` — template-driven mapping patterns.
  - New files from T1: `src/core/theme_spec.rs`, `tests/theme_spec_validation.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test theme_spec_validation_accepts_valid_fixture` exits `0`.
  - [ ] `cargo test theme_spec_validation_rejects_missing_required_keys` exits `0`.
  - [ ] `cargo run -- validate-theme tests/fixtures/themes/valid-minimal` exits `0`.
  - [ ] `cargo run -- validate-theme tests/fixtures/themes/invalid-missing-app-map` exits non-zero and prints `THEME_SCHEMA_INVALID`.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Valid theme passes schema gate
    Tool: Bash
    Steps: Run `cargo run -- validate-theme tests/fixtures/themes/valid-minimal --json`
    Expected: JSON output includes `"status":"valid"` and exit code 0
    Evidence: .sisyphus/evidence/task-2-theme-spec.txt

  Scenario: Invalid theme blocked before mutation
    Tool: Bash
    Steps: Run `cargo run -- apply --theme tests/fixtures/themes/invalid-missing-app-map --dry-run --json`
    Expected: Non-zero exit, output includes `THEME_SCHEMA_INVALID`, and no output files are created under `.cache/yaswitch`
    Evidence: .sisyphus/evidence/task-2-theme-spec-error.txt
  ```

  **Commit**: YES | Message: `feat(schema): add ThemeSpec v1 validation contract` | Files: `src/core/theme_spec.rs`, `tests/theme_spec_validation.rs`, fixtures

- [x] 3. Implement XDG path resolver and path-sandbox guardrails

  **What to do**: Implement canonical path resolution for config/state/cache/backups and enforce path sandboxing that rejects path traversal and symlink escape outside allowed roots.
  **Must NOT do**: Do not permit writes outside user-approved target files and yaswitch-managed state/backup directories.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: security/safety-critical filesystem logic.
  - Skills: [`security-review`, `tdd-workflow`] — defensive path handling and tests.
  - Omitted: [`frontend-patterns`] — no UI scope.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: T7,T13 | Blocked By: T1

  **References** (executor has NO interview context — be exhaustive):
  - Requirement source: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:936-938` — `~/.config/yaswitch` storage model.
  - XDG spec reference: `https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html`.
  - Security baseline: `https://owasp.org/www-community/attacks/Path_Traversal`.
  - New files from T1: `src/core/paths.rs`, `tests/path_sandbox.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test path_sandbox_blocks_dotdot_escape` exits `0`.
  - [ ] `cargo test path_sandbox_blocks_symlink_escape` exits `0`.
  - [ ] `cargo test xdg_path_resolution_uses_expected_defaults` exits `0`.
  - [ ] `cargo run -- doctor --json` reports resolved paths for config/state/cache/backups.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: XDG defaults resolve correctly
    Tool: Bash
    Steps: Run `cargo run -- doctor --json`
    Expected: JSON contains keys `config_dir`, `state_dir`, `cache_dir`, `backup_dir` under user home paths
    Evidence: .sisyphus/evidence/task-3-xdg-paths.txt

  Scenario: Path traversal input rejected
    Tool: Bash
    Steps: Run `cargo test path_sandbox_blocks_dotdot_escape -- --nocapture`
    Expected: Test passes and explicit rejection code `PATH_OUTSIDE_ALLOWED_ROOT` appears in output
    Evidence: .sisyphus/evidence/task-3-xdg-paths-error.txt
  ```

  **Commit**: YES | Message: `feat(paths): add xdg resolution and sandbox guardrails` | Files: `src/core/paths.rs`, `tests/path_sandbox.rs`

- [x] 4. Create typed error/result model and reason-code taxonomy

  **What to do**: Define a unified typed error/result model with stable reason codes (`THEME_SCHEMA_INVALID`, `SKIP_RELOAD_UNSUPPORTED`, `ROLLBACK_APPLIED`, etc.) used by CLI, GUI, reports, and tests.
  **Must NOT do**: Do not use ad-hoc string errors in adapter/core boundaries.

  **Recommended Agent Profile**:
  - Category: `quick` — Reason: focused contract task with high downstream impact.
  - Skills: [`coding-standards`, `backend-patterns`] — consistent error semantics.
  - Omitted: [`visual-engineering`] — no UI rendering in this task.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: T7,T9,T10,T12,T16 | Blocked By: T1

  **References** (executor has NO interview context — be exhaustive):
  - Requirement source: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:936-938` — robustness and broad integration requirement.
  - Metis risk output: `.sisyphus/drafts/yaswitch-wayland-theme-manager.md` — safe-skip and deterministic reporting decisions.
  - Rust error handling reference: `https://doc.rust-lang.org/book/ch09-00-error-handling.html`.
  - New files from T1: `src/core/result.rs`, `tests/result_codes.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test result_codes_are_stable_and_serializable` exits `0`.
  - [ ] `cargo run -- validate-theme tests/fixtures/themes/invalid-missing-app-map --json` includes `THEME_SCHEMA_INVALID`.
  - [ ] `cargo run -- apply --theme tests/fixtures/themes/valid-minimal --compositor dwl --json` includes `SKIP_RELOAD_UNSUPPORTED` once T16 is complete.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Error code contract is machine-readable
    Tool: Bash
    Steps: Run `cargo test result_codes_are_stable_and_serializable -- --nocapture`
    Expected: Test passes and serialized payload contains `code` and `category` fields
    Evidence: .sisyphus/evidence/task-4-result-codes.txt

  Scenario: Unknown error fallback prevented
    Tool: Bash
    Steps: Run `cargo test disallow_untyped_error_boundary -- --nocapture`
    Expected: Test passes; any untyped boundary returns compile/test failure
    Evidence: .sisyphus/evidence/task-4-result-codes-error.txt
  ```

  **Commit**: YES | Message: `feat(core): standardize reason codes and typed results` | Files: `src/core/result.rs`, `tests/result_codes.rs`

- [x] 5. Build TDD harness, fixtures, and evidence output conventions

  **What to do**: Establish fixture directories for themes, wallpapers, and expected adapter outputs. Implement reusable test helpers and evidence writer conventions to `.sisyphus/evidence/` paths.
  **Must NOT do**: Do not hardcode host-specific filesystem assumptions in tests.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: test infrastructure foundation for all waves.
  - Skills: [`tdd-workflow`, `verification-loop`] — robust repeatable test setup.
  - Omitted: [`frontend-patterns`] — no UI implementation in this task.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: T6,T22,T23 | Blocked By: T1

  **References** (executor has NO interview context — be exhaustive):
  - Testing gap report: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:1385-1452` — no existing test infra.
  - Evidence policy: this plan’s `Verification Strategy` section.
  - Recommended path: `tests/fixtures/` and `.sisyphus/evidence/`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test fixtures_can_be_loaded_for_all_core_suites` exits `0`.
  - [ ] `cargo test evidence_writer_creates_expected_paths` exits `0`.
  - [ ] Running one smoke test writes at least one file under `.sisyphus/evidence/`.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Fixture harness works across suites
    Tool: Bash
    Steps: Run `cargo test fixtures_can_be_loaded_for_all_core_suites`
    Expected: Exit 0 and output lists loaded fixture groups (themes, wallpapers, adapters)
    Evidence: .sisyphus/evidence/task-5-test-harness.txt

  Scenario: Evidence write failure detected
    Tool: Bash
    Steps: Run `cargo test evidence_writer_fails_on_unwritable_path`
    Expected: Exit 0 with expected controlled error code (not panic)
    Evidence: .sisyphus/evidence/task-5-test-harness-error.txt
  ```

  **Commit**: YES | Message: `test(infra): add fixtures and evidence harness` | Files: `tests/fixtures/*`, `tests/support/*`, evidence helpers

- [x] 6. Add CI workflow enforcing format/lint/test gates

  **What to do**: Add CI pipeline that runs formatting, linting, unit/integration tests, and stores artifacts/logs for failed runs.
  **Must NOT do**: Do not add flaky environment-dependent compositor E2E in baseline CI.

  **Recommended Agent Profile**:
  - Category: `quick` — Reason: focused infra wiring.
  - Skills: [`deployment-patterns`, `verification-loop`] — stable CI gates.
  - Omitted: [`visual-engineering`] — unrelated.

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: T22,T23 | Blocked By: T1,T5

  **References** (executor has NO interview context — be exhaustive):
  - Test baseline recommendation: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:1429-1447`.
  - GitHub Actions reference: `https://docs.github.com/actions`.
  - Rust CI practices: `https://github.com/rust-lang/cargo` docs for checks/tests.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `.github/workflows/ci.yml` exists and includes jobs for fmt, clippy, test.
  - [ ] Local simulation commands (`cargo fmt`, `cargo clippy`, `cargo test`) all exit `0`.
  - [ ] CI config uploads failure logs/artifacts for test jobs.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: CI config validates expected jobs
    Tool: Bash
    Steps: Parse workflow file and verify job names `fmt`, `clippy`, `test` exist
    Expected: Validation script exits 0
    Evidence: .sisyphus/evidence/task-6-ci-baseline.txt

  Scenario: Gate catches lint regression
    Tool: Bash
    Steps: Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
    Expected: Command fails on intentional warning fixture and passes after cleanup
    Evidence: .sisyphus/evidence/task-6-ci-baseline-error.txt
  ```

  **Commit**: YES | Message: `ci(core): enforce fmt clippy and test gates` | Files: `.github/workflows/ci.yml`

- [x] 7. Implement transactional backup journal and atomic file writer

  **What to do**: Build core mutation primitives: snapshot backup, transaction journal entries, atomic write/rename, rollback restore API, and crash-recovery replay.
  **Must NOT do**: Do not perform in-place edits without backup checkpoints.

  **Recommended Agent Profile**:
  - Category: `deep` — Reason: high-risk safety-critical subsystem.
  - Skills: [`security-review`, `tdd-workflow`] — correctness + failure resilience.
  - Omitted: [`frontend-patterns`] — no UI layer work.

  **Parallelization**: Can Parallel: NO | Wave 2 | Blocks: T8,T10,T11 | Blocked By: T2,T3,T4

  **References** (executor has NO interview context — be exhaustive):
  - Oracle guardrails summary from session continuation (transactional writes required).
  - Metis directives: adapter operations must be rollback-safe.
  - Rust filesystem docs: `https://doc.rust-lang.org/std/fs/`.
  - New files: `src/core/transaction.rs`, `tests/transaction_recovery.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test transaction_writes_are_atomic` exits `0`.
  - [ ] `cargo test transaction_rolls_back_on_mid_apply_failure` exits `0`.
  - [ ] `cargo test transaction_recovers_after_simulated_crash` exits `0`.
  - [ ] No mutation path bypasses transaction APIs (enforced by tests/lints).

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Successful commit persists new content
    Tool: Bash
    Steps: Run `cargo test transaction_writes_are_atomic -- --nocapture`
    Expected: Exit 0 and fixture file content matches expected post-commit hash
    Evidence: .sisyphus/evidence/task-7-transaction-core.txt

  Scenario: Crash recovery restores previous state
    Tool: Bash
    Steps: Run `cargo test transaction_recovers_after_simulated_crash -- --nocapture`
    Expected: Exit 0 and rollback marker `ROLLBACK_APPLIED` appears in output
    Evidence: .sisyphus/evidence/task-7-transaction-core-error.txt
  ```

  **Commit**: YES | Message: `feat(core): add transactional backup and atomic write engine` | Files: `src/core/transaction.rs`, tests

- [x] 8. Implement template rendering and marker-based injection engine

  **What to do**: Implement template rendering for app targets plus marker-block replacement (`yaswitch:begin` / `yaswitch:end`) for non-destructive in-file updates.
  **Must NOT do**: Do not overwrite unrelated user-managed config sections.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: nuanced text processing and safety behavior.
  - Skills: [`backend-patterns`, `tdd-workflow`] — deterministic rendering and replacements.
  - Omitted: [`visual-engineering`] — UI not in scope.

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T11,T17 | Blocked By: T2,T7

  **References** (executor has NO interview context — be exhaustive):
  - Inspiration pattern: `https://github.com/victorsosaMx/vshypr-theme-manager` marker injection model.
  - Inspiration pattern: `https://github.com/nitinbhat972/cwal` template strategy.
  - New files: `src/core/template_engine.rs`, `tests/template_engine.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test template_renders_expected_values` exits `0`.
  - [ ] `cargo test marker_injection_replaces_only_managed_block` exits `0`.
  - [ ] `cargo test marker_injection_appends_block_when_allowed` exits `0`.
  - [ ] Injection operation uses transaction primitives from T7.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Managed block updated without side effects
    Tool: Bash
    Steps: Run `cargo test marker_injection_replaces_only_managed_block -- --nocapture`
    Expected: Exit 0 and diff shows only bounded marker section changed
    Evidence: .sisyphus/evidence/task-8-template-injection.txt

  Scenario: Missing marker handled safely
    Tool: Bash
    Steps: Run `cargo test marker_injection_rejects_when_append_disallowed`
    Expected: Exit 0 and reason code `MARKER_NOT_FOUND_APPEND_DISABLED`
    Evidence: .sisyphus/evidence/task-8-template-injection-error.txt
  ```

  **Commit**: YES | Message: `feat(core): add template rendering and marker injection` | Files: `src/core/template_engine.rs`, tests

- [x] 9. Define adapter capability contract and compliance test suite

  **What to do**: Define trait contract for adapters (`plan`, `apply`, `verify`, `rollback`, `capabilities`) and create reusable compliance tests all adapters must pass.
  **Must NOT do**: Do not allow adapter implementations without explicit capability declarations.

  **Recommended Agent Profile**:
  - Category: `deep` — Reason: cross-cutting architecture contract.
  - Skills: [`backend-patterns`, `tdd-workflow`] — robust interface design.
  - Omitted: [`frontend-patterns`] — not needed.

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T10,T11,T15,T16,T17,T18 | Blocked By: T4

  **References** (executor has NO interview context — be exhaustive):
  - Oracle guidance captured in this planning cycle: plan/apply/verify/rollback pattern.
  - Metis directive: capability matrix and safe-skip reason codes mandatory.
  - New files: `src/adapters/contract.rs`, `tests/adapter_contract.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test adapter_contract_requires_capability_metadata` exits `0`.
  - [ ] `cargo test adapter_contract_enforces_safe_skip_semantics` exits `0`.
  - [ ] A dummy adapter passes full contract compliance tests.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Contract suite validates compliant adapter
    Tool: Bash
    Steps: Run `cargo test adapter_contract_happy_path`
    Expected: Exit 0 for compliant fixture adapter
    Evidence: .sisyphus/evidence/task-9-adapter-contract.txt

  Scenario: Non-compliant adapter rejected
    Tool: Bash
    Steps: Run `cargo test adapter_contract_rejects_missing_capabilities`
    Expected: Exit 0 (test passes by detecting rejection) and reason code present
    Evidence: .sisyphus/evidence/task-9-adapter-contract-error.txt
  ```

  **Commit**: YES | Message: `feat(adapters): define capability contract and compliance tests` | Files: `src/adapters/contract.rs`, tests

- [x] 10. Build apply planner with preflight risk evaluation

  **What to do**: Implement preflight planner that calculates all intended mutations/reloads before apply, including risk labels and safe-skip decisions by adapter capability.
  **Must NOT do**: Do not execute file or process mutation in planning mode.

  **Recommended Agent Profile**:
  - Category: `deep` — Reason: core orchestration logic with dependency fan-out.
  - Skills: [`backend-patterns`, `tdd-workflow`] — deterministic planning contract.
  - Omitted: [`visual-engineering`] — no UI rendering changes.

  **Parallelization**: Can Parallel: NO | Wave 2 | Blocks: T11,T12,T19 | Blocked By: T2,T4,T7,T9

  **References** (executor has NO interview context — be exhaustive):
  - Oracle recommendation from planning: plan-before-apply mandatory.
  - Metis directive: safe-skip unsupported reloads as first-class outcomes.
  - New files: `src/core/planner.rs`, `tests/planner_preflight.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test planner_generates_mutation_graph_for_valid_theme` exits `0`.
  - [ ] `cargo test planner_marks_unsupported_reload_as_safe_skip` exits `0`.
  - [ ] `cargo run -- apply --theme tests/fixtures/themes/valid-minimal --dry-run --json` outputs full action graph without mutation.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Dry-run exposes complete plan
    Tool: Bash
    Steps: Run `cargo run -- apply --theme tests/fixtures/themes/valid-minimal --dry-run --json`
    Expected: JSON includes ordered `actions[]`, `risk_level`, and `expected_outcome` per adapter
    Evidence: .sisyphus/evidence/task-10-preflight-planner.txt

  Scenario: Unsupported reload converted to safe-skip
    Tool: Bash
    Steps: Run `cargo test planner_marks_unsupported_reload_as_safe_skip -- --nocapture`
    Expected: Exit 0 and reason code `SKIP_RELOAD_UNSUPPORTED` present
    Evidence: .sisyphus/evidence/task-10-preflight-planner-error.txt
  ```

  **Commit**: YES | Message: `feat(core): add preflight planner and risk evaluation` | Files: `src/core/planner.rs`, tests

- [x] 11. Implement apply executor with deterministic rollback behavior

  **What to do**: Build executor that consumes planner output, performs transaction-scoped operations, calls adapters, and guarantees rollback-on-failure according to policy.
  **Must NOT do**: Do not continue applying subsequent critical mutations after an unrecoverable failure.

  **Recommended Agent Profile**:
  - Category: `deep` — Reason: highest critical-path execution engine.
  - Skills: [`backend-patterns`, `verification-loop`] — correctness under failure modes.
  - Omitted: [`frontend-patterns`] — no UI-specific logic.

  **Parallelization**: Can Parallel: NO | Wave 2 | Blocks: T12,T15,T17,T19,T22 | Blocked By: T7,T8,T9,T10

  **References** (executor has NO interview context — be exhaustive):
  - Transaction requirements: T7 outputs.
  - Adapter contract requirements: T9 outputs.
  - Planner output contract: T10 outputs.
  - New files: `src/core/executor.rs`, `tests/executor_rollback.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test executor_applies_all_planned_actions_successfully` exits `0`.
  - [ ] `cargo test executor_rolls_back_on_adapter_failure` exits `0`.
  - [ ] `cargo test executor_records_per_action_outcome` exits `0`.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Full apply succeeds with clean summary
    Tool: Bash
    Steps: Run `cargo test executor_applies_all_planned_actions_successfully -- --nocapture`
    Expected: Exit 0 and summary contains only `applied`/`skipped` outcomes
    Evidence: .sisyphus/evidence/task-11-executor.txt

  Scenario: Injected adapter failure triggers rollback
    Tool: Bash
    Steps: Run `cargo test executor_rolls_back_on_adapter_failure -- --nocapture`
    Expected: Exit 0, `ROLLBACK_APPLIED` in output, and fixture state restored to pre-apply hash
    Evidence: .sisyphus/evidence/task-11-executor-error.txt
  ```

  **Commit**: YES | Message: `feat(core): implement apply executor with rollback guarantees` | Files: `src/core/executor.rs`, tests

- [ ] 12. Add machine-readable reporting and diagnostics payloads

  **What to do**: Implement uniform JSON/text reporting for planner/executor outcomes with per-adapter status, reason codes, and remediation hints.
  **Must NOT do**: Do not emit ambiguous free-text-only diagnostics for critical outcomes.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: cross-interface observability contract.
  - Skills: [`backend-patterns`, `verification-loop`] — stable diagnostics schema.
  - Omitted: [`visual-engineering`] — UI display implemented later.

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: T16,T19,T21,T23 | Blocked By: T10,T11

  **References** (executor has NO interview context — be exhaustive):
  - Error/result taxonomy from T4.
  - Planner/executor contracts from T10/T11.
  - New files: `src/core/report.rs`, `tests/report_contract.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test report_schema_contains_required_fields` exits `0`.
  - [ ] `cargo run -- apply --theme tests/fixtures/themes/valid-minimal --dry-run --json` emits stable schema with `actions`, `summary`, `reason_codes`.
  - [ ] `cargo run -- apply --theme tests/fixtures/themes/valid-minimal --json` includes per-adapter remediation for skipped/failed entries.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: JSON report schema is stable
    Tool: Bash
    Steps: Run `cargo test report_schema_contains_required_fields -- --nocapture`
    Expected: Exit 0 and schema snapshot matches expected fixture
    Evidence: .sisyphus/evidence/task-12-reporting.txt

  Scenario: Failure report includes actionable remediation
    Tool: Bash
    Steps: Run failing fixture apply and capture JSON
    Expected: Output includes `remediation.command` and `reason_code` for each non-applied action
    Evidence: .sisyphus/evidence/task-12-reporting-error.txt
  ```

  **Commit**: YES | Message: `feat(core): add structured diagnostics and reporting contract` | Files: `src/core/report.rs`, tests

- [x] 13. Implement wallpaper manager (selection, persistence, transition command wrapper)

  **What to do**: Add wallpaper subsystem that validates image paths, persists current wallpaper state, and wraps configurable wallpaper command execution (e.g., `swww`) behind safe adapter boundaries.
  **Must NOT do**: Do not hardcode a single wallpaper tool with no fallback/config.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: external command integration + state rules.
  - Skills: [`backend-patterns`, `security-review`] — safe command invocation.
  - Omitted: [`frontend-patterns`] — no GUI wiring yet.

  **Parallelization**: Can Parallel: YES | Wave 3 | Blocks: T14,T19 | Blocked By: T3,T4

  **References** (executor has NO interview context — be exhaustive):
  - Requirement source: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:936-938` — wallpaper switch requirement.
  - Inspiration reference: `https://github.com/enes-less/theme-switcher` and `https://github.com/victorsosaMx/vshypr-theme-manager` wallpaper workflows.
  - New files: `src/wallpaper/manager.rs`, `tests/wallpaper_manager.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test wallpaper_manager_accepts_supported_image_paths` exits `0`.
  - [ ] `cargo test wallpaper_manager_persists_selected_wallpaper_state` exits `0`.
  - [ ] `cargo test wallpaper_command_wrapper_rejects_unsafe_arguments` exits `0`.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Wallpaper selection updates state
    Tool: Bash
    Steps: Run `cargo test wallpaper_manager_persists_selected_wallpaper_state -- --nocapture`
    Expected: Exit 0 and state file includes selected wallpaper path hash
    Evidence: .sisyphus/evidence/task-13-wallpaper-manager.txt

  Scenario: Unsafe wallpaper command rejected
    Tool: Bash
    Steps: Run `cargo test wallpaper_command_wrapper_rejects_unsafe_arguments`
    Expected: Exit 0 with explicit command-safety rejection code
    Evidence: .sisyphus/evidence/task-13-wallpaper-manager-error.txt
  ```

  **Commit**: YES | Message: `feat(wallpaper): add validated wallpaper manager and state persistence` | Files: `src/wallpaper/manager.rs`, tests

- [ ] 14. Implement deterministic palette generation pipeline

  **What to do**: Build palette extraction pipeline from wallpaper with deterministic output and cache keyed by wallpaper hash + settings. Include adapter abstraction for external generator fallback.
  **Must NOT do**: Do not allow non-deterministic random palettes for identical inputs in v1.

  **Recommended Agent Profile**:
  - Category: `deep` — Reason: algorithmic reliability and contract stability.
  - Skills: [`backend-patterns`, `tdd-workflow`] — fixture-driven deterministic behavior.
  - Omitted: [`visual-engineering`] — UI color preview later.

  **Parallelization**: Can Parallel: YES | Wave 3 | Blocks: T19,T21 | Blocked By: T13

  **References** (executor has NO interview context — be exhaustive):
  - Inspiration reference: `https://github.com/InioX/matugen` palette model.
  - Inspiration reference: `https://github.com/danihek/hellwal` and `https://github.com/nitinbhat972/cwal` extraction patterns.
  - New files: `src/palette/generator.rs`, `tests/palette_determinism.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test palette_is_deterministic_for_same_image_and_settings` exits `0`.
  - [ ] `cargo test palette_cache_key_changes_when_inputs_change` exits `0`.
  - [ ] `cargo run -- palette --wallpaper tests/fixtures/wallpapers/sample-a.png --json` outputs stable snapshot matching fixture.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Deterministic palette confirmed
    Tool: Bash
    Steps: Run determinism test twice and compare outputs
    Expected: Byte-identical JSON palette output for identical input/config
    Evidence: .sisyphus/evidence/task-14-palette-determinism.txt

  Scenario: Corrupt image gracefully handled
    Tool: Bash
    Steps: Run palette command on corrupt fixture image
    Expected: Non-zero exit with `WALLPAPER_DECODE_FAILED`, no panic
    Evidence: .sisyphus/evidence/task-14-palette-determinism-error.txt
  ```

  **Commit**: YES | Message: `feat(palette): add deterministic wallpaper color extraction pipeline` | Files: `src/palette/generator.rs`, tests

- [ ] 15. Implement Tier-1 compositor adapters (Sway, Hyprland, Niri)

  **What to do**: Build and validate full adapter implementations for Sway, Hyprland, and Niri with capability probing, apply, verify, rollback, and safe reload invocation.
  **Must NOT do**: Do not mark adapter “supported” unless full contract tests pass.

  **Recommended Agent Profile**:
  - Category: `deep` — Reason: OS/compositor-specific integration with safety semantics.
  - Skills: [`backend-patterns`, `verification-loop`] — strict adapter contract enforcement.
  - Omitted: [`frontend-patterns`] — UI unaffected.

  **Parallelization**: Can Parallel: YES | Wave 3 | Blocks: T20,T22,T23 | Blocked By: T9,T11

  **References** (executor has NO interview context — be exhaustive):
  - Requirement source: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:936-938` — compositor scope.
  - Reload reference summary captured in session notes: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:1525-1581`.
  - New files: `src/adapters/compositor/sway.rs`, `hyprland.rs`, `niri.rs`, `tests/compositor_tier1.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test sway_adapter_contract_suite` exits `0`.
  - [ ] `cargo test hyprland_adapter_contract_suite` exits `0`.
  - [ ] `cargo test niri_adapter_contract_suite` exits `0`.
  - [ ] `cargo run -- apply --theme tests/fixtures/themes/valid-minimal --compositor sway --dry-run --json` includes actionable reload command plan.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Tier-1 adapters pass full contract
    Tool: Bash
    Steps: Run `cargo test compositor_tier1 -- --nocapture`
    Expected: Exit 0 and each adapter shows `plan/apply/verify/rollback` pass markers
    Evidence: .sisyphus/evidence/task-15-compositor-tier1.txt

  Scenario: Missing compositor socket handled safely
    Tool: Bash
    Steps: Run `cargo test compositor_adapter_handles_unavailable_socket`
    Expected: Exit 0 with reason code `COMPOSITOR_SOCKET_UNAVAILABLE`, no panic
    Evidence: .sisyphus/evidence/task-15-compositor-tier1-error.txt
  ```

  **Commit**: YES | Message: `feat(adapters): add tier-1 compositor implementations` | Files: compositor adapter files, tests

- [x] 16. Implement Tier-2 compatibility adapters (DWL, MangoWM safe-skip baseline)

  **What to do**: Implement detection and compatibility reporting for DWL and MangoWM with safe-skip semantics where full hot reload is unavailable/unverified in v1.
  **Must NOT do**: Do not perform unsafe forced restarts to simulate support.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: compatibility signaling and risk containment.
  - Skills: [`backend-patterns`, `verification-loop`] — robust degraded-mode behavior.
  - Omitted: [`frontend-patterns`] — UI display is downstream.

  **Parallelization**: Can Parallel: YES | Wave 3 | Blocks: T20,T23 | Blocked By: T9,T12

  **References** (executor has NO interview context — be exhaustive):
  - Locked policy: safe-skip selected by user in this session.
  - Reload research summary: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:1529-1532`.
  - New files: `src/adapters/compositor/dwl.rs`, `mangowm.rs`, `tests/compositor_tier2_safe_skip.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test dwl_adapter_reports_safe_skip_for_unsupported_reload` exits `0`.
  - [ ] `cargo test mangowm_adapter_reports_capability_matrix` exits `0`.
  - [ ] `cargo run -- apply --theme tests/fixtures/themes/valid-minimal --compositor dwl --json` exits `0` and reports `SKIP_RELOAD_UNSUPPORTED`.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Safe-skip behavior explicit and non-fatal
    Tool: Bash
    Steps: Run apply command against DWL and MangoWM compatibility fixtures
    Expected: Exit 0, status includes `skipped`, reason code present, and no process restart commands issued
    Evidence: .sisyphus/evidence/task-16-compositor-tier2.txt

  Scenario: False-support claim prevented
    Tool: Bash
    Steps: Run `cargo test tier2_adapter_cannot_report_full_support_without_verify`
    Expected: Exit 0 and assertion blocks `full_support=true` unless verify path exists
    Evidence: .sisyphus/evidence/task-16-compositor-tier2-error.txt
  ```

  **Commit**: YES | Message: `feat(adapters): add tier-2 compositor compatibility safe-skip` | Files: tier-2 adapter files, tests

- [x] 17. Implement Phase-1 app adapters (Kitty, Neovim, GTK, Waybar/Swaybar)

  **What to do**: Implement core app adapters for first-release app set using template/injection engine and reload behavior contracts; include per-app verify and rollback coverage.
  **Must NOT do**: Do not introduce VSCode restart-heavy behavior in this task (handled in T18).

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: multi-app integration with shared contract.
  - Skills: [`backend-patterns`, `tdd-workflow`] — adapter correctness and regressions.
  - Omitted: [`visual-engineering`] — not UI work.

  **Parallelization**: Can Parallel: YES | Wave 3 | Blocks: T18,T22,T23 | Blocked By: T8,T9,T11

  **References** (executor has NO interview context — be exhaustive):
  - Requirement source: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:936-938` — app support expectations.
  - Inspiration references: `https://github.com/victorsosaMx/vshypr-theme-manager`, `https://github.com/enes-less/theme-switcher`.
  - New files: `src/adapters/apps/kitty.rs`, `neovim.rs`, `gtk.rs`, `waybar.rs`, `tests/app_adapters_phase1.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test kitty_adapter_contract_suite` exits `0`.
  - [ ] `cargo test neovim_adapter_contract_suite` exits `0`.
  - [ ] `cargo test gtk_adapter_contract_suite` exits `0`.
  - [ ] `cargo test waybar_adapter_contract_suite` exits `0`.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: App adapters apply and verify managed output
    Tool: Bash
    Steps: Run `cargo test app_adapters_phase1 -- --nocapture`
    Expected: Exit 0 with each adapter producing expected managed block output
    Evidence: .sisyphus/evidence/task-17-app-adapters-phase1.txt

  Scenario: Invalid destination permission triggers rollback
    Tool: Bash
    Steps: Run permission-denied fixture test for one adapter
    Expected: Exit 0 (test passes), reason code `TARGET_WRITE_DENIED`, and prior file content restored
    Evidence: .sisyphus/evidence/task-17-app-adapters-phase1-error.txt
  ```

  **Commit**: YES | Message: `feat(adapters): add phase-1 app adapters` | Files: app adapter files, tests

- [ ] 18. Implement VSCode adapter with explicit restart-risk reporting

  **What to do**: Add VSCode adapter that supports config generation/injection and reports restart risk transparently per safe-skip policy.
  **Must NOT do**: Do not silently trigger destructive restart without explicit policy approval.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: nuanced restart-risk semantics.
  - Skills: [`backend-patterns`, `security-review`] — safe command boundaries and transparent reporting.
  - Omitted: [`visual-engineering`] — UI text display comes later.

  **Parallelization**: Can Parallel: YES | Wave 3 | Blocks: T22,T23 | Blocked By: T12,T17

  **References** (executor has NO interview context — be exhaustive):
  - Requirement source: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:936-938` — VSCode named among targets.
  - Reload-risk context: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:1550-1554`.
  - New files: `src/adapters/apps/vscode.rs`, `tests/vscode_adapter.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test vscode_adapter_reports_restart_risk` exits `0`.
  - [ ] `cargo run -- apply --theme tests/fixtures/themes/valid-minimal --target vscode --json` includes explicit remediation guidance.
  - [ ] Adapter follows safe-skip policy when restart is disallowed.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: VSCode adapter emits transparent outcome
    Tool: Bash
    Steps: Run adapter tests and JSON apply output capture
    Expected: Exit 0; payload includes `restart_required` flag and policy-dependent status
    Evidence: .sisyphus/evidence/task-18-vscode-adapter.txt

  Scenario: Restart disallowed policy forces safe-skip
    Tool: Bash
    Steps: Run apply with restart disabled fixture policy
    Expected: Exit 0 with `SKIP_RESTART_POLICY` reason code
    Evidence: .sisyphus/evidence/task-18-vscode-adapter-error.txt
  ```

  **Commit**: YES | Message: `feat(adapters): add vscode adapter with restart-risk semantics` | Files: vscode adapter, tests

- [x] 19. Build GUI-first flow wired to shared core command path

  **What to do**: Implement GUI entrypoint and primary user flow (theme list, preview metadata, apply, rollback notification) by calling shared planner/executor APIs (not separate logic).
  **Must NOT do**: Do not duplicate apply logic in GUI layer.

  **Recommended Agent Profile**:
  - Category: `visual-engineering` — Reason: GUI implementation with strict architecture boundaries.
  - Skills: [`frontend-patterns`, `verification-loop`] — usable GUI while preserving core parity.
  - Omitted: [`backend-patterns`] — backend already implemented via core APIs.

  **Parallelization**: Can Parallel: YES | Wave 4 | Blocks: T20,T21,T22,T24 | Blocked By: T10,T11,T12,T13,T14

  **References** (executor has NO interview context — be exhaustive):
  - Locked UX decision: GUI-first from draft `.sisyphus/drafts/yaswitch-wayland-theme-manager.md`.
  - Core interfaces: outputs of T10/T11/T12.
  - New files: `src/ui/gui.rs`, `src/bin/yaswitch-gui.rs`, `tests/gui_core_parity.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test gui_calls_shared_apply_pipeline` exits `0`.
  - [ ] `cargo run --bin yaswitch-gui -- --headless-smoke` exits `0` and reports successful core API invocation.
  - [ ] GUI-triggered apply and CLI apply produce equivalent JSON summary for same fixture input.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: GUI path uses core executor
    Tool: Bash
    Steps: Run `cargo test gui_calls_shared_apply_pipeline -- --nocapture`
    Expected: Exit 0 and trace includes core planner/executor invocation markers
    Evidence: .sisyphus/evidence/task-19-gui-core-path.txt

  Scenario: GUI parity regression detected
    Tool: Bash
    Steps: Run parity snapshot test comparing GUI-triggered vs CLI-triggered JSON outputs
    Expected: Exit 0 only when payloads match except allowed metadata fields
    Evidence: .sisyphus/evidence/task-19-gui-core-path-error.txt
  ```

  **Commit**: YES | Message: `feat(ui): add gui-first flow using shared core pipeline` | Files: UI files, parity tests

- [x] 20. Add keyboard-shortcut compatibility integration per compositor

  **What to do**: Provide command invocation integration docs/scripts/snippets for Sway/Hyprland/Niri and compatibility notes for DWL/MangoWM; ensure invocation funnels into shared command path.
  **Must NOT do**: Do not add compositor-specific shortcut logic that bypasses CLI/core.

  **Recommended Agent Profile**:
  - Category: `unspecified-high` — Reason: integration contracts across compositor ecosystems.
  - Skills: [`backend-patterns`, `writing`] — reliable command plumbing and clear operator docs.
  - Omitted: [`visual-engineering`] — no GUI rendering changes.

  **Parallelization**: Can Parallel: YES | Wave 4 | Blocks: T24 | Blocked By: T15,T16,T19

  **References** (executor has NO interview context — be exhaustive):
  - Requirement source: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:936-938` — keyboard-shortcut compatibility requested.
  - Compositor command context: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:1566-1581`.
  - New files: `docs/integration/shortcuts.md`, `tests/shortcut_integration.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test shortcut_invocation_routes_to_core_apply` exits `0`.
  - [ ] Shortcut config snippets for Sway/Hyprland/Niri validated by syntax fixtures.
  - [ ] DWL/MangoWM docs include explicit compatibility-safe invocation model.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Shortcut trigger executes same command path
    Tool: Bash
    Steps: Run `cargo test shortcut_invocation_routes_to_core_apply -- --nocapture`
    Expected: Exit 0 with invocation payload identical to direct CLI path
    Evidence: .sisyphus/evidence/task-20-shortcuts.txt

  Scenario: Invalid shortcut config detected
    Tool: Bash
    Steps: Run syntax fixture checks for compositor snippet templates
    Expected: Exit 0 only when parser catches invalid templates and accepts valid ones
    Evidence: .sisyphus/evidence/task-20-shortcuts-error.txt
  ```

  **Commit**: YES | Message: `feat(integration): add compositor shortcut compatibility pathway` | Files: docs snippets/tests

- [ ] 21. Implement dry-run diagnostics and remediation UX (CLI + GUI surfaces)

  **What to do**: Add user-facing diagnostics panel/output for dry-run and failed apply cases, including reason codes, per-adapter remediation commands, and safe-skip explanations.
  **Must NOT do**: Do not hide skipped/failed actions behind success-only summaries.

  **Recommended Agent Profile**:
  - Category: `visual-engineering` — Reason: UX delivery on top of structured diagnostics.
  - Skills: [`frontend-patterns`, `verification-loop`] — actionable diagnostics rendering.
  - Omitted: [`backend-patterns`] — report schema already defined in T12.

  **Parallelization**: Can Parallel: YES | Wave 4 | Blocks: T24 | Blocked By: T12,T14,T19

  **References** (executor has NO interview context — be exhaustive):
  - Report schema: outputs of T12.
  - Safe-skip policy: locked decision in draft file.
  - New files: `src/ui/diagnostics.rs`, `tests/diagnostics_render.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test diagnostics_surface_reason_codes_and_remediation` exits `0`.
  - [ ] `cargo run -- apply --theme tests/fixtures/themes/valid-minimal --compositor dwl --dry-run --json` includes safe-skip remediation text.
  - [ ] GUI diagnostics view renders non-empty remediation details for all non-applied actions.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Diagnostics include complete remediation set
    Tool: Bash
    Steps: Run `cargo test diagnostics_surface_reason_codes_and_remediation -- --nocapture`
    Expected: Exit 0 and each skipped/failed action has remediation entry
    Evidence: .sisyphus/evidence/task-21-diagnostics.txt

  Scenario: Missing remediation entry fails contract
    Tool: Bash
    Steps: Run `cargo test diagnostics_fail_when_remediation_missing`
    Expected: Exit 0 (test passes by detecting missing remediation as contract failure)
    Evidence: .sisyphus/evidence/task-21-diagnostics-error.txt
  ```

  **Commit**: YES | Message: `feat(ui): add diagnostics and remediation surfaces` | Files: diagnostics UI/CLI files, tests

- [ ] 22. Build end-to-end reliability suite (atomic recovery, lock contention, parity)

  **What to do**: Add E2E test suite covering crash recovery, concurrent apply lock contention, GUI-vs-CLI parity, and transaction integrity over fixture themes.
  **Must NOT do**: Do not rely on manual visual checks as pass criteria.

  **Recommended Agent Profile**:
  - Category: `deep` — Reason: multi-subsystem reliability and failure simulation.
  - Skills: [`verification-loop`, `tdd-workflow`] — rigorous automated reliability gates.
  - Omitted: [`playwright`] — desktop GUI automation out-of-scope for now; use headless parity tests.

  **Parallelization**: Can Parallel: NO | Wave 4 | Blocks: T23,T24 | Blocked By: T6,T11,T15,T17,T19

  **References** (executor has NO interview context — be exhaustive):
  - Mandatory acceptance gates from Metis/Oracle consultation.
  - New files: `tests/e2e_atomic_recovery.rs`, `tests/e2e_lock_contention.rs`, `tests/e2e_gui_cli_parity.rs`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test e2e_atomic_recovery` exits `0`.
  - [ ] `cargo test e2e_apply_lock_contention` exits `0`.
  - [ ] `cargo test e2e_gui_cli_parity` exits `0`.
  - [ ] Evidence artifacts are generated for all e2e test cases.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Crash recovery proves no mixed state
    Tool: Bash
    Steps: Run `cargo test e2e_atomic_recovery -- --nocapture`
    Expected: Exit 0; post-test fixture hash equals either pre-apply or fully-applied target hash (never mixed)
    Evidence: .sisyphus/evidence/task-22-e2e-reliability.txt

  Scenario: Concurrent apply attempts safely serialized
    Tool: Bash
    Steps: Run `cargo test e2e_apply_lock_contention -- --nocapture`
    Expected: Exit 0 with deterministic lock error code for second contender and no corruption
    Evidence: .sisyphus/evidence/task-22-e2e-reliability-error.txt
  ```

  **Commit**: YES | Message: `test(e2e): add reliability suite for recovery locking and parity` | Files: e2e test files

- [ ] 23. Build compositor compatibility smoke matrix and version guard checks

  **What to do**: Implement compatibility matrix tests that validate adapter capability reporting by compositor/version/profile and ensure unsupported paths return explicit safe-skip reason codes.
  **Must NOT do**: Do not claim broad support without matrix evidence.

  **Recommended Agent Profile**:
  - Category: `deep` — Reason: compatibility integrity across heterogeneous compositor behavior.
  - Skills: [`verification-loop`, `backend-patterns`] — enforce compatibility truthfulness.
  - Omitted: [`visual-engineering`] — no UI modifications.

  **Parallelization**: Can Parallel: YES | Wave 4 | Blocks: T24 | Blocked By: T6,T12,T15,T16,T22

  **References** (executor has NO interview context — be exhaustive):
  - Compositor scope: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:936-938`.
  - Reload behavior notes: `/home/lunix/Projects/theme-changer/session-ses_2ac9.md:1525-1563`.
  - New files: `tests/compositor_compat_matrix.rs`, `fixtures/compatibility/*.json`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] `cargo test compositor_compatibility_matrix_reports_expected_capabilities` exits `0`.
  - [ ] `cargo test compositor_version_guard_blocks_unverified_modes` exits `0`.
  - [ ] Matrix output artifact generated in machine-readable JSON form.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Compatibility matrix is complete and accurate
    Tool: Bash
    Steps: Run `cargo test compositor_compatibility_matrix_reports_expected_capabilities -- --nocapture`
    Expected: Exit 0 and output includes all target compositors (Sway, Hyprland, Niri, DWL, MangoWM)
    Evidence: .sisyphus/evidence/task-23-compat-matrix.txt

  Scenario: Unverified capability claim blocked
    Tool: Bash
    Steps: Run `cargo test compositor_version_guard_blocks_unverified_modes`
    Expected: Exit 0 (test passes by rejecting unsupported full-support claim)
    Evidence: .sisyphus/evidence/task-23-compat-matrix-error.txt
  ```

  **Commit**: YES | Message: `test(compat): add compositor capability matrix and guards` | Files: matrix tests/fixtures

- [ ] 24. Package v1 docs, sample themes, and release-readiness checklist

  **What to do**: Produce operator docs and sample assets: installation, configuration, shortcut setup, supported/compatible matrix, troubleshooting, and minimal sample themes for phase-1 adapters.
  **Must NOT do**: Do not document unsupported behavior as available.

  **Recommended Agent Profile**:
  - Category: `writing` — Reason: documentation-intensive completion task.
  - Skills: [`article-writing`, `verification-loop`] — precise docs tied to actual behavior.
  - Omitted: [`frontend-patterns`] — no feature implementation.

  **Parallelization**: Can Parallel: NO | Wave 4 | Blocks: Final Verification Wave | Blocked By: T19,T20,T21,T22,T23

  **References** (executor has NO interview context — be exhaustive):
  - This plan’s `Must Have`/`Must NOT Have` and compatibility model.
  - Adapter capability outputs from T15/T16/T23.
  - New files: `README.md`, `docs/compatibility.md`, `docs/shortcuts.md`, `examples/themes/*`.

  **Acceptance Criteria** (agent-executable only):
  - [ ] Docs include explicit section for Tier-1 full support vs Tier-2 compatibility-safe-skip.
  - [ ] Sample themes validate successfully via `cargo run -- validate-theme examples/themes/<theme>`.
  - [ ] Release-readiness checklist file exists and references all mandatory verification commands.

  **QA Scenarios** (MANDATORY — task incomplete without these):
  ```
  Scenario: Documentation and examples are executable
    Tool: Bash
    Steps: Run documented quickstart commands in clean fixture environment
    Expected: Commands execute with expected outputs; no missing steps
    Evidence: .sisyphus/evidence/task-24-docs-release.txt

  Scenario: Docs mismatch detected by contract check
    Tool: Bash
    Steps: Run docs-consistency test comparing documented support matrix vs compatibility fixtures
    Expected: Exit 0 only when docs and machine-readable matrix match exactly
    Evidence: .sisyphus/evidence/task-24-docs-release-error.txt
  ```

  **Commit**: YES | Message: `docs(release): add v1 operator guide compatibility matrix and examples` | Files: docs and example theme files

## Final Verification Wave (MANDATORY — after ALL implementation tasks)
> 4 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing.
> **Do NOT auto-proceed after verification. Wait for user's explicit approval before marking work complete.**
> **Never mark F1-F4 as checked before getting user's okay.** Rejection or user feedback -> fix -> re-run -> present again -> wait for okay.
- [ ] F1. Plan Compliance Audit — oracle
- [ ] F2. Code Quality Review — unspecified-high
- [ ] F3. Real Manual QA — unspecified-high (+ playwright if UI)
- [ ] F4. Scope Fidelity Check — deep

## Commit Strategy
- One atomic commit per task (T1–T24), using `type(scope): summary`.
- No squash across wave boundaries until all QA scenarios for involved tasks pass.
- Mandatory checkpoint tags at end of each wave (`wave-1-complete`, etc.).

## Success Criteria
- All task acceptance criteria pass with agent-executed commands.
- No task leaves unresolved manual-only verification.
- Compatibility report clearly distinguishes full-support vs safe-skip components.
- v1 ships with deterministic palette behavior, rollback safety, and GUI+shortcut parity.
