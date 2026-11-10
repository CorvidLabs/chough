---
id: append-only-corrections-for-recorded-transition-reasons-and-archived-session-next-action-167
state: draft
type: feature
base_commit: 3767a46878d44588c37237f088d5f94ce8d51b52
---

# Append-only corrections for recorded transition reasons and archived session next_action (#167)

## Intent

Append-only corrections for recorded transition reasons and archived session next_action (#167)

## Affected Canonical Specs

- `history`
- `reporting`

## Acceptance Criteria

- chough correct <event> --reason appends a correction event naming its EVT- target with the corrected statement in the active session's current cycle; the target's bytes are unchanged; unknown ids, blank or oversized statements and correcting a correction are refused with nothing written; next and status show the correction with its target; timing treats the kind as known with no stage segment.

## No-spec Rationale

Not applicable
