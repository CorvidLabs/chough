---
change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
artifact: testing
---

# Testing

`tests/ready.rs`, each failing on main first:

- a branch rebased onto main after the change recorded its base: main's files are not
  named, and the change's own file outside the paths is;
- a branch that squash-merged main: the same;
- a base commit the repository lacks: one plain line naming it, no `fatal:`, and the
  change's own file outside the paths still named.

Then every verify step, the case on macOS, and the tester's run on the real rebased
branch it measured (690 and 665 findings on main).
