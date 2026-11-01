---
change: report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records
artifact: tasks
---

# Tasks

- [ ] Add the bounded full transition reader in src/records.rs with unit tests.
- [ ] Append gate-opened and gate-closed transitions in src/checkpoint.rs.
- [ ] Add src/timing.rs: stage segments, cycles, pause, side stages and waiting overlap.
- [ ] Add tests, validation and bug sections with unknown handling.
- [ ] Add the native section per the approved native-source choice.
- [ ] Add history ranges, labels, text and JSON output; wire `chough timing` in src/main.rs.
- [ ] Add tests/timing.rs fixtures with fixed timestamps, bounds and byte-identical checks.
- [ ] Add REQ-cli-020 case definitions after the scoped check materializes the requirement.
- [ ] Update docs/cli.md, docs/architecture.md, README.md and the cli.spec.md files list.
- [ ] Update skills/chough-workflow/references/checkpoint.md and plan.md, record known-previous digests in src/skill.rs, and reinstall host copies.

Human gates and post-verification steps (not tasks): scope approval of this change,
including the three open choices in plan.md; push and CI on Ubuntu, macOS and
Windows; the human implementation review; native review and ship.
