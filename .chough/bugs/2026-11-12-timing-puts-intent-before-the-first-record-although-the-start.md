---
kind: bug
id: 2026-11-12-timing-puts-intent-before-the-first-record-although-the-start
aliases: []
title: timing puts intent before the first record although the start entry is a record, and repeats elapsed wall-clock
status: open
classification: product-defect
scope: in-scope
impact: The report misplaces the first stage (t38m F11)
criteria: []
requirements: [REQ-reporting-001]
cases: [TC-reporting-timing-starts-cycle-1-at-the-start-entry]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-reporting-timing-starts-cycle-1-at-the-start-entry
receipts: []
environment: local-macos-arm64
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T22:56:56Z
updated: 2026-11-12T22:56:56Z
---
# timing puts intent before the first record although the start entry is a record, and repeats elapsed wall-clock

**What happened:** chough timing lists intent under 'Before the first record', outside Cycle 1, in a session whose Timeline starts with a start entry, and prints '(elapsed wall-clock)' after every number
Actual: Intent before the first record; the label on every number

**Steps:** Trigger: chough init, move through a stage or two, then chough timing
1. chough init --objective <o>
2. chough stage advance
3. chough timing

**Expected:** Intent in Cycle 1 from the start entry; elapsed wall-clock said once

**Why it matters:** The report misplaces the first stage (t38m F11)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T22:56:56Z · Reported (t38m's third fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-reporting-timing-starts-cycle-1-at-the-start-entry · by claude-code (hc t382)
