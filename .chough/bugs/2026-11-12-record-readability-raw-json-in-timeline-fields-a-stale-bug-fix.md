---
kind: bug
id: 2026-11-12-record-readability-raw-json-in-timeline-fields-a-stale-bug-fix
aliases: []
title: "Record readability: raw JSON in Timeline fields, a stale bug Fix line, an unlabelled trigger, unordered receipts, and empty receipt sections"
status: open
classification: product-defect
scope: in-scope
impact: Records read worse than the rest of 0.6.0
criteria: []
requirements: []
cases: []
evidence: []
receipts: []
environment: local-macos-arm64
revision: ee9cae4eeee05e65d7c25f66e3af4b6489636bdc
found_by: claude-code (hc t382) (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T16:21:40Z
updated: 2026-11-12T16:21:40Z
---
# Record readability: raw JSON in Timeline fields, a stale bug Fix line, an unlabelled trigger, unordered receipts, and empty receipt sections

**What happened:** Timeline relayed_by and direct_write values are raw JSON; a verified bug keeps Fix: Not fixed yet with no flag to set it; --trigger becomes an unlabelled lead line under Steps; a story's receipts are not in time order and do not mark superseded runs; same-day reruns get random suffixes; the migration receipt prints empty Redactions, Kept with leaks and Not carried sections
Actual: As above

**Steps:** A full cycle and a migration

**Expected:** Plain, ordered, labelled records with none where a section is empty

**Why it matters:** Records read worse than the rest of 0.6.0

**Fix:** Not fixed yet.

## Log

- 2026-11-12T16:21:40Z · Reported (t38j's fresh-eyes report on 0.6.0 at c208c8a8 (claude-code, hc t38j, a cold full cycle on a scratch Python CLI and a 0.5.0 migration, 2026-11-12), relayed by t370; recorded by t382, not yet reproduced) · by claude-code (hc t382)
