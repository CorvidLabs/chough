---
id: fix-260-a-new-record-is-written-whole-or-not-at-all
state: archived
type: bug_fix
base_commit: 056cbb8029f6535ff74f866a0523893916f682db
---

# Fix 260: a new record is written whole or not at all

## Intent

Fix 260: a new record is written whole or not at all

## Affected Canonical Specs

- `records`
- `cli`

## Acceptance Criteria

- A new record whose write fails or is cut off, by a full disk or a file-size limit, leaves no file at its name and no temporary file, the command exits non-zero naming the record, and bug list, status, next and library index --check work as before; an empty record file is named as empty; on exFAT, bug add and journal add still write whole records.

## No-spec Rationale

Not applicable
