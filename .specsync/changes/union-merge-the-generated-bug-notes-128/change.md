---
id: union-merge-the-generated-bug-notes-128
state: approved
type: bug_fix
base_commit: 4c68cca7275b3ab8f49af94cc925bf547842757a
---

# Union-merge the generated bug notes (128)

## Intent

Union-merge the generated bug notes (128)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- git check-attr reports merge=union for knowledge/bugs/<id>.md while attachments, snapshots, briefs and the README stay non-union; the test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within REQ-cli-047 and DRIVE-12: the bug notes KNOW-22 generates were left out of the union-merge rules; no requirement text changes.
