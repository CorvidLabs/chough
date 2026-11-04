---
id: show-the-latest-failed-native-check-in-next-and-say-when-it-has-not-passed-for-this-tree-issue-86
state: archived
type: bug_fix
base_commit: 91a1fdd82a4e314e5037c6256f17199ef0d604f0
---

# Show the latest failed native check in next and say when it has not passed for this tree (issue 86)

## Intent

Show the latest failed native check in next and say when it has not passed for this tree (issue 86)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- After a failed native attempt next shows its time and commit and failing commands with exit codes and a later passing attempt replaces it. With the change approved or implementing next says the native check has not passed for this tree and that SpecSync keeps no reason for refusals and names specsync change check. A malformed attempts record is unknown and next makes no extra native read. Each new test fails on main 91a1fdd and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
