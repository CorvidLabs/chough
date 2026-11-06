---
change: an-empty-change-directory-left-by-a-branch-switch-is-not-an-open-native-change-148
artifact: context
---

# Context

Issue #148, filed by ch-c456 during the C5 closeout (#137, PR #146).

A branch switch removes a change's tracked files but leaves its empty directories, because Git does not track directories. Chough reads every directory under `.specsync/changes/` as an open change, in two places:
- the knowledge index's Active list (`library.rs`, `sorted_entries`);
- the trim's open-change checks (`evidence.rs`, `change_dirs`).

So the index written in that worktree differs from a fresh clone, and `library index --check` fails in CI. The trim may also refuse with a false "a native change is open".

The new test leaves `.specsync/changes/left-behind/deltas/` empty. It reproduces the failure: the index lists `left-behind` as Active.
