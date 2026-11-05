---
change: lock-inbox-add-against-a-concurrent-archive-and-protect-observations-named-by-unfolded-inbox-entries-from-retention
artifact: tasks
---

# Tasks

- [x] Failing-first tests in tests/sessions.rs and tests/evidence.rs
- [x] inbox add takes the lock; every_checkpoint adds unfolded inbox observations
- [x] docs/cli.md and CHANGELOG.md
- [x] #99 review: failing-first test for an add killed while writing, and a test for the unreadable-entry refusal
- [x] #99 review: publish each entry whole (temporary file, then hard link), and let checkpoint saves remove inbox temporary files
