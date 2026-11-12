---
kind: bug
id: 2026-11-12-the-journal-post-s-derived-verified-line-copies-an-evidence-id
aliases: []
title: The journal post's derived Verified line copies an evidence ID into the knowledge vault
status: verified
classification: product-defect
scope: blocking
impact: The full suite fails, and the vault copies a record's ID; found by t38c's full-suite run
criteria: [KNOW-31]
requirements: [REQ-records-011]
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-delivery-kinds-kept-dry-run-not
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-delivery-kinds-kept-dry-run-not-5f607b
receipts: []
environment: local-macos-arm64
revision: e8bde99c60a7555434694e8ffadf1bb28fa25fda
found_by: claude-code (hc t382) (t38c's full-suite run at its helper branch (2026-11-12), traced by t382 to the derived journal body of 69b60a68)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T17:40:45Z
updated: 2026-11-12T17:41:51Z
---
# The journal post's derived Verified line copies an evidence ID into the knowledge vault

**What happened:** tests/reduce.rs delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not fails: dry-run: the receipt was copied into the vault
Actual: The derived body's Verified: line carries the receipt line's anchor ^<evidence id>; a regression from 69b60a68 (the case last passed on 2026-11-06 at 6f1117b6, not an ancestor of HEAD, so no regression baseline can be linked)

**Steps:** The move into done of a session without --summary whose newest What was verified entry is a receipt line ending in its evidence ID

**Expected:** The post quotes the run's claim only; no evidence ID appears under knowledge/ (REQ-cli-044: the vault links records and copies none)

**Why it matters:** The full suite fails, and the vault copies a record's ID; found by t38c's full-suite run

**Fix:** Not fixed yet.

## Log

- 2026-11-12T17:40:45Z · Reported (t38c's full-suite run at its helper branch (2026-11-12), traced by t382 to the derived journal body of 69b60a68); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-delivery-kinds-kept-dry-run-not · by claude-code (hc t382)
- 2026-11-12T17:41:51Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-delivery-kinds-kept-dry-run-not-5f607b. Green at ac0cafef: TC-cli-delivery-kinds-kept-dry-run-not passed (reduce 6/6), and TC-records-journal-post-derives-a-title-and-body still passed · by claude-code (hc t382)
