---
change: keep-one-session-per-objective-or-native-change-under-chough-sessions-with-append-only-transitions-an-active-pointer
artifact: research
---

# Research

- Every workflow field (stage, gates, consents, awaiting-human, native change, lists)
  lives in the one checkpoint. `read_session`, `save_checkpoint` and `migrate` in
  src/checkpoint.rs are the only readers and writers of its path, so resolving that
  path once moves every command to the active session.
- History events (REQ-history-002) are already immutable files with random IDs under
  a per-cycle directory, and the checkpoint holds only a cursor. A session that owns
  its own cycle therefore has append-only transitions that never collide with another
  session's.
- REQ-cli-007 says Chough SHALL NOT introduce a second writable checkpoint store, and
  REQ-history-002 forbids a second mutable checkpoint. In sessions mode the session
  files replace `.chough/session.json` and only the active one is written, so there is
  still exactly one writable checkpoint at a time. Inactive and archived sessions are
  only read. This reading is a review question for Leif.
