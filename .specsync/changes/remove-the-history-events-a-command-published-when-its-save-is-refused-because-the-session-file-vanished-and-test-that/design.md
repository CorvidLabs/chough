---
change: remove-the-history-events-a-command-published-when-its-save-is-refused-because-the-session-file-vanished-and-test-that
artifact: design
---

# Design

- src/history.rs: `append_using` records, in this process, each event it publishes (its
  path and the SHA-256 of its bytes) and each cycle directory it reserves.
  `discard_published(root)` removes those event files whose bytes are unchanged, then
  those cycle directories when empty, and returns how many events it removed.
- src/sessions.rs `checkpoint_path`: the two refusals that mean the resolved checkpoint
  vanished while the command ran (the pinned session file was archived or removed, or a
  legacy writer found the project adopted into sessions mode) first call
  `discard_published`, and say how many events were removed. Any other failed save keeps
  REQ-history-002's unselected orphan, as before.
