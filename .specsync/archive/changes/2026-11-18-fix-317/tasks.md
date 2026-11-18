---
change: fix-317
artifact: tasks
---

# Tasks

- [x] Add tests that fail on main: an update saved while `evidence add` waits for the writer lock is lost, and parallel decisions and evidence adds lose updates.
- [x] Take the writer lock before the evidence save reads `session.md`.
- [x] Check `bug add --observation` and `bug update --observation`: both already take the lock first, so add a guard test in a commit of its own and leave `src/bugs.rs` unchanged.
- [x] Add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
