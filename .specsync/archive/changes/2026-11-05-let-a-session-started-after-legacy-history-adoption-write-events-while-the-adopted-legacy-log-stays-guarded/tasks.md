---
change: let-a-session-started-after-legacy-history-adoption-write-events-while-the-adopted-legacy-log-stays-guarded
artifact: tasks
---

# Tasks

- [x] Add the failing test: a legacy project adopts history, adopts its checkpoint as a session, then starts another session, which must write events. The legacy log keeps its bytes, and a later change to the log still refuses writes.
- [x] Guard a cursor without its own anchor with the anchor that an adopting session recorded, and keep the legacy log out of the new session's own history.
- [x] Run the case and the verify lane on Linux.
