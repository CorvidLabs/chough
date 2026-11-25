---
id: fix-438
state: archived
type: feature
base_commit: 2053788676035ddbe4658b6cc07b18d817c597e6
---

# Fix 438

## Intent

fix 438

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- One chough gate record reply naming several gates closes all of them or none: when any named gate's close would be refused (for example a later save over the session record bound), the whole record is refused with nothing written and no earlier gate left closed.

## No-spec Rationale

gate record already requires all-or-none in REQ-cli-061; the code is brought in line, no contract text changes
