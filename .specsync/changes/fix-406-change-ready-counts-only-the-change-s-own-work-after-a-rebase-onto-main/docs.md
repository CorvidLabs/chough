---
change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
artifact: docs
---

# Docs

`docs/cli.md`: the `change ready` section says which files count as the change's own
work after a rebase onto main or a squash merge of it, and what it says when the base
commit is gone. One changelog fragment in `changelog.d/`.
