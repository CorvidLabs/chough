---
id: append-only-corrections-for-recorded-transition-reasons-and-archived-session-next-action-167
state: draft
type: feature
base_commit: 4e7bd5cbba1175f17e69752f317def5a49ac001f
---

# Append-only corrections for recorded transition reasons and archived session next_action (#167)

## Intent

Append-only corrections for recorded transition reasons and archived session next_action (#167)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- An agent can append a correction that names a recorded history event or an archived session's next_action, carrying who recorded it, why, and the corrected statement, without rewriting the original event; history show, next, status and the archived session's readers surface the correction beside what it corrects; the original bytes and event digests are unchanged.

## No-spec Rationale

Not applicable
