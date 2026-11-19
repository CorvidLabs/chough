---
id: fix-250
state: archived
type: bug_fix
base_commit: e47b4cfaabe2e68fd02246be01fb7e8e256fe00b
---

# Fix 250

## Intent

fix 250

## Affected Canonical Specs

- `history`
- `cli`
- `records`

## Acceptance Criteria

- After chough doctor --fix --reason withdraws a stale gate, that gate's gate-closed Timeline entry in the archived story holds the reason as its withdrawn field, in the person's words; doctor's JSON gives decision_reason on each decision finding it fixed and on no other finding; withdrawing still records no human decision.

## No-spec Rationale

Not applicable
