---
change: keep-vault-copies-another-writer-changed-when-a-refused-trim-rolls-back-110
artifact: tasks
---

# Tasks

- [x] Add the failing test from the issue's scenario, plus a mode change and an unchanged copy.
- [x] Record each copy with the bytes and permissions written. On rollback, claim it atomically, compare, and remove it only if unchanged. Otherwise put it back without replacing anything, and name it in the refusal.
- [x] Run the evidence and reduce suites and the verify lane on Linux.
