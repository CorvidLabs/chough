---
change: fix-298
artifact: tasks
---

# Tasks

- [x] Add a test that fails on main: a ship whose history holds a change both open and archived runs SpecSync.
- [x] Check `base..HEAD` for such a commit before the ship runs anything, and refuse naming it and the recovery, marked interim until spec-sync#1082.
- [x] Add a docs line saying the check is interim, the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
