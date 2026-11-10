---
change: keep-finished-sessions-with-open-human-gates-open-on-session-use-new-naming-doctor-s-decision-fix-181
artifact: tasks
---

# Tasks

- [x] RED: regression test archiving gated finished sessions fails before the fix.
- [x] Fix: `archive_done` keeps every finished session with an open gate open, naming the doctor recovery when its change is finalized and the close-gate command otherwise.
- [x] GREEN: regression passes.
- [x] Document the open-gate kept-open variant in docs/cli.md and add a CHANGELOG bullet.
