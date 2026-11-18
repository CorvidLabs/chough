---
change: fix-266
artifact: testing
---

# Testing

`tests/git_merge.rs` builds a real merge conflict in a fixture: two branches change the same session record, then `git merge` stops with it unmerged.
- `a_merge_conflict_in_chough_warns_and_refuses_writes`: `next` and `status` exit 0 and name the merge and the path; `doctor` reports one `git_operation` finding; `checkpoint --decision`, `evidence add` and `bug add` refuse as `refused:unmerged` with every file unchanged. After `git merge --abort`, no warning or finding remains and a write succeeds.
- `a_merge_conflict_outside_chough_warns_without_refusing`: the conflict is in a product file; the reads warn and a write succeeds.

Both fail on main. Case `TC-cli-git-merge-in-progress-warns-and-refuses-writes` binds the first.
