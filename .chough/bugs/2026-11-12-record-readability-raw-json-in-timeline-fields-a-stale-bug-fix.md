---
kind: bug
id: 2026-11-12-record-readability-raw-json-in-timeline-fields-a-stale-bug-fix
aliases: []
title: "Record readability: raw JSON in Timeline fields, a stale bug Fix line, an unlabelled trigger, unordered receipts, and empty receipt sections"
status: verified
classification: product-defect
scope: blocking
impact: Records read worse than the rest of 0.6.0
criteria: []
requirements: []
cases: []
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-relay-fields-are-labelled-lines
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-direct-write-fields-are-labelled-lines
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-story-receipts-in-time-order
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-relay-fields-are-labelled-lines-8fe7da
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-direct-write-fields-are-labelled-lines-7f8b92
  - 2026-11-12-release-0-6-0/2026-11-12-tc-history-story-receipts-in-time-order-4f9404
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-bug-trigger-and-fix
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-receipt-says-none
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-bug-trigger-and-fix-807baa
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-receipt-says-none-95c8f5
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:40Z
updated: 2026-11-12T17:47:44Z
---
# Record readability: raw JSON in Timeline fields, a stale bug Fix line, an unlabelled trigger, unordered receipts, and empty receipt sections

**What happened:** Timeline relayed_by and direct_write values are raw JSON; a verified bug keeps Fix: Not fixed yet with no flag to set it; --trigger becomes an unlabelled lead line under Steps; a story's receipts are not in time order and do not mark superseded runs; same-day reruns get random suffixes; the migration receipt prints empty Redactions, Kept with leaks and Not carried sections
Actual: As above

**Steps:** A full cycle and a migration

**Expected:** Plain, ordered, labelled records with none where a section is empty

**Why it matters:** Records read worse than the rest of 0.6.0

**Fix:** Verified: 2026-11-12-release-0-6-0/2026-11-12-tc-records-receipt-says-none-95c8f5 passed at 06e0a1b0335e7c75b623567e77a8a9d38e087433.

## Log

- 2026-11-12T16:21:40Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
- 2026-11-12T16:48:39Z · status: open -> investigating; scope: in-scope -> blocking; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-relay-fields-are-labelled-lines, 2026-11-12-release-0-6-0/2026-11-12-tc-history-direct-write-fields-are-labelled-lines, 2026-11-12-release-0-6-0/2026-11-12-tc-history-story-receipts-in-time-order. Reproduced (Timeline fields and receipt order): red at da98f439, TC-history-relay-fields-are-labelled-lines and TC-history-direct-write-fields-are-labelled-lines failed (relayed_by and direct_write written as JSON), TC-history-story-receipts-in-time-order failed (receipts in name order). The bug's Trigger and Fix and the receipt's none are with t38c. In scope (Leif, 16:26Z, relayed by t370) · by claude-code (hc t382)
- 2026-11-12T17:20:19Z · status: investigating -> fix-pending-verification; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-history-relay-fields-are-labelled-lines-8fe7da, 2026-11-12-release-0-6-0/2026-11-12-tc-history-direct-write-fields-are-labelled-lines-7f8b92, 2026-11-12-release-0-6-0/2026-11-12-tc-history-story-receipts-in-time-order-4f9404. Green at 56c3f4d3 for the Timeline fields and receipt order. Still to come from t38c: the bug's Trigger and Fix, and none in empty migration-receipt sections · by claude-code (hc t382)
- 2026-11-12T17:46:01Z · evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-bug-trigger-and-fix, 2026-11-12-release-0-6-0/2026-11-12-tc-records-receipt-says-none. Red at caa29f5e for t38c's two parts: TC-cli-bug-trigger-and-fix (no Trigger label, no --fix, Fix still Not fixed yet after verifying) and TC-records-receipt-says-none (an empty Redactions section had no none) · by claude-code (hc t382)
- 2026-11-12T17:47:44Z · status: fix-pending-verification -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-bug-trigger-and-fix-807baa, 2026-11-12-release-0-6-0/2026-11-12-tc-records-receipt-says-none-95c8f5. Green after t38c's fixes 47f0b905 and 474377c2 (cherry-picked as cc3c8a3c and ae50a9c5): a new bug's Steps start with Trigger:, bug update --fix sets and logs Fix, verifying with no Fix names the passing observation and its revision, and an empty receipt section says none; with the labelled Timeline fields and receipt order already green at 56c3f4d3, every part of this bug is fixed · by claude-code (hc t382)
