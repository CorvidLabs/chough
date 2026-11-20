---
change: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
artifact: context
---

# Context

#406, item 2. `chough change ready` names as outside the declared paths every file a
commit on the branch's first-parent line since `base_commit` changed. After a rebase
onto main, or a squash merge of main, main's own commits or their content sit on that
line, so main's files are named. The tester measured it on a real branch rebased onto
main after its two open changes recorded their bases: about 700 findings per change
(690 and 665), every one a file outside the declared paths; 27 of the 28 first-parent
commits since each base were already on the default branch, and the recorded base was
no longer an ancestor of HEAD.

Also #406 (the 66-change run), item 4: a base commit a squash merge removed makes ready
print Git's raw `fatal: bad object` line.

Left out, for Leif to choose: the 66-change run's item 3, several open changes on one
branch, where one change's work is counted as another's.
