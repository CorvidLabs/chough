---
kind: bug
id: 2026-11-12-gate-summaries-name-neither-the-intent-at-an-intent-gate-nor
aliases: []
title: Gate summaries name neither the intent at an intent gate nor the delta past the five-file cap
status: open
classification: product-defect
scope: in-scope
impact: The human approves without seeing what they approve (t38m F4, F5)
criteria: []
requirements: [REQ-cli-061]
cases:
  - TC-cli-intent-gate-summary-names-the-criteria-and-hi-files
  - TC-cli-scope-gate-summary-names-each-delta-first
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-intent-gate-summary-names-the-criteria-and-hi-files
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-scope-gate-summary-names-each-delta-first
receipts: []
environment: local-macos-arm64
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T22:56:46Z
updated: 2026-11-12T22:56:46Z
---
# Gate summaries name neither the intent at an intent gate nor the delta past the five-file cap

**What happened:** chough gate list at an intent gate prints Change: none recorded, What changed: unknown and Diff: unknown, nothing about the intent; at a scope gate the five-file cap left deltas/<module>.md, the contract change, unnamed
Actual: Unknowns at intent; the delta cut by the cap

**Steps:** Trigger: List gates at an intent gate, and at a scope gate whose change has more than five Markdown files
1. chough checkpoint --awaiting-human <intent gate>; chough gate list
2. At plan with a change of 13 files, chough gate list

**Expected:** The intent's criterion IDs and HI files at an intent gate; every delta named first with the requirement IDs it adds or modifies

**Why it matters:** The human approves without seeing what they approve (t38m F4, F5)

**Fix:** Not fixed yet.

## Log

- 2026-11-12T22:56:46Z · Reported (t38m's third fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-intent-gate-summary-names-the-criteria-and-hi-files, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-scope-gate-summary-names-each-delta-first · by claude-code (hc t382)
