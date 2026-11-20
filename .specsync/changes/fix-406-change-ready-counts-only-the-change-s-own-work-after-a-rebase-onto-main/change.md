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

- After a rebase onto the default branch or a squash merge of it, chough change ready names no file only the default branch changed and still names a file outside the declared paths that the change changed; with HEAD on the default branch, its tip at HEAD, or origin/HEAD naming HEAD's own branch, such a file is still named; with a base commit the repository lacks, it says so, prints no fatal: line and counts from the merge base, and with no default branch either it names one finding that the changed files could not be listed, never reporting that nothing is missing.

## No-spec Rationale

Not applicable
