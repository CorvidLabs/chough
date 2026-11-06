---
change: an-empty-change-directory-left-by-a-branch-switch-is-not-an-open-native-change-148
artifact: tasks
---

# Tasks

- [x] Add the failing test `an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change`.
- [x] Count a directory under `.specsync/changes/` as an open change only when it contains a file, at any depth. Use that rule in both the index's Active list and the trim's open-change checks.
- [x] Add a CHANGELOG line, and run verify on Linux.
