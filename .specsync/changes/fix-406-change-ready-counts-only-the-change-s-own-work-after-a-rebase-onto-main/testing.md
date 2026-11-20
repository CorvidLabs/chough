---
change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
artifact: testing
---

# Testing

`tests/ready.rs`, each failing on main first:

- a branch rebased onto main after the change recorded its base: main's files are not
  named, and the change's own file outside the paths is;
- a branch that squash-merged main: the same;
- HEAD on main, main's tip at HEAD, and origin/HEAD naming HEAD's own branch: the
  change's file outside the paths is still named;
- a base commit the repository lacks: one plain line naming it, no `fatal:`, and the
  change's own file outside the paths still named; with no default branch either, one
  finding that the files could not be listed, and no "Nothing is missing."

The existing tests and cases bound to REQ-cli-067 work on a local `main` with no
remote, which this change counts from `base_commit` as before; none is made false.

Then every verify step, the case on macOS, and the tester's run on the real data, since
the risk here is hiding a true finding: on its 66-change set and the rebased branch it
measured (690 and 665 findings on main), the count before and after per change, and
that every finding dropped is a file only the default branch's commits changed, none
the change's own.
