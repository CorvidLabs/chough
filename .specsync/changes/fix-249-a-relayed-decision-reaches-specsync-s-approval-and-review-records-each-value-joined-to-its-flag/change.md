---
id: fix-249-a-relayed-decision-reaches-specsync-s-approval-and-review-records-each-value-joined-to-its-flag
state: draft
type: bug_fix
base_commit: d3138bed5495be77e66d35502a0891b156ffed8e
---

# Fix #249: a relayed decision reaches SpecSync's approval and review records, each value joined to its flag

## Intent

Fix #249: a relayed decision reaches SpecSync's approval and review records, each value joined to its flag

## Affected Canonical Specs

- `cli`
- `records`

## Acceptance Criteria

- Closing a gate that drives approval or review with a human's relay, including chough change ship's review, runs SpecSync with the relay's words, place and recording agent as --words, --where and --recorded-by, each value joined to its flag so text starting with - is never read as a flag

## No-spec Rationale

Not applicable
