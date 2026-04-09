# yaswitch Progress Tracker

Last updated: 2026-04-08

## Status legend

- `DONE` = completed and verified
- `IN_PROGRESS` = actively being worked
- `PLANNED` = approved and queued
- `BLOCKED` = waiting on decision or prerequisite

## Current wave snapshot

Current wave: **Wave 1 — Core reliability and fast apply/cycle**
Current tier focus: **Tier 1**
Primary risk: apply-path breakage under partial adapter failure.

## Wave 1 progress

### Tier 1

| ID | Task | Status | Notes |
|---|---|---|---|
| W1-T1-01 | Apply transaction hardening | PLANNED | Must guarantee no config-loss regression; failure injection tests needed |
| W1-T1-02 | Cycle performance baseline | PLANNED | Speed is top product priority |
| W1-T1-03 | Preflight policy implementation | PLANNED | Strict for destructive ops, warn for recoverable ops |
| W1-T1-04 | Shortcut-first cycle UX | PLANNED | Keep installer/cycle on same core apply path |

### Tier 2

| ID | Task | Status | Notes |
|---|---|---|---|
| W1-T2-01 | Human-first CLI output polish | PLANNED | Pretty JSON + single verbose flag |
| W1-T2-02 | App adapter reliability pass | PLANNED | kitty/waybar/rofi/neovim/vscode/gtk first |
| W1-T2-03 | Debug evidence pack | PLANNED | active in `--debug`, includes redaction |

### Tier 3

| ID | Task | Status | Notes |
|---|---|---|---|
| W1-T3-01 | Docs split refinement | PLANNED | user vs contributor docs |
| W1-T3-02 | Troubleshooting templates | PLANNED | common failure playbooks |

## Major decisions captured

1. Wayland-only scope for now.
2. Sway is mandatory for v1.0 baseline.
3. Cycle command and shortcut flow are core, not optional.
4. Human-readable default output remains primary UX mode.
5. JSON output stays for debugging and automation.
6. GUI track is intentionally deferred beyond MVP.
7. Validation split is fixed: planner=global checks, adapters=target checks.
8. Partial themes are valid and should apply only listed targets.
9. Debug mode should include anonymized environment fingerprinting.
10. Safety gates are mandatory before speed work for new features.

## Update protocol

When work starts on a task:
- move task status to `IN_PROGRESS`
- add date + owner note

When task is completed:
- move to `DONE`
- link PR/commit
- record verification evidence (tests/diagnostics)
