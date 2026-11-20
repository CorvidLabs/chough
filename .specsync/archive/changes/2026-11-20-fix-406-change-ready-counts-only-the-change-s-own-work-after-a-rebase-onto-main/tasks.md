---
change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
artifact: tasks
---

# Tasks

- [x] Failing tests first in `tests/ready.rs`: a rebase onto main, a squash merge of
      main, and a missing base, each red on main.
- [x] The filter and the missing-base fallback in `src/ready.rs`.
- [x] `docs/cli.md`, the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
