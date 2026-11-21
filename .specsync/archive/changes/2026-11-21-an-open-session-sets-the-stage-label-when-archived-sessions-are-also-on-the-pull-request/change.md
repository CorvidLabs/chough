---
id: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
state: archived
type: feature
base_commit: 6bc60913b028761b20832a6906dc8e31d17e70df
---

# An open session sets the stage label when archived sessions are also on the pull request

## Intent

An open session sets the stage label when archived sessions are also on the pull request

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A pull request that introduces an archived session at done and an open session at intent is labeled chough-stage:intent, and the done label is removed. A pull request whose only introduced session is an archived story at done is labeled chough-stage:done. Two open sessions at different stages still get no stage label. The two script copies stay byte-identical.

## No-spec Rationale

Not applicable
