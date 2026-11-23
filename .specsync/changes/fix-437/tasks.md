---
change: fix-437
artifact: tasks
---

# Tasks

- [ ] Write before_state/after_state as none when absent in the direct-write Timeline entry (src/session_md.rs).
- [ ] Change the entry text's state fallback from unknown to none (src/direct_writes.rs).
- [ ] Update/add tests for the absent-state entry.
- [ ] Add changelog.d/2026-11-23-fix-437.md fragment; run tests, fmt, lint, change check.