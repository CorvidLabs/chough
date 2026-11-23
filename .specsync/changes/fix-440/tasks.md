---
change: fix-440
artifact: tasks
---

# Tasks

- [ ] Fix the go-back hint in the `stage advance --to` not-ahead refusal (src/stage.rs) to name `--reason "<why>"`.
- [ ] Update the test asserting on that refusal.
- [ ] Add changelog.d/2026-11-23-fix-440.md fragment.
- [ ] Run the touched tests and `chough change check fix-440`.