---
change: fix-325
artifact: tasks
---

# Tasks

- [x] Add a test that fails on main: a busy `evidence add` writes its entry before it refuses.
- [x] Take the writer lock at the start of the evidence save, before any file is written.
- [x] Add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
