---
change: fix-262
artifact: tasks
---

# Tasks

- [x] Add tests that fail on main: a FIFO in the bugs folder blocks no reader, and a session whose record is a FIFO is listed with the reason and the recovery.
- [x] The bounded record reader refuses a non-regular file before it opens it.
- [x] The evidence-side readers of the bugs and story folders skip non-regular entries, as the bug readers do.
- [x] A session folder whose `session.md` exists but is not a regular file counts as a session that cannot be read; `session shelve` does not read it, and its note says what the entry is.
- [x] Add the case definitions and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
