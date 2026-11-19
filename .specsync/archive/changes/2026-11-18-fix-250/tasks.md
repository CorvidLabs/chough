---
change: fix-250
artifact: tasks
---

# Tasks

- [x] Extend the doctor test so it fails on main: the withdrawn gate's entry carries the reason and doctor's JSON echoes it.
- [x] A Timeline entry has an optional `withdrawn` field, written and read by `session.md`.
- [x] The stale gates fix passes its reason to the gate withdrawal, and an applied decision fix reports `decision_reason`.
- [x] Say so in `docs/cli.md`; add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
