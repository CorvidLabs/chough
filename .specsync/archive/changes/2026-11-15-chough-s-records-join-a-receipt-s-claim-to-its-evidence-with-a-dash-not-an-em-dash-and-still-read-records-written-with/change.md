---
id: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
state: archived
type: bug_fix
base_commit: cf2ff72c9f6f67cfa050cd92562dcc1b1b4f52c8
---

# Chough's records join a receipt's claim to its evidence with a dash, not an em dash, and still read records written with the old join (#278)

## Intent

Chough's records join a receipt's claim to its evidence with a dash, not an em dash, and still read records written with the old join (#278)

## Affected Canonical Specs

- `history`
- `records`
- `cli`

## Acceptance Criteria

- An archived or migrated story's What was verified line and a gate summary's Change line hold no em dash: each joins with ' - ',A journal post derived from a story archived with the earlier ' — ' join still gives the line's claim alone and copies no evidence ID

## No-spec Rationale

Not applicable
