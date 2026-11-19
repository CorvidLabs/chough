# Lesson bundle: fix-283

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 283
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/cases.rs, tests/cases.rs, docs/cli.md, .chough/cases/cli.retired-case-binding.cases.toml, changelog.d/2026-11-18-fix-283.md
- **Acceptance**: chough cases check passes a retired case whose fledge-task binding names a task no longer in fledge.toml, with the binding kept as written, while an active case naming a missing task is still refused naming the task; cases run still refuses the retired case.

## Evidence

- Verification commit: `481be0271e1340e5d261b4a8b45a15518ab8d1c8`
- Base commit: `e47b4cfaabe2e68fd02246be01fb7e8e256fe00b`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

GitHub #283, seen in CorvidLabs/jackdaw#155. `chough cases check` refuses a retired case whose `fledge-task` binding names a task since removed from `fledge.toml`. In `check_links` the task-existence check runs before the retired-case early return. Removing the task is often why the case was retired, so a repo had to keep a dead task or rewrite the retired case's historical binding.

Leif's decision (2026-11-18, planning interview, recorded on #283 and relayed by hc t4c as the scope approval) is to skip the check for retired cases. `cases check` returns early for a retired case before checking its `fledge-task` binding, so the original binding stays as history.

A retired case already never runs: `find_checked` refuses it. No requirement states the old order, so this is a no-spec change.

## From the change's design.md

# Design

In `cases::check_links`, the `status == "retired"` early return moves above the `fledge-task` existence check. A retired case's structural checks, run elsewhere when the file loads, are unchanged. Its links and its task are no longer checked. An active case naming a missing task is refused as before.

## From the change's testing.md

# Testing

`tests/cases.rs::a_retired_case_keeps_a_binding_to_a_removed_fledge_task`, red on main:
- A retired case bound to a task missing from `fledge.toml` passes `cases check`, with its file unchanged.
- The same binding on an active case is refused, naming the task.
- `cases run` on the retired case still refuses it.

Case `TC-cli-a-retired-case-keeps-its-fledge-task-binding` binds it.

## Where these lessons go

- `specs/cli/cli.3md`
