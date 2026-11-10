---
id: append-only-event-corrections-with-chough-correct-never-rewriting-history-167
state: implementing
type: feature
base_commit: 00c34d65d3eb9c56b8f24ed471fcc4e71ebbd90f
---

# Append-only event corrections with chough correct, never rewriting history (#167)

## Intent

Append-only event corrections with chough correct, never rewriting history (#167)

## Affected Canonical Specs

- `history`
- `reporting`
- `cli`

## Acceptance Criteria

- chough correct <event> --reason appends a correction event naming its EVT- target with the corrected statement in the active session's current cycle; the target's bytes are unchanged; unknown ids, blank or oversized statements and correcting a correction are refused with nothing written; next and status show the correction with its target; timing treats the kind as known with no stage segment.

## No-spec Rationale

Not applicable
