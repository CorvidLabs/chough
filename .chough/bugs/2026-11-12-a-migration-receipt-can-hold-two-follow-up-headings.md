---
kind: bug
id: 2026-11-12-a-migration-receipt-can-hold-two-follow-up-headings
aliases: []
title: A migration receipt can hold two Follow-up headings
status: open
classification: product-defect
scope: in-scope
impact: The receipt reads as two sections; a reader may miss one (t38k F12)
criteria: []
requirements: [REQ-records-009]
cases: [TC-cli-migration-receipt-has-one-follow-up-heading]
evidence: [2026-11-12-release-0-6-0/2026-11-12-tc-cli-migration-receipt-has-one-follow-up-heading]
receipts: []
environment: local-macos-arm64
revision: a0152789eec8d548d98dbd11f86b0818ad98f494
found_by: claude-code (hc t382) (t38k's second fresh-eyes round report (relayed by t370); fixed by t38c)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T20:25:59Z
updated: 2026-11-12T20:25:59Z
---
# A migration receipt can hold two Follow-up headings

**What happened:** The receipt chough migrate --apply writes has two '## Follow-up' headings when there are follow-ups of two kinds
Actual: Two '## Follow-up' headings

**Steps:** Trigger: Apply a migration whose plan has more than one kind of follow-up
1. Migrate a 0.5.x fixture with follow-ups
2. Read the receipt

**Expected:** One Follow-up heading holding every follow-up

**Why it matters:** The receipt reads as two sections; a reader may miss one (t38k F12)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T20:25:59Z · Reported (t38k's second fresh-eyes round report (relayed by t370); fixed by t38c); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-migration-receipt-has-one-follow-up-heading · by claude-code (hc t382)
