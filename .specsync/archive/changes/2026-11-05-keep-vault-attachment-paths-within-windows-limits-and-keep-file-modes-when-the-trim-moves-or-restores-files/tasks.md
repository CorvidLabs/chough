---
change: keep-vault-attachment-paths-within-windows-limits-and-keep-file-modes-when-the-trim-moves-or-restores-files
artifact: tasks
---

# Tasks

- [x] Add the failing tests: a long archive ID's short folder, the 220-character guard, the vault copy's mode and the restored file's mode.
- [x] Give `commit_aged` the fixture's Git identity.
- [x] Add `attachment_folder` and use it for moves, lookups and note links.
- [x] Protect a candidate whose attachment path would be too long.
- [x] Keep permissions on vault copies and restores.
- [x] Update docs/cli.md, docs/FRD.md and CHANGELOG.md.
- [x] Run the cases and the verify lane on Linux, then rerun the trim on this repository at closeout.
