---
change: fix-272-uninitialised-root
artifact: tasks
---

# Tasks

- [x] Add tests that fail on main: in a project with no `.chough/`, `journal add` takes the writer lock, and storms of parallel adds leave every post in the feed.
- [x] `journal add` takes the writer lock whether or not `.chough/` exists.
- [x] Add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
