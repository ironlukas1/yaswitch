# yaswitch Execution Roadmap

Last updated: 2026-04-08
Owner: @ironlukas1

## Planning model

- **Wave** = major outcome window (what must be true at the end)
- **Tier** = priority lane inside a wave
  - **Tier 1** = blocking / must ship
  - **Tier 2** = high value / should ship
  - **Tier 3** = optional / stretch
- **Task** = concrete, testable unit of work

## Fixed product decisions (from Q&A)

1. Primary persona: power user (you), with beginner-friendly path later.
2. Platform: Wayland-only for now.
3. Priority order: **Speed → Safety/Stability → Compatibility/Flexibility**.
4. v1.0 compositor minimum: **Sway** (others can land earlier if reliable).
5. Theme cycle is crucial, including shortcut flow.
6. JSON output should exist for debugging.
7. GUI-first future is desired (Qt6 / skwd-shell-style direction), not MVP.

## Wave 1 — Core reliability and fast apply/cycle (current focus)

### Tier 1 (must ship)
- [ ] **W1-T1-01: Apply transaction hardening**
  - Define exact rollback boundary (what is always restorable, what is best-effort).
  - Enforce atomic journal semantics across all app adapters in apply path.
  - Add failure-injection tests for partial adapter failures.
  - **Acceptance**: no config-loss regression in tests; deterministic rollback result codes.

- [ ] **W1-T1-02: Cycle performance baseline**
  - Benchmark cycle on Sway + baseline app set (kitty/waybar/neovim/vscode/gtk).
  - Set concrete budget (target under 350ms dry-run, under 900ms apply on baseline machine).
  - Profile and optimize hottest code paths (planner/template/adapter dispatch).
  - **Acceptance**: benchmark report committed; thresholds enforced in CI/nightly.

- [ ] **W1-T1-03: Preflight policy implementation (chosen)**
  - Use strict preflight for destructive or non-recoverable operations.
  - Use warning-level preflight for recoverable adapter hiccups.
  - Add interactive confirmation only when operation risk is elevated.
  - **Acceptance**: clear reason-code mapping and prompt behavior tested.

- [ ] **W1-T1-04: Shortcut-first cycle UX**
  - Ensure shortcut installer and cycle command stay on same apply core path.
  - Validate Sway include generation + idempotent installer behavior.
  - **Acceptance**: shortcut integration tests pass; docs examples match output.

### Tier 2 (should ship)
- [ ] **W1-T2-01: Human-first CLI output polish**
  - Human-readable default output; pretty JSON with stable keys in `--json` mode.
  - Add single `-v` verbosity flag for deeper diagnostics.

- [ ] **W1-T2-02: App adapter reliability pass (phase 1 list)**
  - Stabilize kitty, waybar, rofi, neovim, vscode (+ insiders), gtk.
  - Mark each adapter with support level and verification status.

- [ ] **W1-T2-03: Debug evidence pack (chosen)**
  - `--debug` mode writes structured bundle: command args, env fingerprint, plan, apply report, adapter outcomes, rollback trace.
  - Redact private paths/tokens.

### Tier 3 (stretch)
- [ ] **W1-T3-01: Quick docs split refinement** (user docs vs contributor docs)
- [ ] **W1-T3-02: Troubleshooting templates for common failures**

---

## Wave 2 — Adapter scale-out and compatibility expansion

### Tier 1 (must ship)
- [ ] **W2-T1-01: Compositor expansion by ranked priority**
  - Hyprland → Niri → KWin → COSMIC → DWL → MangoWM.
  - Keep safe-skip where reload unsupported.
  - **Acceptance**: compatibility matrix + tests per compositor.

- [ ] **W2-T1-02: Adapter conformance suite**
  - Introduce adapter developer-only verification harness.
  - CI route for adapter authors; users are not required to run full conformance locally.
  - **Acceptance**: each adapter exposes pass/fail conformance report; CI blocks merges when required adapter checks fail.

- [ ] **W2-T1-03: Auto-detection robustness pass**
  - Improve compositor/app detection for speed and correctness.
  - Hard-fail only for critical unsupported states; soft-skip recoverable app-level reload failures.
  - **Acceptance**: detection matrix tests added for supported compositors/apps with explicit hard-fail vs soft-skip assertions.

### Tier 2 (should ship)
- [ ] **W2-T2-01: Distribution packaging start** (AUR first, GitHub release artifacts next)
- [ ] **W2-T2-02: Versioning workflow** (alpha/beta milestones + release notes automation)
- [ ] **W2-T2-03: Docs consistency gates expansion**

### Tier 3 (stretch)
- [ ] **W2-T3-01: Initial plugin architecture skeleton for niche adapters**

---

## Wave 3 — GUI track + ecosystem readiness (post-v1 foundation)

### Tier 1 (must ship)
- [ ] **W3-T1-01: GUI architecture RFC**
  - Qt6-based shell direction, preserving CLI as source of truth.
  - **Acceptance**: approved RFC document with architecture diagram, IPC boundary, and parity requirements for `apply`/`cycle`.

- [ ] **W3-T1-02: Plugin model execution**
  - Built-ins for main adapters, plugin lane for niche targets.
  - **Acceptance**: plugin interface spec finalized and at least one proof-of-concept niche adapter loads via plugin path.

### Tier 2 (should ship)
- [ ] **W3-T2-01: Broader audience onboarding flow**
- [ ] **W3-T2-02: Optional telemetry for crash-only diagnostics (offline-first default)**

### Tier 3 (stretch)
- [ ] **W3-T3-01: Cross-machine sync concept validation**

---

## Chosen defaults for unresolved “you choose” answers

1. **Transactional scope**: planner + adapter write operations are transactional; process-external runtime app state is best-effort.
2. **Sandbox strictness**: allow only resolved config/state/theme roots; reject path traversal and unknown write roots.
3. **Schema strictness**: unknown top-level keys warn in alpha, fail in beta+.
4. **Cache policy**: content-hash cache for palette/wallpaper operations with explicit invalidation command.
5. **Critical failure policy**: hard-fail on compositor-level critical unsupported state, soft-skip on isolated app reload failures.
