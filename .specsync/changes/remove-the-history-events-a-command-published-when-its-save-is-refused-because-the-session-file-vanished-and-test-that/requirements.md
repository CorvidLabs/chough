---
change: remove-the-history-events-a-command-published-when-its-save-is-refused-because-the-session-file-vanished-and-test-that
artifact: requirements
---

# Requirements

MODIFIED REQ-history-002 (see `deltas/history.md`): when a save is refused because the
resolved checkpoint vanished while the command ran, the events that command published are
removed first, so it leaves no orphan.
