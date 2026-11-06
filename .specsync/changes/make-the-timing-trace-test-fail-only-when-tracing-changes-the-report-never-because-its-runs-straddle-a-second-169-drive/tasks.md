---
change: make-the-timing-trace-test-fail-only-when-tracing-changes-the-report-never-because-its-runs-straddle-a-second-169-drive
artifact: tasks
---

# Tasks

- [x] Capture DRIVE-19 in hi/drive.md.
- [x] Make the test fail every time under a forced second straddle (`.stub-delay-<id>`), on Linux.
- [ ] Replace the retry loop with the exact clock-shift comparison, plus its strictness checks.
- [ ] Add a CHANGELOG line, and run verify on Linux.
