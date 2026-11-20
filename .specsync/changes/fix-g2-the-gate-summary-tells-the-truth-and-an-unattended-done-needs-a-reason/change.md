---
id: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
state: draft
type: bug_fix
base_commit: 7bf110dcca70bdd3565007086aee40a286f40eff
---

# Fix G2: the gate summary tells the truth and an unattended done needs a reason

## Intent

Fix G2: the gate summary tells the truth and an unattended done needs a reason

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- The gate summary counts a passing observation with no case, shows uncommitted work as git diff <base> and counts a session with no base from the first commit; doctor's unfinished_archive fix names the journal post and finishes no archive while it is uncommitted; without SpecSync, stage advance says no human decision is recorded for plan or accept, and --to across them refuses as refused:no-human-decision unless --reason is given, which lands in the Timeline.

## No-spec Rationale

Not applicable
