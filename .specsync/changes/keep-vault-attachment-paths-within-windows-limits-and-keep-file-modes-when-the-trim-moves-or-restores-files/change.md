---
id: keep-vault-attachment-paths-within-windows-limits-and-keep-file-modes-when-the-trim-moves-or-restores-files
state: approved
type: bug_fix
base_commit: 8a91c5c491321e1ddd5575e99412459c7bdec7f4
---

# Keep vault attachment paths within Windows limits and keep file modes when the trim moves or restores files

## Intent

Keep vault attachment paths within Windows limits and keep file modes when the trim moves or restores files

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Vault attachments of an archive whose ID has more than 24 characters go under a stable folder of at most 25 characters, and no attachment path passes 220 characters (a key item that would stays, protected with that reason), so the repository checks out on Windows. Moved and restored files keep their permissions. The new tests fail first, Windows CI checks the branch out, fledge lanes run verify passes on Linux, and the trim reruns on this repository at closeout.

## No-spec Rationale

Not applicable
