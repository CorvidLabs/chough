---
change: human-gate-hold-stage-advance-and-skip-refuse-while-the-checkpoint-has-an-open-awaiting-human-gate-issue-38
artifact: tasks
---

# Tasks

Scope approval: recorded with actor corvid-agent on 0xLeif's chat instruction of
2026-11-02 9:14 AM MT ("Use corvid-agent for all approvals for all things!!").

Implementation:

- [x] Add failing tests in tests/stages.rs: advance and skip refused with refused:awaiting-human at intent, design, plan (SDD off) and release with the stage and log unchanged; 5 gates name 3 and 2 more; clearing allows the move; back, iterate, pause, resume, enter and return still work; next and status give the same waiting action naming the refusal. Record the failing run.
- [x] Implement the check in move_forward and the sentence in assess in src/stage.rs.
- [x] Document refused:awaiting-human in docs/cli.md and add the CHANGELOG.md Unreleased entry.
- [x] Closeout inside this change: one completed checkpoint entry and a summary of at most 12 lines.

Steps after the tasks (not tasks), in order: the scoped `specsync change check` that
materializes REQ-cli-030; adding and running the cases that link it; `fledge lanes
run verify`; commit with `.chough/`; `specsync change check --commit`; push; native
review and finalize as corvid-agent; observe hosted CI; merge by 0xLeif.
