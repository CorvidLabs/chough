---
id: a-pull-request-that-already-has-its-stage-label-keeps-that-label
state: archived
type: bug_fix
base_commit: 727d3bfdeef47b5a5df912d542565f8ba321a377
---

# A pull request that already has its stage label keeps that label

## Intent

A pull request that already has its stage label keeps that label

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A pull request that already has chough-stage:build, and whose introduced session is build, is not edited: the script does not remove that label and does not add it. A different chough-stage label on that pull request is still removed. The two script copies stay byte-identical.

## No-spec Rationale

REQ-cli-070 already says the script applies the one main stage and removes every other chough-stage label. The script removes the matching label and adds it again. This change makes the script match that sentence. The requirement text stays as it is.
