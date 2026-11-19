---
change: fix-272
artifact: tasks
---

# Tasks

- [x] Add a test that fails on main: `journal add` waits while another process holds the writer lock.
- [x] Add a test that a storm of parallel `journal add` commands leaves a feed with every post.
- [x] Take the writer lock in `journal add` before it reads or writes.
- [x] Add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
