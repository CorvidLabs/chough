---
change: fix-266-prefs
artifact: testing
---

# Testing

`tests/git_merge.rs::preference_writes_refuse_while_chough_is_unmerged`, red on fix-266 before this change. With `.chough/README.md` unmerged in a real stopped merge:
- `pref set` and `pref unset` refuse as `refused:unmerged` with every file unchanged;
- `pref show` still runs.

After `git merge --abort`, `pref set` writes.
