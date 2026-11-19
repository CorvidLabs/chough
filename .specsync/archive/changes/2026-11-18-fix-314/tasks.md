---
change: fix-314
artifact: tasks
---

# Tasks

- [x] Add tests that fail on main: metadata files stop `next`, `status` and `cases check`, `doctor` reports a committed one as unreadable, a binary one's leak is missed, and `migrate` refuses one beside 0.5.x records.
- [x] Add `evidence::is_os_metadata` and skip such names in the evidence readers, the case loader, doctor's bound check and migrate's 0.5.x readers, knowledge notes included, and remove only truly empty folders.
- [x] Read tracked files lossily in doctor's leak scan, and leave a file that is not valid UTF-8 unrewritten under `--fix --redact`, reporting it `skipped`.
- [x] Add the case definitions and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
