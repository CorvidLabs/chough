---
change: lock-inbox-add-against-a-concurrent-archive-and-protect-observations-named-by-unfolded-inbox-entries-from-retention
artifact: design
---

# Design

- src/sessions.rs `inbox add`: take `lock_project` before checking the session is open,
  like `session new`, `use`, `adopt` and `fold`; `archive_done` runs under that lock, so
  the add either precedes the switch's unfolded-entry check (the session stays open) or
  follows the archive (the add refuses with nothing written).
- src/sessions.rs `every_checkpoint`: after the open and archived sessions, add one entry
  per session (open or archived) with unfolded `observation` inbox entries, labelled
  `inbox of session <key>`, whose checkpoint holds those observation IDs. Retention then
  protects them like the session's own. An unreadable entry is an error, as an
  unreadable session is.
