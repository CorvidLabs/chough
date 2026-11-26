---
id: fix-437
state: archived
type: feature
base_commit: 35828a1df3d308677a38b138e7aebe0e7b6770ef
---

# Fix 437

## Intent

fix 437

## Affected Canonical Specs

- `history`
- `records`
- `cli`

## Acceptance Criteria

- When a direct lifecycle write records a state of none (state.json absent), its Timeline entry shows before_state: none and after_state: none rather than omitting the lines, and the entry's text says state none, not unknown.

## No-spec Rationale

The Timeline writer and entry text are brought in line with REQ-history-013, which already requires none when state.json is absent; no contract text changes
