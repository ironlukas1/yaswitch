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

## Pending clarification queue (reworded for clarity)

These came from answers like “explain”/“don’t know” and are restated in simpler terms:

1. **Value proposition wording**: What single sentence should users remember about yaswitch?  
   Example pattern: “yaswitch makes theme switching fast, safe, and predictable on Wayland.”

2. **Remote/headless support**: Should yaswitch be usable entirely without a GUI session (e.g., SSH, scripts, CI)?

3. **Config paths support**: Should we support custom config/state directories beyond default XDG paths (for power users/dotfiles setups)?

4. **Compatibility tiers policy**: Should each integration clearly declare one of: Tier-1 full support, Tier-2 safe compatibility, Tier-3 experimental?

5. **Planner vs adapter validation split**: Should global checks stay in planner while format/app-specific checks live in each adapter?

6. **Optional adapters per theme**: Should a theme be allowed to omit app targets and still be valid (apply only listed targets)?

7. **Theme packs**: Should users be able to export/import a full theme bundle for sharing?

8. **Test gate priorities**: Which CI checks must always block merges (format, clippy, tests, coverage, snapshots)?

9. **CI runtime budget**: What is acceptable max PR pipeline duration for your 5-hour/week maintenance budget?

10. **SemVer preference**: Do you want strict semantic versioning labels from now, even pre-1.0?

11. **Risk tolerance for release cadence**: How conservative should release gating be for alpha/beta builds?

12. **Good first issue policy**: Should we tag beginner-friendly tasks once OSS onboarding starts?

13. **Replayable dry-run traces**: Should users be able to re-run a captured dry-run trace for debugging reproducibility?

14. **Machine-readable error taxonomy docs**: Should reason codes have a dedicated reference file for users and tools?

15. **Environment fingerprinting**: Should debug mode include anonymized environment details (compositor, distro family, versions)?

16. **Scope guardrails**: How strict should we be in saying “not now” to features that hurt speed/safety goals?

## Update protocol

When work starts on a task:
- move task status to `IN_PROGRESS`
- add date + owner note

When task is completed:
- move to `DONE`
- link PR/commit
- record verification evidence (tests/diagnostics)
