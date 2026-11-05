---
change: keep-vault-attachment-paths-within-windows-limits-and-keep-file-modes-when-the-trim-moves-or-restores-files
artifact: design
---

# Design

- `attachments::attachment_folder(id)` names an archive's folder. It is the ID when the ID has at most 24 characters. Otherwise it is the first 16 characters, cut back to before the last hyphen among them, then `-` and 8 hex digits of the ID's SHA-256. It is at most 25 characters and stable. `attachment`, `attachments_of` and the archive note's links (src/library.rs) all use it, so moves, notes and lookups agree. Short IDs keep their readable folder.
- The worst case in this repository is 22 (`knowledge/attachments/`) + 25 + 1 + 161 (the longest path under `.chough/`) = 209 characters. `MAX_ATTACHMENT_PATH` is 220, which leaves 39 characters for the folder a clone sits in under Windows' 260-character limit. `summary_plan` takes the knowledge directory: a candidate with any planned move longer than that is reported protected with the reason, and nothing of it moves or is removed. Nothing is lost, and the next run sees the same.
- The apply reads each file's permissions before removing it, sets them on the vault copy after writing it, and sets them on a restored file after the create-new write. The `held` list carries them.
- `commit_aged` passes the same `user.name`, `user.email` and `commit.gpgsign` settings as the fixture's `git` helper.
