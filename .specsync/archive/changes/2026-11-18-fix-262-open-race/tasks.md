---
change: fix-262-open-race
artifact: tasks
---

# Tasks

- [x] Add `open_regular`: a non-blocking open with the type checked on the opened handle.
- [x] `read_bounded_record` and `session shelve` read through it.
- [x] Add a test that a FIFO present at the open is refused without blocking.
- [x] Add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
