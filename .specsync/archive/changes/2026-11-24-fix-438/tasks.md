---
change: fix-438
artifact: tasks
---

# Tasks

- [x] Add hidden --dry-run to checkpoint --close-gate: all checks, no write, no native step.
- [x] gate record: dry-run every named gate before closing any; refuse all on any refusal.
- [x] Regression test: second close over the session bound refuses with nothing written.
- [x] Add changelog.d/2026-11-23-fix-438.md fragment; run gate/checkpoint tests, fmt, lint, change check.
