---
id: fix-314-folders
state: archived
type: bug_fix
base_commit: b2a227e3aa07fed5bf465dca8c70a8c35c4476ca
---

# Fix 314 folders

## Intent

fix 314 folders

## Affected Canonical Specs

- `records`

## Acceptance Criteria

- A 0.5.x project with a ._* folder holding only an empty folder under .chough/history/ previews with the same plan digest as without it, and after the apply that ._* folder and the folder inside it are both still there; the migration neither descends into nor removes a folder whose name starts with ._.

## No-spec Rationale

REQ-records-015 already says the migration neither reads, lists nor removes a file or folder whose name starts with ._; only the folder clean-up and its test change.
