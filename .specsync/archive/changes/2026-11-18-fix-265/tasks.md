---
change: fix-265
artifact: tasks
---

# Tasks

- [x] Add tests that fail on main: a write with conflict markers goes through, doctor reports nothing, and the migration writes them.
- [x] Refuse a record body holding a conflict marker in the record write path and before the migration writes anything; report committed records holding one in doctor.
- [x] Document the `conflict_markers` finding in `docs/cli.md`; add the case definitions and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
