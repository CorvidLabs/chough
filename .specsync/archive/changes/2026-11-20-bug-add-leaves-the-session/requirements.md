---
change: bug-add-leaves-the-session
artifact: requirements
---

# Requirements

- REQ-cli-012: `bug add` while a session is active records that session's ID as the new bug's `session` and never writes the session's record; a session's bugs are its checkpoint's list followed by the bugs filed in it, listed by `checkpoint` and `context`, named in the story and counted for blocking; a bug whose `session` names an archived or missing session is no open session's and raises no error; `bug update --observation` does not join a bug filed in the active session (#382, #395).
- REQ-cli-054: related also means the bug was filed in the session; `bug add` records the session on the bug.
- REQ-history-006: the story's `## Bugs` names the session's bugs, those its checkpoint holds and those filed in it.
