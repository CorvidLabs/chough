---
id: an-empty-change-directory-left-by-a-branch-switch-is-not-an-open-native-change-148
state: draft
type: bug_fix
base_commit: 64aa77e01d2763395d8af7566e62743ede37cc32
---

# An empty change directory left by a branch switch is not an open native change (148)

## Intent

An empty change directory left by a branch switch is not an open native change (148)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A directory under .specsync/changes with no file at any depth is not an open change: the knowledge index does not list it and the trim does not refuse on it, while a directory with a change file still counts as open; the new test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Issue #148: Chough counted any directory under .specsync/changes as an open change, so an empty one left by a branch switch broke the index check and could refuse the trim; no requirement text changes.
