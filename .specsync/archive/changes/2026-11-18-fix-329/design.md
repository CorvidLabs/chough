---
change: fix-329
artifact: design
---

# Design

- `bugs::join_active_session` takes whether to join regardless of the linked evidence.
  - `bug add` joins the active session whenever one is active and readable.
  - `bug update` joins only when a linked evidence entry is the active session's, as before.
- With no session active, nothing joins, and the bug stays on no session's list. `chough next` on a later session names it under "Other blocking bugs … (do not block this session)", as it does now.
- The join is the same `checkpoint --bug` write, once, under the project lock `bug add` already holds.
