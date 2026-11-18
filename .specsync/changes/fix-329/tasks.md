---
change: fix-329
artifact: tasks
---

# Tasks

- [x] Add a test that fails on main: a blocking bug added in a session with nothing linked does not stop it.
- [x] Join the active session on `bug add` whether or not `--observation` names its evidence; leave `bug update` and the no-session case as they are.
- [x] Update `docs/cli.md`; add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
