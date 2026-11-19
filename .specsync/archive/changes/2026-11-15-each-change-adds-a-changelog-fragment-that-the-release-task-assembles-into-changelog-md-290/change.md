---
id: each-change-adds-a-changelog-fragment-that-the-release-task-assembles-into-changelog-md-290
state: archived
type: operations
base_commit: 49b1a252ae47a9e7a5fd8fe257c993d29351a90a
---

# Each change adds a changelog fragment that the release task assembles into CHANGELOG.md (#290)

## Intent

Each change adds a changelog fragment that the release task assembles into CHANGELOG.md (#290)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A change adds its entry as changelog.d/<date>-<change-id>.md and CHANGELOG.md is not edited by the change

## No-spec Rationale

No requirement describes the changelog's layout; LIFE-16 is unchanged, and the change is repository tooling, its tests, the release doc and the workflow skill's fast path
