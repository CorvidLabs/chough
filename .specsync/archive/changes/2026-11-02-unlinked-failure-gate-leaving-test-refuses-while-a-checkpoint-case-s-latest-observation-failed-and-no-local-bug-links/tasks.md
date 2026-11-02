---
change: unlinked-failure-gate-leaving-test-refuses-while-a-checkpoint-case-s-latest-observation-failed-and-no-local-bug-links
artifact: tasks
---

# Tasks

Scope approval: recorded with actor corvid-agent on 0xLeif's chat instruction of
2026-11-02 9:14 AM MT ("Use corvid-agent for all approvals for all things!!").

Implementation:

- [x] Add failing tests in tests/stages.rs: advance and skip from test refused with refused:unlinked-failure for a referenced case's failed, unlinked latest observation; allowed after a later pass and after a bug links it; not refused for an unreferenced case or a passed latest observation; blocking bugs still refused first; next at test names the refusal. Record the failing run.
- [x] Implement the rule, the prerequisite and the assess branch in src/stage.rs.
- [x] Document refused:unlinked-failure in docs/cli.md and add the CHANGELOG.md Unreleased entry.
- [x] Closeout inside this change: no new lesson (the failure is recorded as a bug, and a lesson here would conflict textually with PR #41's), one completed checkpoint entry, a summary of at most 12 lines.

Steps after the tasks (not tasks), in order: the scoped `specsync change check` that
materializes the delta; adding the case for the gate and rerunning the affected
stage cases; `fledge lanes run verify`; commit with `.chough/`; `specsync change
check --commit`; push and observe hosted CI; 0xLeif's implementation review; native
review and finalize; merge by 0xLeif.
