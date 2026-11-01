---
change: report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records
artifact: tasks
---

# Tasks

- [ ] Add the bounded full transition reader in src/records.rs with unit tests.
- [ ] Append gate-opened and gate-closed transitions in src/checkpoint.rs.
- [ ] Add src/timing.rs: stage segments (gate kinds excluded), cycles from resets, unknown first-segment start, idle `done` time, pause, side stages, and waiting overlap reconciled with the current awaiting-human list.
- [ ] Add tests, validation and bug sections with unknown handling (validation elapsed labelled total run wall-clock; pruned observations from the retention ledger; unavailable ones make reproduced unknown).
- [ ] Add the native section per the approved native-source choice: bounded status reads (current change plus at most 10 archived), gate and timestamp only, validity from status.
- [ ] Add history ranges, labels, text and JSON output; wire `chough timing` in src/main.rs.
- [ ] Add tests/timing.rs fixtures with fixed timestamps, bounds and byte-identical checks.
- [ ] Update docs/cli.md, docs/architecture.md, README.md and the cli.spec.md files list.
- [ ] Update skills/chough-workflow/references/checkpoint.md and plan.md, record known-previous digests in src/skill.rs, and reinstall host copies.

Human gates (not tasks): scope approval of this change, including the three open
choices in plan.md; the human implementation review; native review and ship.

Post-verification steps (not tasks): after the scoped `change check` materializes
REQ-cli-020, add its case definitions and record their runs; push and CI on Ubuntu,
macOS and Windows; then re-run `change check`.
