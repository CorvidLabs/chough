---
change: unlinked-failure-ignores-not-run-leaving-test-keeps-refusing-when-a-referenced-case-s-latest-executed-observation
artifact: tasks
---

# Tasks

Scope approval: recorded with actor corvid-agent on 0xLeif's chat instruction of 2026-11-02 9:14 AM MT ("Use corvid-agent for all approvals for all things!!").

- [x] Add the failing regression test in tests/stages.rs (failed@100 then not-run@200 refuses advance, skip, next and status; a later pass lifts it even with a later not-run; not-run-only does not refuse) and record the failing run.
- [x] Filter unlinked_failure to executed observations in src/stage.rs.
- [x] Reword the rule in docs/cli.md and CHANGELOG.md.
- [x] Closeout inside this change: one completed checkpoint entry.

Steps after the tasks (not tasks): the scoped `specsync change check` that materializes the REQ-cli-008 rewording; running the linked case; `fledge lanes run verify`; commit; `specsync change check --commit`; push; native review and finalize as corvid-agent; observe hosted CI; merge by 0xLeif.
