---
id: append-only-corrections-for-recorded-transition-reasons-and-archived-session-next-action-167
state: draft
type: feature
base_commit: 2d69a5397014204af4c933f5d639b87ce800a800
---

# Append-only corrections for recorded transition reasons and archived session next_action (#167)

## Intent

Append-only corrections for recorded transition reasons and archived session next_action (#167)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- chough correct <event> --reason appends a correction event naming its EVT- target with the corrected statement in the active session's current cycle; the target's bytes are unchanged; unknown ids, blank or oversized statements and correcting a correction are refused with nothing written; next and status show the correction with its target.

## No-spec Rationale

Not applicable
