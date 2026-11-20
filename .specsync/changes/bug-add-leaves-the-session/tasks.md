---
change: bug-add-leaves-the-session
artifact: tasks
---

# Tasks

- [ ] Failing tests first (`tests/bugs.rs`), each red on main.
- [ ] #382 and #395: `bug add` records the session on the bug and never writes the session; `session_takes_a_bug` and `check_save` removed.
- [ ] The session's bugs in `checkpoint`, `context`, the story and the blocking check; the update join skips a bug filed in the session.
- [ ] Docs, changelog fragment, cases.

Human gates (not checkboxes): scope approval, implementation review, merge.
