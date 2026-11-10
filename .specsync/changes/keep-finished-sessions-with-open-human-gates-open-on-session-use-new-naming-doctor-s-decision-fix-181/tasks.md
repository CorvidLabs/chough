---
change: keep-finished-sessions-with-open-human-gates-open-on-session-use-new-naming-doctor-s-decision-fix-181
artifact: tasks
---

# Tasks

- [x] RED: regression test archiving a gated finished session fails before the fix.
- [x] Fix: `archive_done` keeps such sessions open, named with the doctor recovery.
- [x] GREEN: regression passes; full `fledge run test` suite passes (593s, macOS arm64).

Remaining as plain text, not checkboxes: human scope approval (this amends REQ-history-006), then verification, review, archive and merge.
