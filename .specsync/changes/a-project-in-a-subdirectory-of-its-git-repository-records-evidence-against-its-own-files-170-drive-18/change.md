---
id: a-project-in-a-subdirectory-of-its-git-repository-records-evidence-against-its-own-files-170-drive-18
state: approved
type: bug_fix
base_commit: 167790489735b3e6a88ec21a20bb2530d4c96138
---

# A project in a subdirectory of its Git repository records evidence against its own files (170, DRIVE-18)

## Intent

A project in a subdirectory of its Git repository records evidence against its own files (170, DRIVE-18)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- DRIVE-18 (Leif, relayed by orc 2026-11-06, 'Capture both'; chough#170), in the new REQ-cli-056: in a project in a subdirectory of its Git repository, cases run, evidence add and status work from that directory. The snapshot covers the project's own files with the repository's HEAD. Edits outside the project change nothing, and edits inside make it dirty. The same project has the same content fingerprint at the root and in a subdirectory. The '--root must be the Git repository root' refusal is gone. The three new tests in tests/subdirectory.rs fail first; docs/cli.md (the line-5 limitation) and CHANGELOG say so; fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
