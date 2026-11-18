---
id: fix-267
state: archived
type: bug_fix
base_commit: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
---

# Fix 267

## Intent

fix 267

## Affected Canonical Specs

- `records`

## Acceptance Criteria

- A token form that directly follows a hyphen, such as x-ghp_<token> in a checkpoint question, a session key or a raw log name, refuses the write as refused:leak naming the field and the kind token, with every file unchanged; chough doctor reports the same text in a committed record as a leak finding. The same token after a space, colon or dot is still refused.

## No-spec Rationale

REQ-records-006 already refuses a GitHub, GitLab, Slack, AWS or model-provider token form in every value Chough writes; only the matcher and its tests change.
