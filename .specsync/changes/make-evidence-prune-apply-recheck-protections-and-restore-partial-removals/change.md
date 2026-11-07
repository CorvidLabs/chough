---
id: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
state: implementing
type: bug_fix
base_commit: 455cb643cd91a9c804afdf5566e3889abf1bde49
---

# Make evidence prune --apply recheck protections and restore partial removals

## Intent

Make evidence prune --apply recheck protections and restore partial removals

## Affected Canonical Specs

- `history`
- `cli`

## Acceptance Criteria

- A protection appearing between removals stops the apply and restores every earlier removal byte-for-byte without overwriting a recreated file; a later artifact or observation removal failure also restores earlier removals and appends the existing restored ledger event; successful pruning keeps its current eligibility, authorization, ledger and JSON behavior, and existing history remains readable

## No-spec Rationale

Not applicable
