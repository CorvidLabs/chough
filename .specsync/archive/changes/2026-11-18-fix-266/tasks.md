---
change: fix-266
artifact: tasks
---

# Tasks

- [x] Add tests that fail on main: during a merge, `next`, `status` and `doctor` say nothing, and writes go into a conflicted `.chough/`.
- [x] Detect a merge or rebase and the unmerged `.chough/` paths; warn in `next`, `status` and `doctor`; refuse writes at the writer lock and before an evidence entry is written.
- [x] Document the `git_operation` finding and line in `docs/cli.md`; add the case definition and the changelog fragment.

Human gates (not checkboxes): scope approval, implementation review, merge.
