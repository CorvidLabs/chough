---
change: fix-329-check-first
artifact: tasks
---

# Tasks

- [x] Add a test that fails first: `bug add` with the active session full writes an unlinked bug.
- [x] Check, before `create`, that the active session can take the new bug's link, and refuse with nothing written when it cannot.

Human gates (not checkboxes): scope approval, implementation review, merge.
