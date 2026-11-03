---
change: serialize-checkpoint-writers-with-an-inter-process-lock-and-remove-session-temps-left-by-killed-writers-issues-62-and
artifact: tasks
---

# Tasks

- [x] Failing-first tests for parallel writers, killed writers and orphan-only cleanup
- [x] fs4 dependency; `lock_checkpoint` in read_checkpoint, save_checkpoint and migrate
- [x] `remove_orphaned_session_temps` under the lock, with a stderr notice
- [x] Busy refusal unit test
- [x] Unlocked checkpoint reads for validate
- [x] docs/cli.md and CHANGELOG.md
- [x] Tests remove the checkpoint lock files they create (tests/common/mod.rs and tests/cli.rs Fixtures, the unit test)
