---
change: remove-the-history-events-a-command-published-when-its-save-is-refused-because-the-session-file-vanished-and-test-that
artifact: requirements
---

# Requirements

ADDED REQ-history-008 (see `deltas/history.md`): when a save is refused because the
resolved checkpoint vanished while the command ran, the events that command published are
removed first, so it leaves no orphan; other failed saves keep REQ-history-002's orphan.
REQ-history-002's wording is unchanged (tests/spec_split.rs pins it).
