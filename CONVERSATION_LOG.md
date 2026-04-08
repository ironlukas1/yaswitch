# yaswitch Conversation Log

## 2026-04-08 — Steering Session

### Session context
- [DECISION] User requested project status report and extensive steering questions.
- [DECISION] User then provided detailed answers for Sets 1–15 and requested roadmap/progress docs in wave-tier-task structure.
- [DECISION] User requested logging of all project conversations.

### Key user decisions captured
- [DECISION] Primary persona now: power user (personal-first), beginner-friendly path later.
- [DECISION] Direction: eventual GUI-first (Qt6-style), but CLI is current execution path.
- [DECISION] Priority order: speed first, safety/stability second.
- [DECISION] Scope now: Wayland-only.
- [DECISION] Core command focus: apply and cycle (shortcuts included).
- [DECISION] v1.0 compositor baseline: Sway mandatory; broader compositor support phased.
- [DECISION] JSON output: yes, for debugging and automation.
- [DECISION] OSS intent: later, but likely if tool quality is high.

### Requested artifacts
- [DECISION] ROADMAP.md with waves, tiers, tasks.
- [DECISION] PROGRESS.md with actionable tracking.
- [DECISION] IDEAS.md for deferred items and idea-tagged answers.
- [DECISION] Continuous conversation logging in this file.

### Special answer handling policy (from user)
- [DECISION] If answer is `explain` or `don't know`: ask same question again in clearer wording.
- [DECISION] If answer is `you choose`: choose best option based on existing answers and engineering judgment.
- [DECISION] If answer is `idea`: append to IDEAS.md.

### Operational note
- This log is append-only by default.

## 2026-04-08 — Roadmap Execution Update

### Session context
- [DECISION] User requested conversion of answers into actionable roadmap/progress artifacts.
- [DECISION] User requested wave -> tier -> task structure with precise execution planning.
- [DECISION] User requested commit + push workflow.

### Work completed
- [DECISION] Created `ROADMAP.md` with wave/tier/task structure and acceptance criteria.
- [DECISION] Created `PROGRESS.md` with status tracker and reworded clarification queue.
- [IDEA] Created/normalized `IDEAS.md` entries with date/source/value/target wave fields.
- [DECISION] Updated this log with structured tags and append-only updates.

### Verification notes
- [OPEN] `cargo build --workspace --all-features` passed.
- [OPEN] `cargo clippy --workspace --all-targets --all-features -- -D warnings` passed.
- [OPEN] `cargo test --workspace --all-features` currently fails in pre-existing `tests/ci_workflow.rs` (CI workflow file lookup at runtime).

### Git notes
- [DECISION] Committed planning docs as commit `3ad6130`.
- [OPEN] Push failed due missing GitHub credentials in this environment (terminal prompts disabled).
