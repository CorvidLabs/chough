---
id: fix-263
state: archived
type: bug_fix
base_commit: be6c9f126e448bd33945a52261d55db2a9677d3d
---

# Fix 263

## Intent

fix 263

## Affected Canonical Specs

- `records`

## Acceptance Criteria

- On a case-sensitive file system, when knowledge/index.md already exists with other bytes beside the 0.5.x knowledge/INDEX.md, chough migrate --apply exits non-zero with refused:target-exists naming knowledge/index.md, and no file is written, removed or changed, so no receipt exists. On a case-insensitive file system, where the two names are one file, the apply still migrates as before.

## No-spec Rationale

REQ-records-009 already says the apply refuses, writing nothing more, when a file it would write already exists with other bytes, and removes the 0.5.x files only after every write is read back. The pre-write check skipped a target that differs only in case from a removed file, so on a case-sensitive file system a separate knowledge/index.md passed it. Only the check and its test change.
