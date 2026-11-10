---
id: know-29-roll-up-copies-never-moves-tracked-session-records-165
state: verifying
type: bug_fix
base_commit: 5359f42550362eebac3eef28d4df34639ffeb9c2
---

# KNOW-29 roll-up copies never moves tracked session records (#165)

## Intent

KNOW-29 roll-up copies never moves tracked session records (#165)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- At the move into done with the librarian on, archived sessions beyond the five newest are copied byte for byte into the vault (hard link sharing bytes) and remain in .chough/sessions/archive/ unchanged; the report names them under copied. A vault attachment with identical bytes means nothing to do; different bytes leave both sides untouched and are reported. The legacy-anchor holder never rolls. Tracked session records never leave the tree, whether or not knowledge/ is committed.

## No-spec Rationale

Not applicable
