---
id: fix-406-change-ready-counts-only-the-change-s-own-work-after-a-rebase-onto-main
state: draft
type: bug_fix
base_commit: 1fb67c29295cc2472285bbd03b6a10cf8c5f6c40
---

# Fix 406: change ready counts only the change's own work after a rebase onto main

## Intent

Fix 406: change ready counts only the change's own work after a rebase onto main

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- On a branch rebased onto the default branch, or one that squash-merged it, after the change recorded its base, chough change ready names no file whose content at HEAD is the default branch's, and still names a file outside the declared paths that the change's own commits changed; with a base commit the repository does not have, it says so, naming it, prints no fatal: line, and counts from HEAD's merge base with the default branch.

## No-spec Rationale

Not applicable
