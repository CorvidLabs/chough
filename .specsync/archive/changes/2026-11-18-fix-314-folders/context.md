---
change: fix-314-folders
artifact: context
---

# Context

A follow-up to fix-314 on PR #321, from Leif's review thread [P3] "Exclude metadata directories from recursive cleanup". REQ-records-015 says the migration neither reads, lists nor removes a file or folder whose name starts with `._`. But `prune_empty_folders` in `src/migrate.rs` still descends into such a folder and removes it when it is empty. In a 0.5.x fixture, `.chough/history/._metadata-folder/empty-child` left the preview digest unchanged and was not listed, yet the apply deleted both folders. No file bytes were lost, but the ignored metadata subtree changed outside the preview.

Leif chose "Fix it in #321 first" (2026-11-18, relayed by hc t4c): a small follow-up change on #321's branch, approved by his P3 comment and that answer. fix-314 itself is already archived on the branch. No requirement text changes.
