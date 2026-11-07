---
id: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
state: approved
type: bug_fix
base_commit: 1fc4c67b59f05097cab4cafa446aa58198f8ee56
---

# Correct prune capture authority and unsafe rollback for PR 197

## Intent

Correct prune capture authority and unsafe rollback for PR 197

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- An artifact changed between eligible-digest validation and byte capture is never treated as deletion authority: retain or refuse it and restore any earlier removals; a removed destination or ancestor recreated as a symlink keeps its identity and target untouched while original bytes survive in a named safe hold, all other held files are attempted, and restored events account for every ledgered set; existing eligibility, protections, authorization, bounds, successful output and ledger schemas and all historical archives remain unchanged.

## No-spec Rationale

Not applicable
